//! Play one game between two bot configs and summarise it for the harness.

use crate::bot::{self, BotCfg};
use crate::rules::*;
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Buyout {
    pub ply: u32,
    pub buyer: Color,
    pub kind: Kind,
    pub price: u8,
    /// The seller's flip swing at the moment of sale, in coins at the buyer's scale.
    pub swing_coins: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub ply: u32,
    pub side: Color,
    pub action: Action,
    pub effect: Effect,
    pub treasury: [f64; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRecord {
    pub seed: u64,
    pub white: String,
    pub black: String,
    pub winner: Option<Color>,
    pub reason: Reason,
    pub plies: u32,
    pub buyouts: Vec<Buyout>,
    /// Actions by kind per side: [white, black].
    pub moves: [u32; 2],
    pub buys: [u32; 2],
    pub reprices: [u32; 2],
    pub captures: [u32; 2],
    pub final_treasury: [f64; 2],
    pub nodes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
}

/// Plies of uniformly random legal moves at the start, so seeds give different games.
pub const RANDOM_OPENING_PLIES: u32 = 4;

pub fn play(params: Params, white: &BotCfg, black: &BotCfg, seed: u64, log: bool) -> GameRecord {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut st = State::new(params);
    bot::post_setup_prices(&mut st, Color::White, white);
    bot::post_setup_prices(&mut st, Color::Black, black);
    let mut rec = GameRecord {
        seed,
        white: white.name.clone(),
        black: black.name.clone(),
        winner: None,
        reason: Reason::PlyCap,
        plies: 0,
        buyouts: vec![],
        moves: [0; 2],
        buys: [0; 2],
        reprices: [0; 2],
        captures: [0; 2],
        final_treasury: [0.0; 2],
        nodes: 0,
        events: if log { Some(vec![]) } else { None },
    };
    let mut buf = Vec::with_capacity(16);
    while st.outcome.is_none() {
        let side = st.side;
        let cfg = if side == Color::White { white } else { black };
        let action = if st.ply < RANDOM_OPENING_PLIES {
            let mv = st.legal_moves();
            mv[(rng.next_u32() as usize) % mv.len()]
        } else {
            bot::choose(&st, cfg, &mut rng, &mut rec.nodes).expect("no action in a live game")
        };
        let i = side.idx();
        if let Action::Buy { sq } = action {
            let p = st.at(sq).unwrap();
            rec.buyouts.push(Buyout {
                ply: st.ply,
                buyer: side,
                kind: p.kind,
                price: p.price,
                swing_coins: bot::flip_swing(&st, sq, &mut buf) / cfg.coin_value,
            });
        }
        let ply = st.ply;
        let effect = st.apply(&action);
        match action {
            Action::Move { .. } => rec.moves[i] += 1,
            Action::Buy { .. } => rec.buys[i] += 1,
            Action::Reprice { .. } => rec.reprices[i] += 1,
        }
        if effect.captured.is_some() {
            rec.captures[i] += 1;
        }
        if let Some(ev) = rec.events.as_mut() {
            ev.push(Event { ply, side, action, effect, treasury: st.treasury });
        }
    }
    let o = st.outcome.unwrap();
    rec.winner = o.winner;
    rec.reason = o.reason;
    rec.plies = st.ply;
    rec.final_treasury = st.treasury;
    rec
}
