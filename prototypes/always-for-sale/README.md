# Always For Sale: prototype

Code for [POC 7](../../plans/poc-07-always-for-sale.md): a 6×6 chess variant under a Harberger tax, where every piece has a posted price and any piece you attack can be bought and flips sides where it stands.

**Play it:** [Always For Sale](https://claude.ai/artifact/Gi7ZfbFSXrVHtJP94ZNZWK) (private Claude artifact), or open `web/always-for-sale.html` in a browser. You play against the same bot that ran the sweeps.

## Result: Gate A passes, with one big caveat

Gate A asks whether self-play finds a band of settings where buyouts happen every 6–10 plies, the full bot beats both a no-buy bot and a textbook-pricing bot at least 60% of the time (95% CI lower bound above 55%), and draws stay under 15%.

- **Exploration:** 128 Sobol settings × 300 games (seeds 1–50 per matchup). Three settings passed every point estimate: [explore_cells.csv](results/explore_cells.csv).
- **Pre-registration:** gates, the three centres and the confirmation design were committed in [harness/gates.yaml](harness/gates.yaml) at commit `75a4884`, before any confirmatory game. (The tag `afs-gate-a-prereg` exists locally, but this environment couldn't push tags.)
- **Confirmation:** 234,000 games on fresh seeds. **GO.** Around the "Lean" centre (x033), three adjacent tax rates (3.5%, 4.5%, 5.5%) and three starting treasuries (13, 18, 23 coins) pass. The other two centres don't; one failed at its own centre, the winner's curse the protocol guards against. Full tables: [GATE_A.md](results/GATE_A.md).
- **Caveat:** a [follow-up test](results/FOLLOW_UP.md) shows repricing *during play* adds nothing for these bots (50.0% against an identical bot that never reprices). Pricing matters through the opening price level and the tax it costs, not as a move. The bots reprice on about 1% of turns. Gate B needs people to reprice on 10%.

The passing settings:

| Setting | Start | Tax per round | Rent per centre square | Reprices per turn | Coin scale (bot) |
| --- | --- | --- | --- | --- | --- |
| Lean (x033) | 12.6 coins | 5.53% of posted prices | 0.17 | 1 | 217 centipawns per coin |

Other fixed rules: progressive tax 10% of treasury above 50.7 coins, tax rising 0.51 points a round from round 30, 200-ply limit.

### What to do before Gate B

The plan's Gate B (a Steam Playtest where the median player reprices on at least 10% of turns) is the real test of the design's signature idea, the quiet move that makes several enemy quotes stale. The bots suggest repricing is currently too weak to compete with a move. Options worth a cheap bot test first:

1. **Reprice alongside a move:** allow one reprice in the same turn as a move or buy. Repricing becomes routine, so the skill moves to choosing which quote to update.
2. **Stale quotes decay:** a piece's price drops by 1 each round it isn't repriced, so ignoring quotes has a cost.
3. **Rule out the bot first:** rerun the no-reprice comparison with a depth-3 or depth-4 bot. If repricing starts to win, the rules are fine and the depth-2 bot was too shallow to see forks of cheap quotes coming.

Each can be tested with the same harness in under an hour.

## What's here

| Path | Contents |
| --- | --- |
| `src/rules.rs` | Rules core: board, legality, buyouts, repricing, tax, rent, bankruptcy |
| `src/bot.rs` | Evaluator with a flip-swing term, depth-2 alpha-beta over moves and buys, hedged pricing, stale-quote repricing; the no-buy, rote, no-reprice and Monopolist variants |
| `src/sim.rs` | One game between two bots, summarised for the harness |
| `src/bin/afs-sim.rs` | CLI: `run` (sweep cells to JSONL), `game` (one annotated game), `perft` |
| `src/wasm.rs` | Browser entry point (JSON in, JSON out) |
| `src/tests.rs` | Rule tests: buyouts, the no-buy-back ply, buying the checking piece, promotion, settlement, random-game invariants |
| `harness/sweep.py` | Sobol exploration, pre-registered confirmation lines, DuckDB metrics, bootstrap CIs, the gate report |
| `harness/gates.yaml` | The pre-registration |
| `results/` | Exploration and confirmation summaries (raw game logs are git-ignored) |
| `web/` | Page template and build script; `always-for-sale.html` is the built page |

## Checks

- **Move generation** matches Fairy-Stockfish's Los Alamos implementation (via `pyffish`) from the start position: 10, 100, 1,212, 14,332 and 191,846 positions at depths 1–5. The plan's check on 50 further positions isn't done yet.
- **Rules:** 12 tests pass (`cargo test --release`).
- **Speed:** about 80–100 bot games a second on 4 cores.

## Departures from the plan

- No Fairy-Stockfish in the loop: flip swing uses the in-house static evaluator for every run, which also avoids the GPL question. Fairy-Stockfish is used only to check move generation.
- A bought piece can't be bought back on the very next ply. Without this, two bots trade the same piece back and forth.
- Promoted pieces are priced at their textbook value (Q9, R5, N3).
- The front end is a web page (WebAssembly) instead of Godot, and there are no relics, shop or multi-board runs yet.
- "Turns" in the buyout gap are plies (one player's action).

## Rebuild

```sh
cargo test --release
python3 harness/sweep.py explore    # ~7 min on 4 cores
python3 harness/sweep.py confirm    # ~45 min; needs harness/gates.yaml
python3 web/build.py                # needs: rustup target add wasm32-unknown-unknown
```

Python needs `scipy numpy pandas duckdb pyarrow pyyaml`.
