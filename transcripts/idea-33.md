# Idea 33 — Minute Hand
> Seed: 33. [wild] **Quantum computing × puzzle games × algorithm discovery.** Players solve levels on real QPUs, and good solutions become research.

### Turn 1 — Visionary
**Working name:** Minute Hand

**Hook:** A tile-merging puzzle whose rules are a complete axiomatization of quantum mechanics. Its global high score is the date of Q-Day.

**The isomorphism.** The ZX-calculus draws quantum circuits as graphs of green and red "spiders." Its central rewrite, spider fusion, merges adjacent same-colour spiders and adds their phases. That is 2048. ZX is also **complete**, first for Clifford+T (Jeandel, Perdrix & Vilmart, 2018), then for all qubit QM: every true circuit equation is reachable by legal moves. Players write proofs, not toys. Every move is sound, so every solution is correct by construction; Foldit needed a lab to check designs, here checking is free.

**Core loop. The verb is fuse.**
1. **Level.** A real subcircuit from a published resource estimate: the modular arithmetic inside Shor's algorithm for elliptic curves.
2. **Fuse, pivot, complement.** Rewrite by dragging spiders. Score is **T-count**: T gates need magic-state distillation, which dominates fault-tolerant cost.
3. **Run.** Nightly, winning subcircuits small enough for Helios or Nighthawk run on hardware, with measured fidelity before and after: "your move made the computer 0.4% more accurate."
4. **Splice.** Improvements flow back into the full Shor-for-ECC estimate, and the **Q-Day clock** on the home screen moves.
5. **Replay.** Every winning rewrite path is a proof trace, training a search model the way Lean proofs feed AlphaProof.

**Why this is wild, honestly.** The bet is that human pattern sense finds rewrite *strategies* that searches miss, and that distilled strategies count as algorithm discovery. Quantum Moves hints the human edge fades once algorithms catch up. The clock reports either way.

**Why now.** The cost to break ECC fell to about 1,098 logical qubits (§2). Helios hit 99.9995% logical fidelity four days ago (§6). Oxford's "Quantum in Pictures" pilot taught ZX to schoolchildren.

### Turn 2 — Builder
**What exists.** The puzzle board exists. ZXLive (Oxford and Quantinuum) is a drag-the-spiders GUI for ZX proofs, and PyZX does automatic T-count reduction (Kissinger & van de Wetering, 2020). Citizen-science precedents include Foldit, Quantum Moves and Wootton's Decodoku. The opponent to respect is **AlphaTensor-Quantum** (DeepMind, 2025). It cut T-counts on these exact arithmetic circuits and rediscovered Karatsuba-like multiplication. Players are racing a reinforcement learner.

**Hardest problem: where is the human edge?** Small instances are where search is strongest. Shor-for-ECC runs to tens of millions of Toffolis, which no human can play. Two quieter traps. First, getting a circuit back out of an arbitrary ZX diagram can be #P-hard (de Beaudrap, Kissinger & van de Wetering, 2022), so a beautiful low-T diagram may never become a circuit again. Second, running a 12-qubit fragment on Helios measures physical noise, not fault-tolerant cost. That's a trophy, not evidence.

**Twist: play the instance, win the family.** Humans don't beat search at instances. They beat it at **generalization**. Each level is a parametric family, a modular adder at n = 3, 4 and 5. The player's rewrite path is recorded as a **tactic**, and the server replays it at n = 256 under the scalable ZX calculus (!-boxes). The score is T-count saved at cryptographic size, measured against the best bot. A tactic that works only at n = 4 scores nothing. The winning artifact is a strategy, not a diagram. Extractability is built in: moves that break the gflow condition grey out, so every board stays a circuit.

**MVP.** A web game on ZXLive and PyZX: 30 families from Litinski's 2023 ECC arithmetic, tactic replay at n = 256, and PyZX and AlphaTensor-Quantum baselines on the leaderboard.
**Kill metric:** within 90 days, no player tactic beats the best bot on any family at n = 256.

### Turn 3 — Visionary
**Taking tactics, and making the QPU evidence instead of a trophy.** Physical runs are trophies; run fragments at the **logical** level instead. Quantinuum showed logical non-Clifford gates fed by magic states in 2025, and Helios's roughly 48 logical qubits can run a family's n = 3 instance fault-tolerantly. The QPU then measures exactly what the score claims: magic states consumed and logical error per shot. Nightly runs check the T-count accounting on real hardware.

**The human edge, borrowed from speedrunning (unseeded).** Speedrunners settled human versus bot long ago: humans discover "tech," tool-assisted runs execute it perfectly, and both leaderboards coexist. So players don't race AlphaTensor-Quantum; they feed it. Each tactic becomes a **macro-action** in the bot's search. A player's score is how many T gates the bot saves at n = 256 *with* their tactic compared with *without* it. The human edge becomes a measurable marginal contribution; if it's zero, the kill metric fires honestly.

