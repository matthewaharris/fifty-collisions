# Fifty Collisions

Fifty app, game, system and institution ideas, each built from two to four fields that rarely meet. Two Claude agents worked through every idea in a fixed six-turn discussion: a **Visionary** pitched and pushed it, and a **Builder** grounded it, stress-tested it and wrote the record. Every idea is scored for innovation and feasibility on a 10-point scale, anchored to OACP ([liteneen.net](https://liteneen.net)) at innovation 3.

## Published pages

- [Fifty Collisions](https://claude.ai/artifact/YPAsb9so3a9x6JD263G3rv): the map, filterable idea cards, full dossiers and discussions, with the 11 prototype picks marked.
- [Plain-Language Guide](https://claude.ai/artifact/QYkQSoJ8Q6PrkDVbWh5Mhd): every idea in everyday words, plus the prototype shortlist and a jargon decoder.

## Prototype shortlist

Each pick has a high-level build plan in [`plans/`](plans/README.md), with a timebox, budget, architecture and pre-registered go/no-go test.

| POC | # | Idea | Go/no-go test |
| --- | --- | --- | --- |
| 1 | 47 | Piepowder | Do pairs of online juries agree on replayed agent failures more often than chance? |
| 2 | 39 | Bottleneck | Does it block more prompt-injection attacks than the best current defence on a standard attack benchmark? |
| 3 | 45 | Repro | Does a sandbox with planted secrets filter junk bug reports while letting real ones through? |
| 4 | 3 | Litmus Ghost | Does any consciousness-theory team engage with a $20K counterexample season on IIT? |
| 5 | 9 | Caught | Over four weeks, does breaths-to-catch improve, and does that improvement track an independent attention test (SART)? |
| 6 | 8 | Smoke-Filled Room | Do AI bots rediscover 3 of 4 historical auction exploits on their own? |
| 7 | 24 | Always For Sale | Is there a tax setting where buying pieces matters without breaking the game? |
| 8 | 44 | Seventeen Moments | In millions of bot games, does any single card dominate? |
| 9 | 1 | Precedent | Do doctors sign at least half the pre-filled letters within 7 days, across 200 live appeals? |
| 10 | 50 | Dry Run | Do AI agents find the 2017 tax bill's grain-glitch loophole within 72 hours? |
| 11 | 10 | One Take | Does sharing the same quantum-seeded dice cut the noise in model comparisons by at least 30%? |

## What's in this repo

| Path | Contents |
| --- | --- |
| `catalog.md` | One line per idea: name, tier, innovation/feasibility, tagline |
| `records/idea-NN.json` | The structured record for each idea, written by the Builder on turn 6 |
| `transcripts/idea-NN.md` | The full six-turn discussion for each idea |
| `all-transcripts.md` | Every transcript in one file |
| `seeds.md` | The 50 starting prompts plus the bonus round |
| `PROTOCOL.md` | The rules both agents followed, including the tier-discipline rule added after batch 1 |
| `persona_visionary.md`, `persona_builder.md` | The two agents' role prompts |
| `research/trends_brief.md` | The October 2026 trends research the agents used for grounding |
| `plain/` | Plain-language summaries (`batch-*.json`), their brief and the script that formats them |
| `fresh-spins/` | Eight near-term product ideas: fresh spins on boring products aimed at market fit within 12 months, each with a demand test and kill metric. Page: [Fresh Spins](https://claude.ai/artifact/3fX3cqisXuWbY9o94MCqiq) |
| `prototypes/` | Working prototypes: Always For Sale (rules engine, sweeps, play page) and Caught (phone web app) |
| `plans/` | High-level build plans for the 11 prototype picks, with an index, a suggested build order and the components they share |
| `site/` | The page template and the build script |
| `fifty-collisions.html` | The built page |
| `archive/` | Retired material: #10 Crossover and the catalog before the swap |

## Rebuilding the page

```sh
python3 site/build.py
```

This reads `records/` and `transcripts/` and writes `fifty-collisions.html`. The prototype shortlist is the `POC` list in `site/build.py`.

## Changes since the original run

- **#3** was renamed from Zombie Bounty to **Litmus Ghost**. Its contest modes are now Hollow and Overlooked.
- **#10 Crossover** was retired because it duplicated #25 Offtaker. Slot 10 now holds **One Take**, from a later bonus round on quantum randomness and AI.
- **Prototype plans** were added in `plans/`. For Caught and Precedent, the go/no-go test above now matches each record's own kill metric. The earlier questions (chest-strap accuracy, and a pharmacist and lawyer audit) are now early milestones.
- **Fresh Spins** (`fresh-spins/`) added eight near-term ideas over two rounds: Ran It, Posted, Morning Tile (plus an app-version supplement), Counter (the OACP one) and Afterlist; then, apps only, Cashout, Seated and While You're Here.
