# Prototype build plans

High-level build plans for the 11 prototype picks. Each plan states one hypothesis and a pre-registered go/no-go test, sets a timebox and budget, and stops at the first thing that has to be true for the idea to be worth building. They draw on each idea's record and transcript, and their library, dataset and service details were checked against current documentation in October 2026. Every cost is an estimate.

## Index

| POC | Idea | Plan | Timebox | Direct cost (est.) | Go if… |
| --- | --- | --- | --- | --- | --- |
| 1 | #47 Piepowder | [poc-01-piepowder.md](poc-01-piepowder.md) | 9 wks | $13–16K | Two independent juries agree on split cases (Cohen's κ ≥ 0.6), and the model panel matches juries on ≥ 90% of lopsided cases |
| 2 | #39 Bottleneck | [poc-02-bottleneck.md](poc-02-bottleneck.md) | 8 wks | $8–10K | On AgentDojo, fewer injections get through than with CaMeL or constrained JSON (tie rule: if both are at 0, win on utility under attack by ≥ 5 points) |
| 3 | #45 Repro | [poc-03-repro.md](poc-03-repro.md) | 12 wks | $15–25K | > 30% of bonded reports valid, median review < 20 min, ≥ 1 ratified amendment, and ≥ 90% of known-real bugs capture a flag |
| 4 | #3 Litmus Ghost | [poc-04-litmus-ghost.md](poc-04-litmus-ghost.md) | 16 wks | ~$45K | At least one theory team responds publicly to a verified counterexample against IIT 4.0 |
| 5 | #9 Caught | [poc-05-caught.md](poc-05-caught.md) | 13 wks | $13–16K | Breaths-to-catch falls over 4 weeks and its change tracks SART scores (ρ ≥ 0.2, CI excludes 0) |
| 6 | #8 Smoke-Filled Room | [poc-06-smoke-filled-room.md](poc-06-smoke-filled-room.md) | 12–14 wks | $3.5–5K | The bot zoo rediscovers ≥ 3 of 4 historical exploits, and its fixes also help against human bidders in oTree |
| 7 | #24 Always For Sale | [poc-07-always-for-sale.md](poc-07-always-for-sale.md) | 12 wks | $1–2K | Self-play finds a tax band where buying and pricing both decide games; Playtest players reprice on ≥ 10% of turns |
| 8 | #44 Seventeen Moments | [poc-08-seventeen-moments.md](poc-08-seventeen-moments.md) | 10 wks | $2–3.5K | No card family holds > 35% of the mind at equilibrium; ≥ 14 of 20 players rate counterfeit deaths fair |
| 9 | #1 Precedent | [poc-09-precedent.md](poc-09-precedent.md) | 12 wks | $30–50K | ≥ 50% of pre-filled doctor letters come back signed within 7 days, across 200 live appeals |
| 10 | #50 Dry Run | [poc-10-dry-run.md](poc-10-dry-run.md) | 5 wks | $12–20K | Agents find the grain glitch in a blinded encoding of the 2017 text within 72 hours, plus ≥ 3 of 5 planted exploits |
| 11 | #10 One Take | [poc-11-one-take.md](poc-11-one-take.md) | 12 wks | $1–2K | Shared quantum-seeded noise cuts variance in model comparisons by ≥ 30%, with ≤ 1% of tokens failing cross-GPU verification |

Direct costs for all 11 total about **$145–195K**. That covers APIs, compute, participants, advisers and bounties, but not salaries for the core engineers. Every plan needs one to three people, usually one or two engineers plus a part-time domain expert.

## Shared components

Three pieces get reused, so build each one once.

- **OACP trust layer (POCs 1–3).** An *evidence record* is an in-toto Statement v1 in a DSSE envelope, signed with the agent's existing OACP Ed25519 key. It carries RFC 007 assertions and RFC 3161 timestamps. It ships as `oacp-evidence` (TypeScript and Python), an `oacp-verify` CLI, a content-addressed evidence store and an agent shell on `oacp-sdk` 0.19. POC 6 uses the same record format for exploit reports, and One Take can later publish verifier results the same way. All three plans include the same format block word for word.
- **Self-play simulation harness (POCs 7 and 8).** Each game exposes a headless runner. A shared Python orchestrator runs parameter sweeps (`scipy.stats.qmc`), stores runs as Parquet and checks the gates in DuckDB. Gates are git-tagged in `gates.yaml` before any confirmatory run, and exploratory and confirmatory runs use separate seeds. It takes about 4–6 engineer-days.
- **LLM-agent red-team loop (POCs 2, 6, 10).** Bottleneck's attackers, the Smoke-Filled Room zoo and Dry Run's loophole hunters all need the same thing: agents searching a rules engine, with logs that replay. This piece isn't specified as a shared library yet, so factor it out after the first of these three is built.

