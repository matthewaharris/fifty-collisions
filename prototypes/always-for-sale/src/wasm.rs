//! Browser build: one JSON-in, JSON-out entry point over the same rules core
//! and bots, so the web page plays exactly the game the sweeps tested.
//!
//! JS writes a UTF-8 command into memory from `alloc`, calls `call(ptr, len)`,
//! and reads the reply from the returned (ptr << 32 | len).

use crate::bot::{self, BotCfg};
use crate::rules::*;
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::RefCell;

#[derive(Serialize, Clone)]
struct Receipt {
    ply: u32,
    buyer: Color,
    kind: Kind,
    sq: String,
    price: u8,
    /// What the piece was worth to the seller (flip swing in coins) when it sold.
    value: f64,
}

struct Game {
    st: State,
    human: Color,
    cpu: BotCfg,
    rng: ChaCha8Rng,
    log: Vec<Value>,
    receipts: Vec<Receipt>,
    counts: [[u32; 3]; 2],
}

thread_local! {
    static GAME: RefCell<Option<Game>> = RefCell::new(None);
}

#[no_mangle]
pub extern "C" fn alloc(n: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(n);
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

#[no_mangle]
pub extern "C" fn dealloc(p: *mut u8, n: usize) {
    unsafe { drop(Vec::from_raw_parts(p, 0, n)) }
}

#[no_mangle]
pub extern "C" fn call(p: *const u8, n: usize) -> u64 {
    let input = unsafe { std::slice::from_raw_parts(p, n) };
    let reply = match serde_json::from_slice::<Value>(input) {
        Ok(cmd) => dispatch(&cmd).unwrap_or_else(|e| json!({ "error": e })),
        Err(e) => json!({ "error": e.to_string() }),
    };
    let mut bytes = reply.to_string().into_bytes();
    bytes.shrink_to_fit();
    let (ptr, len) = (bytes.as_mut_ptr(), bytes.len());
    std::mem::forget(bytes);
    ((ptr as u64) << 32) | len as u64
}

#[derive(Deserialize)]
struct NewGame {
    params: Params,
    coin_value: f64,
    human: Color,
    bot: String,
    seed: u64,
}

fn dispatch(cmd: &Value) -> Result<Value, String> {
    let name = cmd["cmd"].as_str().ok_or("missing cmd")?;
    if name == "new" {
        let ng: NewGame = serde_json::from_value(cmd.clone()).map_err(|e| e.to_string())?;
        let cpu = match ng.bot.as_str() {
            "monopolist" => BotCfg::monopolist(ng.coin_value),
            "rote" => BotCfg::rote(ng.coin_value),
            _ => BotCfg::full(ng.coin_value),
        };
        let mut st = State::new(ng.params);
        // The human starts from the bot's hedged quotes and can change them before move 1.
        bot::post_setup_prices(&mut st, ng.human, &BotCfg::full(ng.coin_value));
        bot::post_setup_prices(&mut st, ng.human.other(), &cpu);
        let g = Game {
            st,
            human: ng.human,
            cpu,
            rng: ChaCha8Rng::seed_from_u64(ng.seed),
            log: vec![],
            receipts: vec![],
            counts: [[0; 3]; 2],
        };
        GAME.with(|c| *c.borrow_mut() = Some(g));
        return Ok(view());
    }
    GAME.with(|c| {
        let mut b = c.borrow_mut();
        let g = b.as_mut().ok_or("no game")?;
        match name {
            "state" => {}
            "setup" => {
                // Free repricing before the first ply: [[sq, price], ...]
                if g.counts[g.human.idx()].iter().sum::<u32>() != 0 {
                    return Err("opening quotes can only be set before your first action".into());
                }
                for pair in cmd["prices"].as_array().ok_or("prices")? {
                    let s = pair[0].as_u64().ok_or("sq")? as u8;
                    let pr = (pair[1].as_u64().ok_or("price")? as u8).clamp(MIN_PRICE, MAX_PRICE);
                    if let Some(p) = g.st.board[s as usize].as_mut() {
                        if p.color == g.human && p.kind != Kind::King {
                            p.price = pr;
                        }
                    }
                }
            }
            "act" => {
                let a: Action = serde_json::from_value(cmd["action"].clone()).map_err(|e| e.to_string())?;
                if g.st.side != g.human || !g.st.is_legal(&a) {
                    return Err("illegal action".into());
                }
                play(g, a);
            }
            "bot" => {
                if g.st.outcome.is_none() && g.st.side != g.human {
                    let mut nodes = 0;
                    let a = bot::choose(&g.st, &g.cpu, &mut g.rng, &mut nodes).ok_or("bot has no action")?;
                    play(g, a);
                }
            }
            _ => return Err(format!("unknown cmd {name}")),
        }
        Ok(())
    })?;
    Ok(view())
}

fn play(g: &mut Game, a: Action) {
    let side = g.st.side;
    let mut buf = Vec::with_capacity(16);
    if let Action::Buy { sq } = a {
        let p = g.st.at(sq).unwrap();
        let cv = g.cpu.coin_value;
        g.receipts.push(Receipt {
            ply: g.st.ply,
            buyer: side,
            kind: p.kind,
            sq: name(sq),
            price: p.price,
            value: (bot::flip_swing(&g.st, sq, &mut buf) / cv * 10.0).round() / 10.0,
        });
    }
    let k = match a {
        Action::Move { .. } => 0,
        Action::Buy { .. } => 1,
        Action::Reprice { .. } => 2,
    };
    g.counts[side.idx()][k] += 1;
    let ply = g.st.ply;
    let before = g.st;
    let e = g.st.apply(&a);
    g.log.push(json!({ "ply": ply, "side": side, "text": describe(&before, &a, &e) }));
}

fn describe(st: &State, a: &Action, e: &Effect) -> String {
    let piece = |s: u8| match st.at(s).map(|p| p.kind) {
        Some(Kind::Pawn) => "pawn",
        Some(Kind::Knight) => "knight",
        Some(Kind::Rook) => "rook",
        Some(Kind::Queen) => "queen",
        Some(Kind::King) => "king",
        None => "?",
    };
    match *a {
        Action::Move { from, to, promo } => {
            let mut t = format!("{} {}-{}", piece(from), name(from), name(to));
            if e.captured.is_some() {
                t.push_str(" takes");
            }
            if let Some(k) = promo {
                t.push_str(&format!(" ={}", k.letter().to_ascii_uppercase()));
            }
            t
        }
        Action::Buy { sq } => format!("buys the {} on {} for {}", piece(sq), name(sq), e.paid),
        Action::Reprice { n, sqs, prices } => {
            let parts: Vec<String> =
                (0..n as usize).map(|i| format!("{} {} to {}", piece(sqs[i]), name(sqs[i]), prices[i])).collect();
            format!("reprices {}", parts.join(", "))
        }
    }
}

fn view() -> Value {
    GAME.with(|c| {
        let b = c.borrow();
        let g = b.as_ref().unwrap();
        let st = &g.st;
        let maps = [st.attack_map(Color::White), st.attack_map(Color::Black)];
        let squares: Vec<Value> = (0..SQUARES as u8)
            .map(|s| match st.at(s) {
                Some(p) => json!({
                    "color": p.color, "kind": p.kind, "price": p.price,
                    "attacked": maps[p.color.other().idx()][s as usize],
                    "fresh": p.bought_ply == st.ply as i32 - 1,
                }),
                None => Value::Null,
            })
            .collect();
        let human_turn = st.outcome.is_none() && st.side == g.human;
        let (moves, buys) = if human_turn { (st.legal_moves(), st.legal_buys()) } else { (vec![], vec![]) };
        let hi = g.human.idx();
        let total: u32 = g.counts[hi].iter().sum();
        json!({
            "squares": squares,
            "side": st.side,
            "human": g.human,
            "ply": st.ply,
            "round": st.round(),
            "treasury": st.treasury,
            "settlement": [st.settlement(Color::White), st.settlement(Color::Black)],
            "posted": [st.posted_total(Color::White), st.posted_total(Color::Black)],
            "tax_rate": st.tax_rate_now(),
            "params": st.params,
            "in_check": st.in_check(st.side),
            "moves": moves,
            "buys": buys,
            "outcome": st.outcome,
            "log": g.log,
            "receipts": if st.outcome.is_some() { serde_json::to_value(&g.receipts).unwrap() } else { Value::Null },
            "human_counts": { "moves": g.counts[hi][0], "buys": g.counts[hi][1], "reprices": g.counts[hi][2], "total": total },
        })
    })
}
