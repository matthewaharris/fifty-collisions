//! Bots: a static evaluator with a flip-swing term, shallow alpha-beta over
//! moves and buyouts, greedy buys (the search takes a buy when the flip swing is
//! worth more than the price) and repricing of the stalest quotes.

use crate::rules::*;
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};

pub const MATE: f64 = 100_000.0;
/// Rounds of future tax and rent the evaluator weighs.
const HORIZON: f64 = 6.0;
/// Weight on the latent buyout risk of unattacked, under-priced pieces.
const LATENT: f64 = 0.15;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BotCfg {
    pub name: String,
    /// May buy enemy pieces.
    pub buys: bool,
    /// May reprice its own pieces during play.
    pub reprices: bool,
    /// Posts conventional material values (P1 N3 R5 Q9) at setup instead of flip swing.
    pub rote_setup: bool,
    /// Price = flip swing x this (Honest 1.0, Monopolist 3.0).
    pub price_mult: f64,
    /// Centipawns per coin: how the bot converts board value to money.
    pub coin_value: f64,
    pub depth: u8,
    /// Uniform root noise in centipawns, so self-play games differ.
    pub noise: f64,
}

impl BotCfg {
    pub fn full(coin_value: f64) -> BotCfg {
        BotCfg {
            name: "full".into(),
            buys: true,
            reprices: true,
            rote_setup: false,
            price_mult: 1.0,
            coin_value,
            depth: 2,
            noise: 10.0,
        }
    }
    pub fn no_buy(coin_value: f64) -> BotCfg {
        BotCfg { name: "no_buy".into(), buys: false, ..BotCfg::full(coin_value) }
    }
    pub fn rote(coin_value: f64) -> BotCfg {
        BotCfg { name: "rote".into(), reprices: false, rote_setup: true, ..BotCfg::full(coin_value) }
    }
    pub fn monopolist(coin_value: f64) -> BotCfg {
        BotCfg { name: "monopolist".into(), price_mult: 3.0, ..BotCfg::full(coin_value) }
    }
}

pub fn material(k: Kind) -> f64 {
    match k {
        Kind::Pawn => 100.0,
        Kind::Knight => 300.0,
        Kind::Rook => 500.0,
        Kind::Queen => 900.0,
        Kind::King => 0.0,
    }
}

fn pst(p: &Piece, s: u8) -> f64 {
    let (f, r) = (file_of(s) as f64, rank_of(s) as f64);
    let centre_dist = (f - 2.5).abs() + (r - 2.5).abs();
    match p.kind {
        Kind::Pawn => {
            let adv = if p.color == Color::White { r - 1.0 } else { 4.0 - r };
            12.0 * adv
        }
        Kind::Knight | Kind::Queen => 10.0 * (3.0 - centre_dist),
        Kind::Rook => 0.0,
        Kind::King => -6.0 * (3.0 - centre_dist),
    }
}

/// How much the owner of the piece on `s` would lose, in centipawns, if it
/// flipped colour where it stands: twice its value (one side loses it, the other
/// gains it) plus what it would immediately threaten in its old camp.
pub fn flip_swing(st: &State, s: u8, buf: &mut Vec<u8>) -> f64 {
    let p = match st.at(s) {
        Some(p) if p.kind != Kind::King => p,
        _ => return 0.0,
    };
    let mut swing = 2.0 * (material(p.kind) + pst(&p, s));
    st.attacks_as(s, p.kind, p.color.other(), buf);
    for &t in buf.iter() {
        if let Some(q) = st.at(t) {
            if q.color == p.color {
                swing += if q.kind == Kind::King { 150.0 } else { 0.1 * material(q.kind) };
            }
        }
    }
    swing
}

/// Share of its flip swing a bot posts on a piece nobody attacks: lower quotes
/// save tax, and the bot reprices to full value once the piece is attacked.
pub const HEDGE: f64 = 0.4;

/// The price a bot with this config would post for the piece on `s`:
/// full flip swing (times `price_mult`) when attacked, a hedged share when not.
pub fn ideal_price(st: &State, s: u8, cfg: &BotCfg, buf: &mut Vec<u8>) -> u8 {
    let p = st.at(s).expect("ideal_price of empty square");
    if cfg.rote_setup && !cfg.reprices {
        return p.kind.rote_price();
    }
    let attacked = st.is_attacked(s, p.color.other());
    let share = if attacked { 1.0 } else { HEDGE };
    let coins = flip_swing(st, s, buf) / cfg.coin_value * cfg.price_mult * share;
    coins.round().clamp(MIN_PRICE as f64, MAX_PRICE as f64) as u8
}