**Emergent: the opening book becomes algorithm discovery.** Tactics get names, the way chess openings do. When one named tactic saves T across unrelated families, say adders, multipliers and QROM lookups, it's a primitive, as Karatsuba's was. The real output is a cross-family tactic book with provenance, and the discovery claim is tested by transfer: a tactic counts only on families it never saw.

**Second-order: two clocks.** Add families from quantum chemistry, such as the FeMoco and P450 Hamiltonians. The home screen now has two hands: **Q-Day** and **Useful Day**, the first chemistry result a classical computer can't match. Arithmetic primitives are shared, so most tactics move both hands. The public sees that the date we can break elliptic curves and the date we can design catalysts are nearly the same date. Migration deadlines get read off a clock players maintain.

### Turn 4 — Builder
**Kills and fixes.**
- *Nightly logical runs to "check T accounting."* T-count is read off the circuit; no hardware needed. What hardware can test is the **cost model**: whether fewer T gates actually lower logical error on real fault-tolerant hardware, or whether CNOT depth and routing dominate. Run that monthly on Helios as calibration, on research credits.
- *Raw T-count as the score.* Magic-state cultivation (Gidney, Shutty & Jones, 2024) made T gates cheaper, so qubit-count and depth tradeoffs now matter. Score in spacetime volume from Google's Qualtran estimator, with assumptions published.
- *A date on the clock.* A date depends on hardware roadmaps, not on players, and the press will read it as a forecast. The hands should show **cost-to-break in logical-qubit-days**. A date appears only behind a tap, with roadmap assumptions stated.

**Speedrun framing kept.** Marginal contribution, with and without a tactic, is the right metric, and leave-one-out handles overlapping tactics. AlphaTensor-Quantum's training stack may not be public, so the reference bot is open: PyZX plus a Monte Carlo tree search (MCTS) we ship. Transfer families stay sealed and rotate each season so they can't leak.

**Abuse.** Bots submitting as humans: let them, on a separate tool-assisted board. Rewrites are sound, so only attribution can be faked.

**Ethics.** A leaderboard that makes breaking Bitcoin cheaper is dual-use. But attack-side estimates are already published openly, and a credible clock speeds up migration. Results go out as papers co-authored with post-quantum migration teams.

**Money.** DARPA's Quantum Benchmarking Initiative, hardware credits, and post-quantum bounty pools from crypto foundations. The tactic book feeds Quantum in Pictures classrooms.

**90 days.** Weeks 1–4: !-box replay engine, 10 adder and multiplier families, Qualtran scoring. Weeks 5–8: closed beta with 500 ZX-literate players. Weeks 9–12: first sealed transfer test, plus one Helios calibration run.
**Kill metric:** no tactic shows a positive marginal contribution on a sealed family.

### Turn 5 — Visionary
**Final form.** Minute Hand is a ZX-calculus puzzle game in which the verb is *fuse*, and what players produce is **tactics**, not diagrams. Levels are parametric families from published fault-tolerant estimates: ECC arithmetic, QROM, FeMoco and P450 kernels. You solve at n = 3–5 on a gflow-preserving board. The server replays your rewrite path at n = 256 with !-boxes, adds it as a macro-action to an open PyZX+MCTS bot, and scores you on the **marginal spacetime volume** the bot saves with your tactic compared with without it, using Qualtran with published assumptions. Named tactics enter an opening book. A tactic is promoted to *primitive* only after it saves volume on sealed families it never saw. Two hands, Q-Day and Useful Day, show cost in logical-qubit-days, with a date one tap away. Once a month, Helios checks whether the cost model holds on logical hardware. I accept every kill: hardware as calibration beats hardware as trophy.

**Moonshot (2036).** The tactic book *is* the compiler. Every rewrite is sound, and ZX is complete, so each tactic with an !-box is a machine-checkable theorem. The book exports to Lean as a verified rewrite library, and fault-tolerant toolchains ship it as an optimisation pass with player provenance in the changelog. The big prize is a cross-family primitive with a better *asymptotic* cost, a Karatsuba of fault-tolerant arithmetic found by a fourteen-year-old. Migration deadlines and chemistry funding calls cite the clock as public infrastructure.

**Scores.** Innovation **8**. ZXLive, PyZX and Foldit exist. What's new is crowd discovery of proof *strategies*, scored by their measured marginal value to a search bot, feeding a civic clock. Feasibility **5**. The stack is open and buildable now. Whether people have a real edge at all is a coin-flip, and the kill metric checks it.

### Turn 6 — Builder
Innovation 8 holds, though Foldit and Quantum Moves already proved the crowd-science format. What earns the wild score is scoring human strategies by their measured marginal value to an open search bot, and requiring a tactic to transfer to sealed families before it counts as a discovery. I'm cutting feasibility to 4. Lifting a rewrite path from n = 3–5 to an !-box tactic that holds at n = 256 is program synthesis, not replay, and that engine is research-grade before any player arrives. The kill metric is honest either way: if humans add nothing, the game still ships an open benchmark and a clock nobody else maintains.
