# Idea 06 — Seedless
> Seed: 6. [wild] **Quantum randomness × simulation theory × generative world models.** Provably non-pseudorandom entropy as the "glitch in the Matrix."

### Turn 1 — Visionary
**Working name:** Seedless

**Hook:** A generative world with no seed. An AI "Demiurge" predicts your next choice so it can pre-render the world cheaply. When you surprise it, reality is drawn fresh from a certified quantum beacon.

**Core mechanic.** The verb is **surprise the simulator**.
1. A Genie-style world model renders the scene. What matters, like what's behind the door, is decided by beacon pulses timestamped *after* you committed your move. Nobody, developer or speedrunner, could have known it.
2. At each fork a predictor bets on your choice first. Aaronson's keypress "Oracle" guesses most people right roughly 70% of the time. Predicted choices get cheap, stale terrain. Surprises open "unrendered" territory, scored by the predictor's log-loss.
3. Verified-unpredictable bits are banked and offered to a Bell-test lab as measurement settings, as the 2016 BIG Bell Test did with about 100,000 volunteers.

**Why the domains collide.** "Glitch in the Matrix" looks in the wrong place. A good PRNG passes every statistical test, so no bitstream can prove it is quantum. Non-pseudorandomness belongs to the *process and its proof*, not the pattern. Device-independent randomness assumes your adversary built the device, which is exactly the simulator threat model. Its only escape hatch is superdeterminism: settings correlated with the particles all along. So simulation theory's sharpest testable form is the freedom-of-choice loophole, and outguessing a predictor is the most fun way to produce free choices. Honest limits: expect a null, a non-local simulator could run quantum mechanics fine, and human bits can't close the locality loophole.

**Why now.** Certified randomness is now a public service: CU/NIST's CURBy Bell-test beacon and Quantinuum–JPMorgan certified randomness (both *Nature*, 2025). Helios hit 99.9995% logical fidelity on Oct 19, 2026 (brief §6). World models made lazy rendering an engineering object (§7). The brief's §6 lists verifiable quantum randomness in consumer products as an open gap.

### Turn 2 — Builder
**What exists.** **Randonautica** (2020) already sold ANU QRNG entropy plus Matrix-flavoured "intention" to millions of users. Verifiable game randomness is shipped plumbing: **drand** (League of Entropy, 3-second rounds) and Chainlink VRF. On world models, **Genie 3** is still a research preview, while Decart's Oasis and the open Matrix-Game 2.0 run in real time but drift. The demand signal is speedrunning: Dream's 2020 Minecraft run was rejected over drop luck nobody could audit.

**Corrections.** Lazy rendering saves *latency*, not cost: a world-model frame costs the same whether you predicted it or not. CURBy produces a few hundred 512-bit pulses a day, which is far too slow for per-move play. Quantinuum's certified bits needed Frontier-class supercomputers to verify. And cosmic Bell tests (2018) already used quasar light to push freedom-of-choice back about 7.8 billion years, so labs aren't short of settings. Make the BIG Bell donation a seasonal event, not the loop.

**Hardest problem: the dominant strategy is to be a coin.** Against a predictor, uniform randomness maximises surprise. Players open a dice app, the "free choices" become pseudorandom, and the game is rock-paper-scissors against a bot, which is fun for three minutes.

**MVP.** A browser roguelite on a forking map: an n-gram/LLM predictor, drand for per-move reveals, a CURBy pulse sealing each floor, pre-baked world-model vignettes, a public verification page.

**Twist: sealed prophecies.** The Demiurge hashes its prediction *before* you move and builds the next room's traps around it. Surprise scores only if you also survive, and the options carry unequal payoffs, so the right play is a skilled mixed strategy (poker, not dice), and a coin-flipper dies. After the run, the hashes open: "It called 41 of your 60 moves." That receipt is the glitch you can feel, and it proves the house committed before seeing you.

### Turn 3 — Visionary
Corrections accepted. CURBy seals floors, drand handles moves, and the Bell donation becomes a seasonal event. The lazy-rendering correction gives us an aesthetic: predicted rooms were pre-generated while you deliberated, so they appear instantly, and a surprise makes the world visibly **stutter** while it renders. Déjà vu, the cat walking past twice, is now a real tell.

**Mutation: sealed prophecies are Newcomb's problem.** The unseeded domain is decision theory. Sealed prophecies come in two room types that look identical:
- **Adversary rooms** play matching pennies. The Demiurge trapped the path it predicted, so surprise pays.
- **Oracle rooms** are Newcomb boxes. The Demiurge filled the opaque chest only if it predicted you'd leave the clear one alone. Against a predictor right about 70% of the time, one-boxing wins easily, *but only if you're predictable*.

