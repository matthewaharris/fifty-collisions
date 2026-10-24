//! Headless self-play CLI, the harness contract from plans/poc-07.
//!
//!   afs-sim run --cells cells.jsonl --out games.jsonl [--threads 4]
//!   afs-sim game [--seed 1] [--white full] [--black rote] [--params '{...}'] [--coin-value 100]
//!   afs-sim perft --depth 4
//!
//! A cell line is {"cell_id", "params", "coin_value", "matchup", "seed_start", "pairs"}.
//! matchup is "self", "vs_no_buy" or "vs_rote". Each pair plays the same seed twice
//! with sides swapped. One JSON line is written per game.

use afs::bot::BotCfg;
use afs::rules::*;
use afs::sim;
use serde::Deserialize;
use std::io::{BufRead, BufWriter, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};

#[derive(Deserialize, Clone)]
struct Cell {
    cell_id: String,
    params: Params,
    coin_value: f64,
    matchup: String,
    seed_start: u64,
    pairs: u64,
}

fn arg(args: &[String], key: &str) -> Option<String> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1).cloned())
}

fn bot_named(name: &str, cv: f64) -> BotCfg {
    match name {
        "full" | "honest" => BotCfg::full(cv),
        "no_buy" => BotCfg::no_buy(cv),
        "rote" => BotCfg::rote(cv),
        "monopolist" => BotCfg::monopolist(cv),
        other => panic!("unknown bot {other}"),
    }
}

fn opponent(matchup: &str, cv: f64) -> BotCfg {
    match matchup {
        "self" => BotCfg::full(cv),
        "vs_no_buy" => BotCfg::no_buy(cv),
        "vs_rote" => BotCfg::rote(cv),
        "vs_monopolist" => BotCfg::monopolist(cv),
        other => panic!("unknown matchup {other}"),
    }
}

fn run(args: &[String]) {
    let cells_path = arg(args, "--cells").expect("--cells");
    let out_path = arg(args, "--out").expect("--out");
    let threads: usize = arg(args, "--threads").map(|s| s.parse().unwrap()).unwrap_or(4);
    let f = std::fs::File::open(&cells_path).expect("open cells");
    let cells: Vec<Cell> = std::io::BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(&l).expect("bad cell line"))
        .collect();
    // One job = (cell index, pair index, full plays black?)
    let mut jobs = Vec::new();
    for (ci, c) in cells.iter().enumerate() {
        for p in 0..c.pairs {
            jobs.push((ci, p, false));
            jobs.push((ci, p, true));
        }
    }
    let total = jobs.len();
    let jobs = Arc::new(jobs);
    let cells = Arc::new(cells);
    let next = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel::<String>();
    let start = std::time::Instant::now();
    let mut handles = Vec::new();
    for _ in 0..threads {
        let (jobs, cells, next, tx) = (jobs.clone(), cells.clone(), next.clone(), tx.clone());
        handles.push(std::thread::spawn(move || loop {
            let j = next.fetch_add(1, Ordering::Relaxed);
            if j >= jobs.len() {
                break;
            }
            let (ci, p, full_black) = jobs[j];
            let c = &cells[ci];
            let full = BotCfg::full(c.coin_value);
            let opp = opponent(&c.matchup, c.coin_value);
            let seed = c.seed_start + p;
            let (w, b) = if full_black { (&opp, &full) } else { (&full, &opp) };
            let rec = sim::play(c.params, w, b, seed, false);
            let full_color = if full_black { Color::Black } else { Color::White };
            let score = match rec.winner {
                None => 0.5,
                Some(x) if x == full_color => 1.0,
                Some(_) => 0.0,
            };
            let mut v = serde_json::to_value(&rec).unwrap();
            let o = v.as_object_mut().unwrap();
            o.insert("cell_id".into(), c.cell_id.clone().into());
            o.insert("matchup".into(), c.matchup.clone().into());
            o.insert("full_color".into(), serde_json::to_value(full_color).unwrap());
            o.insert("full_score".into(), score.into());
            tx.send(v.to_string()).unwrap();
        }));
    }
    drop(tx);
    let mut out = BufWriter::new(std::fs::File::create(&out_path).expect("create out"));
    let mut n = 0usize;
    for line in rx {
        writeln!(out, "{line}").unwrap();
        n += 1;
        if n % 500 == 0 || n == total {
            eprintln!("{n}/{total} games, {:.0}s", start.elapsed().as_secs_f64());
        }
    }
    for h in handles {
        h.join().unwrap();
    }
}

fn game(args: &[String]) {
    let seed: u64 = arg(args, "--seed").map(|s| s.parse().unwrap()).unwrap_or(1);
    let cv: f64 = arg(args, "--coin-value").map(|s| s.parse().unwrap()).unwrap_or(100.0);
    let params: Params = arg(args, "--params").map(|s| serde_json::from_str(&s).unwrap()).unwrap_or_default();
    let w = bot_named(&arg(args, "--white").unwrap_or("full".into()), cv);
    let b = bot_named(&arg(args, "--black").unwrap_or("full".into()), cv);
    let t = std::time::Instant::now();
    let rec = sim::play(params, &w, &b, seed, true);
    let mut st = State::new(params);
    afs::bot::post_setup_prices(&mut st, Color::White, &w);
    afs::bot::post_setup_prices(&mut st, Color::Black, &b);
    println!("{}", st.ascii());
    for e in rec.events.as_ref().unwrap() {
        let desc = match e.action {
            Action::Move { from, to, promo } => {
                format!("{}{}{}", name(from), name(to), promo.map(|k| format!("={}", k.letter())).unwrap_or_default())
            }
            Action::Buy { sq } => format!("BUY {} for {}", name(sq), e.effect.paid),
            Action::Reprice { n, sqs, prices } => {
                (0..n as usize).map(|i| format!("{}->{}", name(sqs[i]), prices[i])).collect::<Vec<_>>().join(" ")
            }
        };
        println!("{:>3} {:?}: {:<20} W {:>6.1}  B {:>6.1}", e.ply, e.side, desc, e.treasury[0], e.treasury[1]);
        st.apply(&e.action);
    }
    println!("\n{}", st.ascii());
    println!(
        "result {:?} by {:?} after {} plies | buyouts {} | reprices W{} B{} | {} nodes in {:.2}s",
        rec.winner,
        rec.reason,
        rec.plies,
        rec.buyouts.len(),
        rec.reprices[0],
        rec.reprices[1],
        rec.nodes,
        t.elapsed().as_secs_f64()
    );
}

fn perft(st: &State, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = st.legal_moves();
    if depth == 1 {
        return moves.len() as u64;
    }
    moves
        .iter()
        .map(|a| {
            let mut c = st.clone();
            c.apply(a);
            perft(&c, depth - 1)
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => run(&args),
        Some("game") => game(&args),
        Some("perft") => {
            // Pure chess move generation (no buys, reprices or economy), for checking against
            // a Los Alamos reference. Generous treasury and no tax keep the economy out of it.
            let d: u32 = arg(&args, "--depth").map(|s| s.parse().unwrap()).unwrap_or(4);
            let params = Params { tax_rate: 0.0, rent: 0.0, late_slope: 0.0, prog_rate: 0.0, max_plies: 10_000, ..Params::default() };
            let st = State::new(params);
            for i in 1..=d {
                println!("perft {i}: {}", perft(&st, i));
            }
        }
        _ => eprintln!("usage: afs-sim run|game|perft (see source header)"),
    }
}
