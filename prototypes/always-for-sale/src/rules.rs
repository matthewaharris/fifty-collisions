//! Rules core: a 6x6 Los Alamos-style board where every non-king piece carries a
//! posted price. Each ply the side to move makes exactly one action: a chess move,
//! a buyout of an attacked enemy piece at its posted price, or a reprice of up to
//! `reprice_limit` of its own pieces. Tax, rent and bankruptcy settle after each
//! full round. See RULES.md for the full text.

use serde::{Deserialize, Serialize};

pub const N: usize = 6;
pub const SQUARES: usize = N * N;
pub const MIN_PRICE: u8 = 1;
pub const MAX_PRICE: u8 = 12;
/// c3, d3, c4, d4: the rent-paying centre.
pub const CENTRE: [u8; 4] = [14, 15, 20, 21];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn other(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn idx(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum Kind {
    Pawn,
    Knight,
    Rook,
    Queen,
    King,
}

impl Kind {
    /// Conventional material value in coins, also the rote price.
    pub fn rote_price(self) -> u8 {
        match self {
            Kind::Pawn => 1,
            Kind::Knight => 3,
            Kind::Rook => 5,
            Kind::Queen => 9,
            Kind::King => 0,
        }
    }
    pub fn letter(self) -> char {
        match self {
            Kind::Pawn => 'p',
            Kind::Knight => 'n',
            Kind::Rook => 'r',
            Kind::Queen => 'q',
            Kind::King => 'k',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
    /// Posted price in whole coins (0 for kings, which are never for sale).
    pub price: u8,
    /// Ply on which this piece was last bought; it can't be bought on the next ply.
    pub bought_ply: i32,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Params {
    /// Tax per round as a fraction of the sum of a player's posted prices.
    pub tax_rate: f64,
    /// Coins per round for each piece standing on a centre square.
    pub rent: f64,
    pub start_treasury: f64,
    /// Treasury above this pays `prog_rate` of the excess each round.
    pub prog_threshold: f64,
    pub prog_rate: f64,
    /// From this round on, the tax rate rises by `late_slope` per round.
    pub late_start: u32,
    pub late_slope: f64,
    pub reprice_limit: u8,
    pub max_plies: u32,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            tax_rate: 0.08,
            rent: 1.0,
            start_treasury: 30.0,
            prog_threshold: 60.0,
            prog_rate: 0.1,
            late_start: 30,
            late_slope: 0.005,
            reprice_limit: 2,
            max_plies: 200,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Action {
    Move { from: u8, to: u8, promo: Option<Kind> },
    Buy { sq: u8 },
    /// Up to two (square, new price) pairs; `n` says how many are used.
    Reprice { n: u8, sqs: [u8; 2], prices: [u8; 2] },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Reason {
    Checkmate,
    Bankruptcy,
    MutualBankruptcy,
    Stalemate,
    Insufficient,
    PlyCap,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub winner: Option<Color>,
    pub reason: Reason,
}

/// What an action did, for logs, receipts and telemetry.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Effect {
    pub captured: Option<Kind>,
    pub bought: Option<Kind>,
    pub paid: u8,
    pub repriced: u8,
}

/// The board as a fixed array (no heap, cheap to copy), serialised as a list.
mod board_serde {
    use super::{Piece, SQUARES};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(b: &[Option<Piece>; SQUARES], s: S) -> Result<S::Ok, S::Error> {
        b.as_slice().serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[Option<Piece>; SQUARES], D::Error> {
        let v: Vec<Option<Piece>> = Vec::deserialize(d)?;
        v.try_into().map_err(|_| serde::de::Error::custom("board must have 36 squares"))
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct State {
    #[serde(with = "board_serde")]
    pub board: [Option<Piece>; SQUARES],
    pub side: Color,
    pub ply: u32,
    pub treasury: [f64; 2],
    pub params: Params,
    pub outcome: Option<Outcome>,
}

pub fn sq(file: usize, rank: usize) -> u8 {
    (rank * N + file) as u8
}
pub fn file_of(s: u8) -> i32 {
    (s as usize % N) as i32
}
pub fn rank_of(s: u8) -> i32 {
    (s as usize / N) as i32
}
pub fn name(s: u8) -> String {
    format!("{}{}", (b'a' + file_of(s) as u8) as char, rank_of(s) + 1)
}

const KNIGHT: [(i32, i32); 8] = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];
const KING: [(i32, i32); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
const ROOK_DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn on_board(f: i32, r: i32) -> bool {
    (0..N as i32).contains(&f) && (0..N as i32).contains(&r)
}

impl State {
    /// Los Alamos back rank (R N Q K N R) on 6x6, pawns on ranks 2 and 5.
    /// Prices start at the rote value; `set_prices` lets a policy post its own.
    pub fn new(params: Params) -> State {
        let mut board = [None; SQUARES];
        let back = [Kind::Rook, Kind::Knight, Kind::Queen, Kind::King, Kind::Knight, Kind::Rook];
        for f in 0..N {
            for (color, br, pr) in [(Color::White, 0, 1), (Color::Black, 5, 4)] {
                let k = back[f];
                board[sq(f, br) as usize] = Some(Piece { color, kind: k, price: k.rote_price(), bought_ply: -10 });
                board[sq(f, pr) as usize] =
                    Some(Piece { color, kind: Kind::Pawn, price: Kind::Pawn.rote_price(), bought_ply: -10 });
            }
        }
        State {
            board,
            side: Color::White,
            ply: 0,
            treasury: [params.start_treasury; 2],
            params,
            outcome: None,
        }
    }

    pub fn at(&self, s: u8) -> Option<Piece> {
        self.board[s as usize]
    }

    pub fn round(&self) -> u32 {
        self.ply / 2
    }

    pub fn tax_rate_now(&self) -> f64 {
        let r = self.round();
        let late = if r >= self.params.late_start { (r - self.params.late_start + 1) as f64 } else { 0.0 };
        self.params.tax_rate + self.params.late_slope * late
    }

    pub fn king_sq(&self, c: Color) -> Option<u8> {
        (0..SQUARES as u8).find(|&s| matches!(self.at(s), Some(p) if p.color == c && p.kind == Kind::King))
    }

    /// Squares the piece on `s` attacks (captures it could make), ignoring pins.
    pub fn attacks_from(&self, s: u8, out: &mut Vec<u8>) {
        out.clear();
        if let Some(p) = self.at(s) {
            self.attacks_as(s, p.kind, p.color, out);
        }
    }

    /// Squares a `kind` of colour `color` standing on `s` would attack.
    pub fn attacks_as(&self, s: u8, kind: Kind, color: Color, out: &mut Vec<u8>) {
        out.clear();
        let p = Piece { color, kind, price: 0, bought_ply: 0 };
        let (f, r) = (file_of(s), rank_of(s));
        match p.kind {
            Kind::Pawn => {
                let dr = if p.color == Color::White { 1 } else { -1 };
                for df in [-1, 1] {
                    if on_board(f + df, r + dr) {
                        out.push(sq((f + df) as usize, (r + dr) as usize));
                    }
                }
            }
            Kind::Knight | Kind::King => {
                let deltas = if p.kind == Kind::Knight { &KNIGHT } else { &KING };
                for &(df, dr) in deltas {
                    if on_board(f + df, r + dr) {
                        out.push(sq((f + df) as usize, (r + dr) as usize));
                    }
                }
            }
            Kind::Rook | Kind::Queen => {
                let diag: &[(i32, i32)] = if p.kind == Kind::Queen { &KING } else { &ROOK_DIRS };
                for &(df, dr) in diag {
                    let (mut x, mut y) = (f + df, r + dr);
                    while on_board(x, y) {
                        let t = sq(x as usize, y as usize);
                        out.push(t);
                        if self.at(t).is_some() {
                            break;
                        }
                        x += df;
                        y += dr;
                    }
                }
            }
        }
    }

    /// Whether `by` attacks `target`, by looking outward from the target.
    pub fn is_attacked(&self, target: u8, by: Color) -> bool {
        let (f, r) = (file_of(target), rank_of(target));
        let is = |x: i32, y: i32, kinds: &[Kind]| {
            on_board(x, y) && matches!(self.at(sq(x as usize, y as usize)), Some(p) if p.color == by && kinds.contains(&p.kind))
        };
        // A white pawn attacks upward, so it sits one rank below its target.
        let pr = if by == Color::White { r - 1 } else { r + 1 };
        if is(f - 1, pr, &[Kind::Pawn]) || is(f + 1, pr, &[Kind::Pawn]) {
            return true;
        }
        if KNIGHT.iter().any(|&(df, dr)| is(f + df, r + dr, &[Kind::Knight])) {
            return true;
        }
        if KING.iter().any(|&(df, dr)| is(f + df, r + dr, &[Kind::King])) {
            return true;
        }
        for &(df, dr) in &KING {
            let sliders: &[Kind] = if df != 0 && dr != 0 { &[Kind::Queen] } else { &[Kind::Queen, Kind::Rook] };
            let (mut x, mut y) = (f + df, r + dr);
            while on_board(x, y) {
                if let Some(p) = self.at(sq(x as usize, y as usize)) {
                    if p.color == by && sliders.contains(&p.kind) {
                        return true;
                    }
                    break;
                }
                x += df;
                y += dr;
            }
        }
        false
    }

    /// For each square, whether `by` attacks it.
    pub fn attack_map(&self, by: Color) -> [bool; SQUARES] {
        let mut m = [false; SQUARES];
        let mut buf = Vec::with_capacity(16);
        for s in 0..SQUARES as u8 {
            if matches!(self.at(s), Some(p) if p.color == by) {
                self.attacks_from(s, &mut buf);
                for &t in &buf {
                    m[t as usize] = true;
                }
            }
        }
        m
    }

    pub fn in_check(&self, c: Color) -> bool {
        match self.king_sq(c) {
            Some(k) => self.is_attacked(k, c.other()),
            None => true,
        }
    }

    fn pseudo_moves(&self, out: &mut Vec<Action>) {
        let me = self.side;
        let mut buf = Vec::with_capacity(16);
        for s in 0..SQUARES as u8 {
            let p = match self.at(s) {
                Some(p) if p.color == me => p,
                _ => continue,
            };
            if p.kind == Kind::Pawn {
                let (f, r) = (file_of(s), rank_of(s));
                let dr = if me == Color::White { 1 } else { -1 };
                let last = if me == Color::White { N as i32 - 1 } else { 0 };
                let push = |to: u8, out: &mut Vec<Action>| {
                    if rank_of(to) == last {
                        for k in [Kind::Queen, Kind::Rook, Kind::Knight] {
                            out.push(Action::Move { from: s, to, promo: Some(k) });
                        }
                    } else {
                        out.push(Action::Move { from: s, to, promo: None });
                    }
                };
                if on_board(f, r + dr) {
                    let t = sq(f as usize, (r + dr) as usize);
                    if self.at(t).is_none() {
                        push(t, out);
                    }
                }
                for df in [-1, 1] {
                    if on_board(f + df, r + dr) {
                        let t = sq((f + df) as usize, (r + dr) as usize);
                        if matches!(self.at(t), Some(q) if q.color != me) {
                            push(t, out);
                        }
                    }
                }
            } else {
                self.attacks_from(s, &mut buf);
                for &t in &buf {
                    match self.at(t) {
                        Some(q) if q.color == me => {}
                        _ => out.push(Action::Move { from: s, to: t, promo: None }),
                    }
                }
            }
        }
    }

    /// Legal chess moves for the side to move.
    pub fn legal_moves(&self) -> Vec<Action> {
        let mut v = Vec::with_capacity(40);
        self.pseudo_moves(&mut v);
        v.retain(|a| self.leaves_king_safe(a));
        v
    }

    /// Enemy pieces the side to move may buy: attacked by one of its pieces,
    /// not a king, affordable, and not bought on the previous ply.
    pub fn legal_buys(&self) -> Vec<Action> {
        let me = self.side;
        let map = self.attack_map(me);
        let mut v = Vec::new();
        for s in 0..SQUARES as u8 {
            if let Some(p) = self.at(s) {
                if p.color != me
                    && p.kind != Kind::King
                    && map[s as usize]
                    && (p.price as f64) <= self.treasury[me.idx()]
                    && p.bought_ply != self.ply as i32 - 1
                {
                    let a = Action::Buy { sq: s };
                    if self.leaves_king_safe(&a) {
                        v.push(a);
                    }
                }
            }
        }
        v
    }

    /// Whether any legal move or buy exists (stops at the first one).
    pub fn has_move_or_buy(&self) -> bool {
        let mut v = Vec::with_capacity(40);
        self.pseudo_moves(&mut v);
        v.iter().any(|a| self.leaves_king_safe(a)) || !self.legal_buys().is_empty()
    }

    pub fn is_legal(&self, a: &Action) -> bool {
        if self.outcome.is_some() {
            return false;
        }
        match *a {
            Action::Move { .. } => self.legal_moves().contains(a),
            Action::Buy { .. } => self.legal_buys().contains(a),
            Action::Reprice { n, sqs, prices } => {
                if n == 0 || n > self.params.reprice_limit.min(2) || self.in_check(self.side) {
                    return false;
                }
                if n == 2 && sqs[0] == sqs[1] {
                    return false;
                }
                (0..n as usize).all(|i| {
                    matches!(self.at(sqs[i]), Some(p) if p.color == self.side && p.kind != Kind::King
                        && (MIN_PRICE..=MAX_PRICE).contains(&prices[i]) && prices[i] != p.price)
                })
            }
        }
    }

    fn leaves_king_safe(&self, a: &Action) -> bool {
        let mut t = *self;
        t.apply_board(a);
        !t.in_check(self.side)
    }

    /// Board and money changes of an action, without ending the ply.
    fn apply_board(&mut self, a: &Action) -> Effect {
        let me = self.side;
        let mut e = Effect::default();
        match *a {
            Action::Move { from, to, promo } => {
                let mut p = self.board[from as usize].take().expect("move from empty square");
                e.captured = self.board[to as usize].map(|q| q.kind);
                if let Some(k) = promo {
                    p.kind = k;
                    p.price = k.rote_price();
                }
                self.board[to as usize] = Some(p);
            }
            Action::Buy { sq } => {
                let p = self.board[sq as usize].as_mut().expect("buy on empty square");
                let price = p.price;
                p.color = me;
                p.bought_ply = self.ply as i32;
                self.treasury[me.idx()] -= price as f64;
                self.treasury[me.other().idx()] += price as f64;
                e.bought = Some(p.kind);
                e.paid = price;
            }
            Action::Reprice { n, sqs, prices } => {
                for i in 0..n as usize {
                    if let Some(p) = self.board[sqs[i] as usize].as_mut() {
                        p.price = prices[i];
                    }
                }
                e.repriced = n;
            }
        }
        e
    }

    pub fn posted_total(&self, c: Color) -> u32 {
        self.board.iter().flatten().filter(|p| p.color == c).map(|p| p.price as u32).sum()
    }

    pub fn centre_count(&self, c: Color) -> u32 {
        CENTRE.iter().filter(|&&s| matches!(self.at(s), Some(p) if p.color == c)).count() as u32
    }

    /// Net change to `c`'s treasury at the next round's settlement.
    pub fn settlement(&self, c: Color) -> f64 {
        let p = &self.params;
        let t = self.treasury[c.idx()];
        let tax = self.tax_rate_now() * self.posted_total(c) as f64;
        let prog = if t > p.prog_threshold { p.prog_rate * (t - p.prog_threshold) } else { 0.0 };
        p.rent * self.centre_count(c) as f64 - tax - prog
    }

    /// Apply a legal action and end the ply (settlement, game end checks).
    /// The caller is responsible for legality; bots only pass generated actions.
    pub fn apply(&mut self, a: &Action) -> Effect {
        let e = self.apply_board(a);
        let mover = self.side;
        self.ply += 1;
        if mover == Color::Black {
            let dw = self.settlement(Color::White);
            let db = self.settlement(Color::Black);
            self.treasury[0] += dw;
            self.treasury[1] += db;
            let (bw, bb) = (self.treasury[0] < 0.0, self.treasury[1] < 0.0);
            if bw && bb {
                self.outcome = Some(Outcome { winner: None, reason: Reason::MutualBankruptcy });
            } else if bw {
                self.outcome = Some(Outcome { winner: Some(Color::Black), reason: Reason::Bankruptcy });
            } else if bb {
                self.outcome = Some(Outcome { winner: Some(Color::White), reason: Reason::Bankruptcy });
            }
        }
        self.side = mover.other();
        if self.outcome.is_none() {
            self.outcome = self.terminal();
        }
        e
    }

    fn terminal(&self) -> Option<Outcome> {
        let me = self.side;
        let non_king = |c: Color| self.board.iter().flatten().any(|p| p.color == c && p.kind != Kind::King);
        if !non_king(Color::White) && !non_king(Color::Black) {
            return Some(Outcome { winner: None, reason: Reason::Insufficient });
        }
        if !self.has_move_or_buy() {
            if self.in_check(me) {
                return Some(Outcome { winner: Some(me.other()), reason: Reason::Checkmate });
            }
            if !non_king(me) {
                return Some(Outcome { winner: None, reason: Reason::Stalemate });
            }
        }
        if self.ply >= self.params.max_plies {
            return Some(Outcome { winner: None, reason: Reason::PlyCap });
        }
        None
    }

    /// Plain-text board for debugging: uppercase White, price after each piece.
    pub fn ascii(&self) -> String {
        let mut s = String::new();
        for r in (0..N).rev() {
            s.push_str(&format!("{} ", r + 1));
            for f in 0..N {
                match self.at(sq(f, r)) {
                    Some(p) => {
                        let c = p.kind.letter();
                        let c = if p.color == Color::White { c.to_ascii_uppercase() } else { c };
                        if p.kind == Kind::King {
                            s.push_str(&format!(" {}  ", c));
                        } else {
                            s.push_str(&format!(" {}{:<2}", c, p.price));
                        }
                    }
                    None => s.push_str(" .  "),
                }
            }
            s.push('\n');
        }
        s.push_str("   a   b   c   d   e   f\n");
        s.push_str(&format!(
            "ply {} | W {:.1} coins | B {:.1} coins | tax {:.3}\n",
            self.ply,
            self.treasury[0],
            self.treasury[1],
            self.tax_rate_now()
        ));
        s
    }
}
