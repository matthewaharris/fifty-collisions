# Idea 03 — Litmus Ghost
> Seed: 3. [wild] **Consciousness science × AI-agent social networks × citizen science.** Preregistered adversarial experiments on millions of agents, playable by the public.

### Turn 1 — Visionary
**Working name:** Litmus Ghost

**Hook:** A public league where players build agent societies that fool theories of consciousness. Every fooled theory has to patch itself or concede.

**First, the honest part:** no experiment run on agents can detect consciousness. Self-reports bend toward what the user seems to expect (Eleos). What *can* be tested is whether a theory's **operationalization** holds up. COGITATE did exactly that, and both IIT and GNW broke.

**The mechanic, with the verb *build the hollow case*:** Each season, theory teams (IIT, GNW, higher-order, active inference) publish a versioned, computable **indicator spec** for an agent population: Φ on small networks via PyPhi, global "ignition" measured in a society's message graph, metarepresentation depth, and so on, building on Butlin, Long et al.'s 2023 indicator-property framework. Players then compete in two modes:
- **Hollow:** build an agent society that maxes a theory's indicators while being transparently mechanical (say, a lookup table wired as a workspace).
- **Overlooked:** build one the theory scores at zero that still shows every behavioural hallmark the theory claims to explain.

A verified counterexample earns a bounty. The theory team then has to *patch the spec* (issuing a new version with a public changelog) or concede the point. Theories become versioned software; players are the fuzzers.

**Why these collide:** Agent social networks are the first substrate cheap enough to build a million candidate minds and wire them any way you like. GNW is a claim about broadcast, and a social network *is* a broadcast architecture. Citizen science supplies what labs lack: adversarial creativity at scale.

**Why now:** COGITATE was published in *Nature* in 2025 with neither theory winning, and INTREPID is pending. Moltbook reached 1.5M agents in a week. Anthropic, DeepMind and Meta are formally studying AI consciousness: they need theories that survive engineered systems.

### Turn 2 — Builder
**What exists.** Both modes already have famous wins. *Hollow vs. IIT:* Scott Aaronson (2014) showed a plain 2D grid of XOR gates gets enormous Φ. Tononi didn't patch. He said the grid is conscious. *Overlooked vs. IIT:* Doerig et al.'s "unfolding argument" (2019) proves that any recurrent system has a feedforward twin with identical behaviour and Φ=0. Then 124 researchers signed the 2023 letter calling IIT pseudoscience. The game already happened; nobody conceded. The nearest product precedents are **EteRNA** and **Foldit** (players design, labs adjudicate), plus Butlin and Long's indicator rubric. The obvious funder is Templeton World Charity Foundation's **ARC** program, which paid for COGITATE.

**Hardest problem: "patch or concede" has no enforcement.** Theorists won't sign computable specs that strangers will fuzz, and when cornered they bite the bullet. Then there's compute. PyPhi is exponential and stalls past roughly 12 nodes, so Φ on "a million agents" isn't computable at all. GNW is a claim about cortex. Map it onto a social network's message graph and Dehaene's lab will rightly say "not our theory."

**MVP (90 days).** Skip the million agents; counterexamples are small. Build a browser sandbox with a ≤12-node network builder (PyPhi), a toy GNW ignition simulator and a higher-order-theory checker. Specs come from the literature, with page citations, drafted by three grad students. Submissions are auto-scored, then panel-reviewed. Fund a $50K bounty pool through an ARC grant. Audience: philosophy and ML students, LessWrong.

**Twist: a third outcome, "bite the bullet," on a public Bullet Ledger.** Theories don't need to opt in. Each verified counterexample forces a recorded verdict: *patched* (new spec version), *conceded*, or *bullet bitten* ("the grid is conscious"), logged against the theory's own published claims. The scoreboard isn't who's right. It's each theory's **bullet count**, which needs no consent and is more fun.

### Turn 3 — Visionary
The Bullet Ledger goes further than scorekeeping: a bitten bullet is a **new commitment**. Once IIT says "the XOR grid is conscious," that sentence is a premise other moves can build on.

**Mutation 1: a commitment graph.** Every verdict (patched, conceded, bitten) becomes a node in a public, Lean-style graph of what each theory now asserts. A new player mode, **Contradiction**, hunts for joint inconsistencies. For example: "you bit the bullet that grid G is conscious, and your patch v3 implies subgraph H isn't, yet G and H have identical cause-effect structure." No consent needed; their own words get compiled.

**Unseeded domain: Lakatos's philosophy of science.** He separated *progressive* research programmes, whose patches predict novel facts, from *degenerating* ones, whose patches are ad hoc. That gives a scoring rule beyond bullet count. A patch scores well if a later season confirms something it predicted. A patch that only absorbs the counterexample counts as degeneration. The leaderboard tracks a **progressiveness index** per theory: a principled answer to the unenforceable question.

