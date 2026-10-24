# Idea 04 — Rule of Thumb
> Seed: 4. [mild] **Freestyle/Chess960 × LLM coaching × pedagogy.** Openings can't be memorized anymore, so teach *principles* instead.

### Turn 1 — Visionary
**Working name:** Rule of Thumb

**Hook:** A Chess960 coach that treats chess principles as hypotheses. Before each move you name the principle you're following, and the engine checks whether it holds in *this* starting position.

**Core mechanic (the verb is *falsify*):** Principles get taught as commandments (knights before bishops, castle early). Chess960 is the natural experiment that shows they're really **position-dependent heuristics**. With the king on b1 and bishops in the corners, "castle early" may lose half a pawn. The loop:
1. At each critical move, the student taps one of about 15 principles ("I'm contesting the centre," "I'm opening the cornered bishop's diagonal").
2. The engine runs a **principle ablation**: the best line that follows the principle against the best line that breaks it, with the eval delta between them.
3. The LLM only *narrates that contrast*. It never generates chess analysis itself. The engine supplies facts; the LLM supplies grammar.
4. Your **principle profile** builds up over time: which heuristics you over-trust, and in which kinds of start positions.

**The second layer is a Principle Atlas.** We precompute engine self-play across all 960 starts to measure where each classical principle holds, weakens or inverts. Position 518, standard chess, turns out to be one special case of 960.

**Why the domains collide:** Pedagogy research says interleaved, varied practice builds transferable understanding, while blocked repetition builds brittle recall. Opening memorization is blocked repetition at its most extreme, and Chess960 is interleaving built into the rules. An LLM coach becomes reliable once it's demoted from analyst to narrator of engine-verified contrasts.

**Why now:** The first official FIDE Freestyle World Championship was held in February 2026, and Esports World Cup chess carries a $1.5M pool. Chess.com's Play Coach shows demand; independent tests show LLM coaches still blunder tactically.

### Turn 2 — Builder
**What exists.** **DecodeChess** has done "engine supplies facts, explainer narrates" since 2017, with plans and concepts in plain language. **Aimchess** builds weakness profiles from your Lichess and Chess.com games. Chess.com Game Review and Play Coach own the mass market, and Lichess gives 960 analysis away free. On the research side, DeepMind's concept-probing work on AlphaZero (McGrath et al., PNAS 2022) and its 2020 study of alternative chess rules with Kramnik are the precedent for the Atlas. Nobody frames principles as *falsifiable per start position*.

**Hardest problem: "principle ablation" is ill-defined, and engine truth is the wrong truth.** "Castle within 10 moves" compiles to a move predicate. "Contest the centre" and "improve your worst piece" don't, and a forced violating line often measures a tactic, not the principle. Worse, Stockfish's verdict tells a 1400 player nothing. A principle can be engine-false and still win games at club level, because it simplifies decisions.

**MVP (90 days).**
- A web app with Lichess OAuth that imports your 960 games.
- **Post-game** review of 3 critical moments. Tagging a principle before every move is friction nobody keeps up.
- 12 principles operationalized as python-chess predicates. The fuzzy ones wait.
- Stockfish 17 in WASM for the contrast lines.
- An LLM narrator restricted to the lines and evals we hand it, with a validator that rejects any move not in the supplied PV.

**Twist: make principle truth rating-dependent.** Build the Atlas from **real human 960 games**, not engine self-play. The Lichess open database holds millions, split by rating band, and **Maia-2** (the human-like engine) can ablate at *your* level. The coach then says something more interesting: "Castling early in this start is −0.4 for Stockfish, but 1400s who do it score 6% better." Principles graduate as you climb. The profile becomes a map of which heuristics you've outgrown.

### Turn 3 — Visionary
Rating-dependent truth changes the product from "is this principle correct" to **"is this principle still paying for *you*."**

**Solving ablation: mine principles instead of compiling them.** Hand-written predicates break on "contest the centre," so turn the direction around. Compute about 60 cheap positional features at move 12 (central pawn count, developed minors, king-shelter pawns, open bishop diagonals). Then, per start position and rating band, find the feature combinations that best predict the score gap between players who advance to the next band and those who stall. Those combinations *are* the principles, measured from human games. The LLM's only job is to name them, and humans approve the name. Fuzzy principles become discovered regressions, not brittle code.

**Emergent: new principles nobody has named.** Some combinations will be specific to 960, for example: "with adjacent bishops, trade the one whose diagonal your own knight blocks." The community gets naming rights on verified discoveries, so 960 theory gets written as a canon of principles, not lines.

