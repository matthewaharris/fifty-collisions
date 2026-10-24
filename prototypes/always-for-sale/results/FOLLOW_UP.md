# Follow-up: does repricing during play matter?

Not pre-registered. This was run after the Gate A decision, on fresh seeds (from 2,000,000,000), at the x033 centre.

The rote ablation in Gate A differs from the full bot in two ways: it posts textbook prices at the start, and it never reprices. To separate the two, the full bot played a copy of itself that posts the same hedged opening prices but never reprices (`no_reprice` in `src/bot.rs`). Each row is 1,000 seed pairs, 2,000 games.

| Reprice limit | Full bot's score vs no-reprice bot | 95% CI | Full bot reprices on |
| --- | --- | --- | --- |
| 1 (the x033 setting) | 50.0% | 48.3–51.7% | 0.8% of its turns |
| 2 | 50.9% | 49.2–52.7% | 0.9% of its turns |

**Reading.** For these bots, repricing during play adds nothing measurable. Gate A's "pricing matters" result comes from the level of opening prices, which sets each side's tax bill, not from repricing as a move. Either the depth-2 bot undervalues repricing, or the rules make it a weak move. Gate B (whether people reprice on at least 10% of turns) is now the deciding test, and it would be worth a rules change before spending on a Steam Playtest (see the README).

Reproduce: add cells with `"matchup": "vs_no_reprice"` and run `afs-sim run`.