That kills the coin. A coin-flipper loses in Oracle rooms because an unpredictable agent gets empty boxes. The skill is **selective legibility**: be readable when cooperation pays and opaque when it doesn't, and work out which room you're in from tells.

**The house's own randomness gets certified.** The Demiurge's mixed strategy draws from the beacon, so the house can't adapt mid-run. Your choices are the only free variable in the system.

**Emergent behaviour.**
1. Every opened prophecy is public, so the community reverse-engineers the Demiurge's tells, as poker players did with solvers. It retrains each season on everyone. The boss is the population's own past predictability.
2. Players split into schools, much as philosophers split into one-boxers and two-boxers. Receipts settle the argument empirically for this predictor.
3. The by-product is a growing dataset on how predictable humans are against frontier models, which bears directly on Aaronson's "freebit" speculation. The Bell test was never going to produce that.

### Turn 4 — Builder
**Kills.**
- *The "freebit" dataset.* Accuracy measures the model and the player's effort, not Knightian freedom. It's a consented predictability benchmark, nothing more.
- *Fake stutter.* In a game sold on provenance, a scripted stutter is a lie. It's real generation time or nothing.
- *Indistinguishable rooms.* If nothing tells Oracle rooms from Adversary rooms, the right play is one fixed mix and learning dies. Tells exist and are probabilistic, and the floor-level ratio is disclosed.

**Business model.** A premium roguelite ($15–20, Steam) with free seasonal retrains. No paid randomness: that's a loot box, which Belgium bans. Year two, a B2B **sealed-RNG SDK** for verifiable runs in other roguelites and on speedrun boards.

**Stack.** Godot. A per-player context-tree predictor plus a population transformer. SHA-256 commits that include the drand round, in an append-only Merkle log. drand verified client-side, CURBy checked through its Twine chain.

**Failure modes and abuse.**
- *Calibration.* Aaronson's ~70% is for binary key-mashing. Choices with payoffs may be 90% predictable, which makes Adversary rooms brutal. Tune Newcomb payoffs to measured accuracy every season.
- *Solvers.* Public tells mean bots will model the Demiurge. Run a Solver League instead of fake anti-cheat.
- *Commit grinding.* The client refuses input until it holds the logged commit.
- *Profiling.* A per-person predictability model is ad-tech gold, so it is never sold, and research data is aggregated only.
- *Derealization.* "You escaped the simulation" copy courts the AI-psychosis pattern, so keep the tone playful and claim no proofs.

**90 days.**
- Weeks 1–3: a text prototype, accuracy measured on 200 testers.
- Weeks 4–8: a three-floor slice, receipts screen, and verification page.
- Weeks 9–12: a Next Fest demo. **Kill metric:** if a coin-flip bot or an always-predictable bot scores within 10% of top-decile humans, there's no skill gradient. Redesign the payoffs.

### Turn 5 — Visionary
All three kills accepted. One addition to the kill metric: run a **mimic bot** that copies the population's median mix. If it reaches the top decile, players aren't reading tells, and selective legibility isn't a skill.

**Final form.** Seedless is a premium roguelite about being predicted. Before each move, the Demiurge (a per-player predictor with a population prior) commits a hashed prophecy that includes the drand round. Rooms are matching-pennies Adversary rooms or Newcomb Oracle rooms. Tells are probabilistic, and each floor discloses its ratio. Surprise pays in one room type and legibility in the other, so the skill is knowing which self to show. The house's mixing comes from the beacon, and CURBy pulses seal each floor. The receipt opens every prophecy: "It called 41 of your 60." The game adds seasonal retrains, a Solver League, and a Bell-settings festival that is never billed as proof.

**Moonshot (2036): prediction receipts as a civic right.** Recommenders, ad auctions and insurers predict us constantly and never show their work. Sealed prophecy becomes a standard: any system that acts on a prediction of your behaviour must commit to it before you act and open it afterwards. Your feed, bank and car send monthly receipts, like "We called 83% of your choices." Seedless is where people first *feel* modelled and learn to manage their legibility.

**Scores.** **Innovation 6.** An empirical Newcomb arena with cryptographic prophecies is a new mechanic, and the receipts institution is close to wild. But honestly we walked the seed down from "glitch in the Matrix" to a bold game, and the physics is now garnish. **Feasibility 6.** Godot, drand and an n-gram predictor ship the core. Live world-model rendering and calibrating payoffs against highly predictable humans are the risks.

### Turn 6 — Builder
The mimic bot is the right addition, and I'd build it before any art, because Seedless lives or dies on whether reading tells beats copying the crowd. My verdict: a real, shippable bold game whose genuinely new piece is the sealed prophecy, which turns Newcomb's problem into something you can lose at and audit. I'm scoring innovation 5, not 6, because commit-reveal and keypress oracles are known primitives and the quantum layer is now garnish. The receipts-as-civic-right moonshot is where I'd point the B2B SDK.