**Unseeded domain: U-shaped learning from child language acquisition.** Children say "went," then "goed," then "went" again. The overgeneralization dip is a sign the rule is being *learned*, not a regression. Chess principles follow the same curve: adopt "open the cornered bishop" and you'll overapply it and lose rating for a few weeks. Rule of Thumb predicts the dip from your profile and tells you it's coming, which turns the most demoralizing moment in improvement into a milestone.

**Second-order effect: scouting.** A principle profile built from someone's *public* games is a map of the heuristics they over-trust. Strong players will use it for opponent prep ("she castles early even in b1 starts; delay central tension"). An adversarial edge on a teaching tool, and probably what sells the premium tier.

### Turn 4 — Builder
**The flaw in mining: confounding.** Climbers do *everything* better, mostly tactics, so regressing on advancement rediscovers "be stronger." The fix is **within-player fixed effects**: compare the same player across starts where they did and didn't follow the feature with game score as the outcome, controlling for opponent rating. Data thins fast per start and band, so **cluster the 960 starts into about 30 structural families** (king file, bishop colour layout, corner pieces) and key principles to families. Mined principles must replicate on held-out months before an LLM names them.

**The U-dip: test it first.** Lichess rating histories can show whether adoption precedes a dip. Until then it's copy, not a prediction.

**The market is small; say so.** 960 is a sliver of online play. It survives because profiles also apply to standard chess (position 518 is one family). Pricing is freemium at $7 a month, in line with Aimchess. **Scouting** is the premium tier: it uses public Lichess API data only, finished games only, never a live game. Add a scholastic coach dashboard and a Freestyle Chess broadcast "principle meter."

**Stack.** Lichess open-database PGNs fed through python-chess and DuckDB, fixed-effects models in statsmodels, Maia-2 on one GPU, Stockfish 17 WASM in the browser, an LLM narrator behind a PV validator, Postgres, Next.js.

**Failure modes.**
- *Spurious principles that confuse learners.* Mitigated by the holdout gate and human sign-off on every name.
- *Fair-play complaints.* The app ingests nothing from games in progress.
- *Offensive community names.* Moderate them.

**90 days.**
- Weeks 1–3: ingest, extract features, cluster starts into families.
- Weeks 4–6: fixed-effects mining; keep the roughly 20 principles that replicate.
- Weeks 7–9: the post-game app.
- Weeks 10–12: 500 Lichess 960 players and 3 coaches. **Kill metric:** if more than half the mined principles fail holdout, ship the hand predicates with Stockfish contrast only.

### Turn 5 — Visionary
Fixed effects and families accepted; regressing on advancement would have rediscovered "be stronger." Families are also a gift: "king on b, bishops in corners" is a *teachable* category; 30 families are learnable, 960 positions aren't. The U-dip stays a hypothesis until the Lichess histories show it.

**Final form.** Rule of Thumb is a post-game coach for Chess960 and standard chess, and its truth is rating-dependent. It mines principles from Lichess games using within-player fixed effects across about 30 structural start families. A principle ships only if it replicates on held-out months and a human approves its name. Each critical moment gets a Stockfish/Maia-2 contrast narrated by a validator-gated LLM. Your **principle profile** shows which heuristics still pay at your level and which you've outgrown. Premium adds scouting (finished public games only), coach dashboards and a broadcast principle meter.

**Moonshot (2036).** The Atlas becomes a **translation layer between engines and humans**. DeepMind already extracted AlphaZero concepts and taught them to grandmasters (Schut et al., 2023). Rule of Thumb generalizes that: concepts the engine discovers are filtered through the fixed-effects test into *the ones that actually help a human at your band*, then named and sequenced into a curriculum. The method also travels beyond chess. Any domain with millions of rated human decisions, such as Go variants, poker, RTS games and eventually supervised clinical or flight-simulator decisions, can get its folk heuristics measured, graded by skill level, and given a known expiry point.

**Scores.** **Innovation 3.** This is at the top of mild. Mined, rating-dependent, expiring principles are a fresh framing, but every component exists (DecodeChess, Maia, Lichess data). **Feasibility 8.** Open data, ordinary stack, a two-person quarter, a planned fallback. The only real risk is market size.

### Turn 6 — Builder
One thing neither of us said: Chess960 *randomly assigns* the start position, so the within-player variation is closer to a natural experiment than almost any other teaching dataset gets. That's the quiet reason mined principles might actually be causal here. I'm recording feasibility at 7, not 8. The fallback ships for certain, but the mined-principle core is unproven until the holdout gate passes. Innovation stays at 3: an honest, very buildable product that will be a better teacher than a business.