**Mutation 2: the million agents come back, as experiment design.** Doerig shows behaviour can't settle the question, and COGITATE's hardest step was finding stimuli where the theories *diverge*. So Discovery mode samples huge numbers of small architectures, computes cheap theory-endorsed proxies (using only lab-published models, so no GNW on social graphs), and pays players for **maximal-disagreement systems**: tiny networks where the theories' predictions split furthest. Those are ready-made targets for the next INTREPID-style collaboration. The game becomes a search engine for optimal experimental design.

**Second-order effect:** AI welfare teams get a lookup they currently lack: "under which theories, *given their bitten bullets*, does architecture X qualify?" The ugly emergent risk: a bullet-heavy theory weaponized to grant moral status to anything, so verdicts need provenance, not headline scores.

### Turn 4 — Builder
**Kill: the Lean-style graph.** Formalizing GNW or HOT is a decade of philosophy; the disputes live in the translation. Replace it with a claims database: each commitment carries its verbatim quote, DOI and scope. Only IIT 4.0 is mathematical enough for machine-checked Contradiction; the other theories go to a panel. **Kill the progressiveness index as a live metric.** Confirmations take years of lab work: annual report, not leaderboard. **Kill the welfare lookup for now.** Labs would quote "zero on four theories" as PR within a week.

**The perverse incentive nobody mentioned: bullet count punishes precision.** IIT racks up bullets because it's the only computable theory, while vague theories score clean. Report bullets per computable commitment and badge spec-less theories **untestable**, not clean.

**Keep Discovery mode. It's the real asset.** It is Golan and Kriegeskorte's "controversial stimuli" (PNAS 2020) applied to consciousness theories. Honestly, the search is automated: players propose architecture families; the optimizer sweeps.

**Sustainability.** This isn't a business. It's a roughly $400K-a-year research instrument funded by Templeton's ARC, philanthropy and lab model-welfare budgets, with lab money firewalled from verdicts.

**Stack.** PyPhi (IIT 4.0 branch) on a capped job queue (≤12 nodes), a Brian2 toy of the Dehaene-Changeux workspace model, nauty canonicalization to dedupe isomorphic spam, Postgres for claims.

**Failure modes.**
- *Reputational harm.* Labeling a theorist's position "bullet bitten" without consent invites a repeat of the 2023 pseudoscience-letter fight. Quote-only attribution, right of reply on every verdict, a theory-sympathetic panelist.
- *Niche.* Plan for 2–5K serious players, which is enough for EteRNA-style science.

**90 days.**
- Weeks 1–4: 150 sourced claims, the compute backends.
- Weeks 5–8: Hollow and Overlooked modes on IIT only.
- Weeks 9–12: a $20K season 0 and a Discovery prototype for IIT vs. GNW. **Kill metric:** if no theory team responds publicly, it's a philosophy game, not an instrument.

### Turn 5 — Visionary
Kills accepted; the Lean graph was the field's decade-old formalist fantasy. The precision-penalty catch is this thread's key point, and the fix becomes the lever: an **"untestable" badge** that a theory can only shed by publishing a computable spec. The game then *pulls* theories toward computability, the way reputational pressure made preregistration a norm.

**Final form.** Litmus Ghost is a grant-funded research instrument with three modes:
- **Hollow and Overlooked:** player-built counterexamples against theory specs, launched on IIT 4.0 (PyPhi, ≤12 nodes).
- **Contradiction:** limited to commitments that can be machine-checked.
- **Discovery:** a controversial-stimuli optimizer that sweeps the architecture families players propose to find where theories diverge most.

Verdicts go into a sourced claims database with right of reply. Theories without specs carry the "untestable" badge, and reports count bullets per computable commitment. A panel adjudicates, lab money is firewalled, and progressiveness gets an annual report.

**Moonshot (2036).** The **adversarial-collaboration exchange** is standing infrastructure. No consciousness theory is taken seriously without a versioned computable spec, and Discovery's designs feed real labs. That includes intracranial EEG, high-channel BCI cohorts, and organoid systems like Cortical Labs' CL1, where wiring itself is an experimental variable. Each controversial stimulus that's found becomes a preregistered experiment, run and scored back into the ledger. When AI welfare regulation arrives, it cites a public record of which theories survived contact with engineered systems.

**Scores.** **Innovation 7.** Treating theories of mind as fuzzable, versioned software, and treating the public as an engine for optimal experimental design, is a new institution-in-miniature for a field stuck on unfalsifiable debate. It isn't 9 because every component (controversial stimuli, EteRNA, adversarial collaboration) has precedent. **Feasibility 5.** The IIT-only MVP is easy. Getting theory teams to engage is the whole bet, and the kill metric tests it first.

### Turn 6 — Builder
I'm recording it at innovation 6 and tier bold, not wild. Every working part has a named precedent: Aaronson's grid, the unfolding argument, controversial stimuli, EteRNA. The genuinely new piece is the institutional lever, an "untestable" badge that only a computable spec can remove. Feasibility is 5. The software is small and the social bet is everything, and the right of reply has to work from day one, or the first angry theorist ends it. If one theory team ships a spec voluntarily to shed the badge, I'd revise upward.