/// Set every piece of colour `c` to the bot's setup price.
pub fn post_setup_prices(st: &mut State, c: Color, cfg: &BotCfg) {
    let mut buf = Vec::with_capacity(16);
    for s in 0..SQUARES as u8 {
        if matches!(st.at(s), Some(p) if p.color == c && p.kind != Kind::King) {
            let price = if cfg.rote_setup { st.at(s).unwrap().kind.rote_price() } else { ideal_price(st, s, cfg, &mut buf) };
            st.board[s as usize].as_mut().unwrap().price = price;
        }
    }
}

/// Best gain (centipawns, never negative) `buyer` could get from one buyout now.
fn best_buy_gain(st: &State, buyer: Color, map: &[bool; SQUARES], cv: f64, buf: &mut Vec<u8>) -> f64 {
    let mut best: f64 = 0.0;
    for s in 0..SQUARES as u8 {
        if let Some(p) = st.at(s) {
            if p.color != buyer
                && p.kind != Kind::King
                && map[s as usize]
                && (p.price as f64) <= st.treasury[buyer.idx()]
                && p.bought_ply != st.ply as i32 - 1
            {
                best = best.max(flip_swing(st, s, buf) - p.price as f64 * cv);
            }
        }
    }
    best
}

/// Static evaluation from `me`'s point of view, in centipawns.
pub fn evaluate(st: &State, me: Color, cv: f64) -> f64 {
    let mut buf = Vec::with_capacity(16);
    let mut score = [0.0f64; 2];
    for s in 0..SQUARES as u8 {
        if let Some(p) = st.at(s) {
            score[p.color.idx()] += material(p.kind) + pst(&p, s);
        }
    }
    let maps = [st.attack_map(Color::White), st.attack_map(Color::Black)];
    let tax = st.tax_rate_now();
    for c in [Color::White, Color::Black] {
        let i = c.idx();
        let t = st.treasury[i];
        score[i] += t * cv;
        score[i] -= st.posted_total(c) as f64 * tax * HORIZON * cv;
        score[i] += st.centre_count(c) as f64 * st.params.rent * HORIZON * cv;
        if t + st.settlement(c) < 0.0 {
            score[i] -= 2000.0;
        }
        let gain = best_buy_gain(st, c, &maps[i], cv, &mut buf);
        score[i] += if c == st.side { 0.6 * gain } else { 0.15 * gain };
    }
    // Latent risk: an unattacked piece quoted below its swing can be attacked
    // and bought later, faster than the owner can reprice everything.
    for s in 0..SQUARES as u8 {
        if let Some(p) = st.at(s) {
            if p.kind != Kind::King && !maps[p.color.other().idx()][s as usize] {
                let gap = flip_swing(st, s, &mut buf) - p.price as f64 * cv;
                if gap > 0.0 {
                    score[p.color.idx()] -= LATENT * gap;
                }
            }
        }
    }
    score[me.idx()] - score[me.other().idx()]
}

fn terminal_score(o: &Outcome, stm: Color, ply: u32) -> f64 {
    match o.winner {
        None => 0.0,
        Some(w) if w == stm => MATE - ply as f64,
        Some(_) => -MATE + ply as f64,
    }
}

fn order_key(st: &State, a: &Action) -> i32 {
    match *a {
        Action::Buy { sq } => 2000 + material(st.at(sq).unwrap().kind) as i32,
        Action::Move { to, promo, .. } => {
            let cap = st.at(to).map(|p| material(p.kind) as i32).unwrap_or(0);
            cap + if promo.is_some() { 800 } else { 0 }
        }
        Action::Reprice { .. } => -1,
    }
}

fn gen(st: &State, buys: bool) -> Vec<Action> {
    let mut v = st.legal_moves();
    if buys {
        v.extend(st.legal_buys());
    }
    v.sort_by_key(|a| -order_key(st, a));
    v
}

pub struct Searcher {
    pub nodes: u64,
}

impl Searcher {
    fn qsearch(&mut self, st: &State, mut alpha: f64, beta: f64, cv: f64, qdepth: u8) -> f64 {
        self.nodes += 1;
        if let Some(o) = &st.outcome {
            return terminal_score(o, st.side, st.ply);
        }
        let stand = evaluate(st, st.side, cv);
        if stand >= beta || qdepth == 0 {
            return stand;
        }
        alpha = alpha.max(stand);
        let caps: Vec<Action> = st
            .legal_moves()
            .into_iter()
            .filter(|a| matches!(a, Action::Move { to, .. } if st.at(*to).is_some()))
            .collect();
        for a in caps {
            let mut c = *st;
            c.apply(&a);
            let v = -self.qsearch(&c, -beta, -alpha, cv, qdepth - 1);
            if v >= beta {
                return v;
            }
            alpha = alpha.max(v);
        }
        alpha
    }