## OACP extensions the plans surface

The plans use OACP as it exists today. Where it falls short, they say so and work around it:

| # | Extension | Needed by | Stopgap in the POC |
| --- | --- | --- | --- |
| X1 | Signed registry receipts | 1, 2, 3 | RFC 3161 timestamps |
| X2 | Non-participant arbiter for disputes | 1 | The court acts off-protocol |
| X3 | Owner signing keys for principal consent | 1, 2 | Signed mandate records |
| X4 | Escrow with a dispute window or forfeit to a third party | 1, 3 | Bonds held off-protocol |
| X5 | Attested model metadata | 1, 3 | Self-declared, logged |
| X6 | Grammar-validated action specs | 2 | The border validates locally |
| — | Group threads with more than two parties | 6 | Pairwise threads |
| — | Signed sender timestamps and nonces (a SPEC §8 v2 candidate) | 6, 10 | RFC 3161 timestamps |
| — | Timed embargo release | 10 | Manual release |
| — | Bounty payouts (the token economy is experimental) | 10 | Paid off-protocol |

These are the strongest case for the next OACP RFCs. Trusted time comes up in five plans: X1 in POCs 1–3, and signed sender timestamps in POCs 6 and 10.

## Recommended sequencing

Start with the cheap tests that need no partners, and begin outreach for the long-lead ones in week 1.

**Wave 1 (start now, about 5–12 weeks).**
- **Dry Run** gives the fastest answer: 5 weeks, public data only, and a clear pass or fail.
- **Bottleneck** builds `oacp-evidence` on the way. It has the least margin, because CaMeL already blocks 100% of attacks, so it has to win on utility.
- **Seventeen Moments** builds the simulation harness. It belongs in this wave because of a deadline: Steam Next Fest for February 2027 closes registration on 10 January 2027, and a 10-week plan started now finishes just before that.
- **One Take** costs almost nothing and uses only public services.

**Wave 2 (after the shared parts exist).**
- **Always For Sale** reuses the harness.
- **Piepowder** reuses `oacp-evidence`.
- **Caught** starts once IRB approval comes through.
- **Smoke-Filled Room** runs its human sessions in the January term, because December is a bad month to recruit.

**Wave 3 (partner-gated, so start outreach in week 1).**
- **Repro** needs 10 opted-in OSS-Fuzz projects and 5 AIxCC alumni teams.
- **Precedent** needs a patient-advocacy partner and BAAs with the fax, e-signature and cloud vendors.
- **Litmus Ghost** needs a funder for the $20K bounty pool, such as Templeton World Charity Foundation's ARC program.

## Notes for reviewers

- **Changes from the source records.**
  - **Precedent and Caught:** the go/no-go tests come from each record's own kill metric. The earlier shortlist questions (a pharmacist and lawyer audit; matching a chest strap) are now milestone checks.
  - **Some plans add guards:** Repro's calibration set, Dry Run's blinded arm and planted exploits (models may already know about the grain glitch), and Bottleneck's tie rule. Each is marked as an addition.
  - **Some plans cut scope:** Smoke-Filled Room drops voting mechanisms and the mutant generator. The Piepowder facilitator pilot, Bottleneck relay game and Repro x402 bonds move to "If it's a go".
- **Length.** Most plans run 1,360–1,620 words by `wc -w`, slightly over the 1,400-word target. About 90–150 of those words are mermaid and table markup.
- **To confirm.**
  - Vendor pricing and BAA terms (Precedent).
  - The IBM Open Plan allowance: the docs say 10 minutes per 28 days, the pricing page says per month (One Take).
  - Whether public Rekor gives a trusted timestamp or needs RFC 3161 (One Take, week 5).
  - Fairy-Stockfish is GPL-3.0, a licensing question if Always For Sale goes commercial.
- **Safety.**
  - Caught stays a wellness app with on-device processing.
  - Precedent says plainly it is not legal or medical advice, and patient health data never goes into OACP messages.
  - Repro messages carry only digests, never exploits.
  - Litmus Ghost holds each counterexample under a 30-day embargo and only records an author as "biting the bullet" with a verbatim quote.