    /// Negamax over moves and buyouts. Opponent nodes always consider buys.
    fn search(&mut self, st: &State, depth: u8, mut alpha: f64, beta: f64, cv: f64) -> f64 {
        self.nodes += 1;
        if let Some(o) = &st.outcome {
            return terminal_score(o, st.side, st.ply);
        }
        if depth == 0 {
            return self.qsearch(st, alpha, beta, cv, 3);
        }
        let acts = gen(st, true);
        if acts.is_empty() {
            // Not mated (terminal() would have said so): the side can still reprice.
            return evaluate(st, st.side, cv);
        }
        let mut best = -f64::INFINITY;
        for a in acts {
            let mut c = *st;
            c.apply(&a);
            let v = -self.search(&c, depth - 1, -beta, -alpha, cv);
            if v > best {
                best = v;
            }
            alpha = alpha.max(v);
            if alpha >= beta {
                break;
            }
        }
        best
    }
}

/// Reprice candidates: the stalest one or two quotes reset to the bot's price.
/// Under-priced pieces under attack are the most urgent.
pub fn reprice_candidates(st: &State, cfg: &BotCfg) -> Vec<Action> {
    let me = st.side;
    let mut buf = Vec::with_capacity(16);
    let enemy = st.attack_map(me.other());
    let mut stale: Vec<(f64, u8, u8)> = Vec::new();
    for s in 0..SQUARES as u8 {
        if let Some(p) = st.at(s) {
            if p.color == me && p.kind != Kind::King {
                let ideal = ideal_price(st, s, cfg, &mut buf);
                let d = ideal as f64 - p.price as f64;
                if d == 0.0 {
                    continue;
                }
                let urgency = if d > 0.0 && enemy[s as usize] { 3.0 * d } else { d.abs() };
                if urgency >= 2.0 {
                    stale.push((urgency, s, ideal));
                }
            }
        }
    }
    stale.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut out = Vec::new();
    if let Some(&(_, s0, p0)) = stale.first() {
        out.push(Action::Reprice { n: 1, sqs: [s0, 0], prices: [p0, 0] });
        if st.params.reprice_limit >= 2 {
            if let Some(&(_, s1, p1)) = stale.get(1) {
                out.push(Action::Reprice { n: 2, sqs: [s0, s1], prices: [p0, p1] });
            }
        }
    }
    out.retain(|a| st.is_legal(a));
    out
}

/// Choose an action for the side to move. Returns None only if the game is over.
pub fn choose(st: &State, cfg: &BotCfg, rng: &mut ChaCha8Rng, nodes: &mut u64) -> Option<Action> {
    if st.outcome.is_some() {
        return None;
    }
    let mut cands = gen(st, cfg.buys);
    if cfg.reprices && !st.in_check(st.side) {
        cands.extend(reprice_candidates(st, cfg));
    }
    if cands.is_empty() && !cfg.buys {
        // A no-buy bot whose only legal actions are buys must buy (it can't pass).
        cands = st.legal_buys();
    }
    if cands.is_empty() {
        // Only a reprice can be legal here (no moves, no buys, not in check).
        let me = st.side;
        let s = (0..SQUARES as u8).find(|&s| matches!(st.at(s), Some(p) if p.color == me && p.kind != Kind::King))?;
        let p = st.at(s).unwrap().price;
        let np = if p < MAX_PRICE { p + 1 } else { p - 1 };
        return Some(Action::Reprice { n: 1, sqs: [s, 0], prices: [np, 0] });
    }
    let mut srch = Searcher { nodes: 0 };
    let mut best = (-f64::INFINITY, cands[0]);
    for a in cands {
        let mut c = *st;
        c.apply(&a);
        // Anything that can't beat the current best even with maximum noise can fail low.
        let floor = best.0 - 2.0 * cfg.noise;
        let mut v = -srch.search(&c, cfg.depth.saturating_sub(1), -f64::INFINITY, -floor, cfg.coin_value);
        if cfg.noise > 0.0 {
            let u = (rng.next_u32() as f64) / (u32::MAX as f64);
            v += (u * 2.0 - 1.0) * cfg.noise;
        }
        if v > best.0 {
            best = (v, a);
        }
    }
    *nodes += srch.nodes;
    Some(best.1)
}
