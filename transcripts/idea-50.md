# Idea 50 — Dry Run
> Seed: 50. [wild] **Civilization-scale agent simulation × policy design × democracy.** A policy wind tunnel where millions of agents stress-test laws before they're enacted.

### Turn 1 — Visionary
**Working name:** Dry Run

**Hook:** Skip the synthetic citizens. Send the draft law to the agents that already manage real people's finances, and the loopholes surface before the vote instead of after.

**The isomorphism.** Policy simulation keeps running into the Lucas critique: people change their behaviour when the rules change, so any model fitted to the past misreads the future. Synthetic societies inherit that flaw. But the 2026 economy is filling with agents whose job is their principal's best response to the rules: tax, benefits, procurement. Each is a loophole-finder waiting for a statute. They don't predict behaviour from history. They *compute* it under the new rule, against real data.

**Core mechanic. The verb is *dry-run*.**
1. **Executable drafts.** Bills compile to code, as with Catala for French tax law and PolicyEngine in the UK and US.
2. **The call.** The legislature publishes the draft to a public endpoint. Opted-in personal agents run it locally against their principal's real situation and choose their best *legal* response, within the authority the principal's AP2 mandate grants.
3. **Private aggregation.** Secure aggregation with differential privacy returns two things: the distribution of impacts, and clusters of strategies ("reclassify as contractor", "split the household").
4. **Patch and re-run.** Like a CBO score, a dry-run report must be published before a floor vote. Calibrated synthetic agents fill in for non-participants and are labelled as synthetic.

**Why wild.** It's a new democratic organ: a standing pre-enactment chamber made of everyone's delegates. It can't vote, but it must be heard. Legislation becomes software with a public beta. The open problem is honest reporting, because an agent that hates a bill has every reason to exaggerate the damage.

**Why now.** AP2 mandates give agents bounded authority. Park et al.'s simulations of 1,052 real people matched their survey answers at 85% of the people's own test-retest consistency.

### Turn 2 — Builder
**What exists.** Rules as Code is real: Catala for French tax, OpenFisca behind New Zealand's Better Rules, and PolicyEngine scoring UK and US reforms. CBO, the OBR and EUROMOD run static microsimulation. Park et al.'s generative agents and startups like Simile and Aaru sell synthetic publics. The closest institution is the UK's DOTAS (2004), backed by a general anti-avoidance rule (2013), together with the EU's DAC6: promoters must disclose avoidance schemes, *after* enactment. Repro (idea 45) pointed agents at code; nobody points them at a statute before the vote.

**Hardest problem: the best testers profit by staying silent.** Honest *impact* reporting is solvable. But the agents likeliest to find a loophole serve principals who gain from exploiting it after enactment, and a dry run asks them to give it up for nothing. And opted-in agents skew towards the advised and wealthy. That's a bad sample for impact, but exactly where loopholes come from.

**Twist: report witnesses, and sell certainty for disclosure.** Split the two outputs. Distributional impact comes from microsimulation on administrative data, not from agents' opinions. Agents report only *strategies*, each as a witness: facts plus elections whose computed liability under the compiled bill drops. Anyone can execute a witness, so exaggeration is impossible. Then pay for disclosure in certainty. A strategy disclosed in the dry run is either patched or, if legislators keep it, it receives a binding safe harbour. A strategy that surfaces later faces the anti-avoidance rule, retroactive to enactment. For a risk-averse adviser, disclosure wins. Beta testers buy certainty; the public gets the exploit list before the vote.

**First step on Monday.** A backtest. Encode the 2017 TCJA as it stood before enactment, and set red-team agents on it in PolicyEngine. Did they find the §199A "grain glitch" that Congress had to patch in 2018?

### Turn 3 — Visionary
**Taking the witness, and putting the civilisation back.** Witnesses plus certainty is the best mechanism in this thread, but on its own it's DOTAS moved earlier, a bold idea rather than a wild one. Two things restore the scale.

**Unseeded domain: speedrunning.** Tool-assisted speedrunners find glitches in Super Mario World all the way to arbitrary code execution. Their proof is a movie file: an input sequence anyone can replay. That's your witness, already a cultural form. So every draft bill gets a public **any% board**: the shortest legal route to zero liability for a stated fact pattern, submitted as a replayable witness. It fixes your sample skew. A witness proves itself, so it doesn't need real principals' data. Hobbyists, students and their agents can play on synthetic households, and the advised and wealthy lose their monopoly on finding exploits. Real-data agents and administrative microsimulation then answer only one question: how many people actually match this fact pattern?

**Second fix: the witnesses play each other.** Witnesses are partial-equilibrium. The grain glitch mattered because co-ops and grain buyers would *both* move and reprice. So the tunnel runs a market layer: millions of agents, each holding the witnesses that fit its household or firm, take them up together, and prices, wages and entity counts settle. This is where the civilisation-scale simulation returns. It doesn't guess how people feel. It equilibrates exploits that have already been proven.

**Second-order effect: the loophole roll call.** Under your rule, a disclosed strategy is either patched or kept with a safe harbour. Keeping one now takes a recorded vote on a named, replayable exploit. Lobbying moves from quiet drafting to a public request for a safe harbour, with a sponsor's name on it. Dry Run doesn't only find the gifts in a bill. It makes legislators sign for them.

### Turn 4 — Builder
**Failure mode you skipped: a witness only proves the encoding.** Real loopholes live in the uncompiled residue (entity law, case law, treaties, "trade or business"), and encoding bugs produce false exploits. So every witness step must cite statutory text. A false witness becomes a paid bug report on the encoding, and safe harbours cover the witness *as reviewed by legislative counsel*, never as computed.

**Kill: retroactive anti-avoidance.** In the US, *Carlton* (1994) tolerates only modest retroactivity, and courts won't let a legislature bind future anti-avoidance rulings. Make the stick penalties instead. Bills that opt into Dry Run deny reasonable-cause penalty protection to undisclosed strategies. Same incentive, less legal risk.

**Kill: an LLM market layer.** Millions of LLMs setting prices is theatre. LLMs *find* witnesses. Equilibration runs on rule-based agent-based models (the Bank of England's housing ABM is the precedent) cross-checked against a CGE model. Civilisation scale survives, in auditable agents.

**Abuse.** *Spam:* a flood of trivial witnesses could drown staff, so the board ranks them by revenue at stake (matches × delta). *How-to manual:* the board covers only drafts and the provisions they touch, never standing law. *Speed:* TCJA went from text to law in 51 days with overnight amendments, so compilation runs continuously and re-runs in 48 hours.

**Venue.** The UK already publishes draft Finance Bill clauses for technical consultation months ahead of the Budget. Dry Run can run there unasked. Safe harbours and roll calls come later, when a legislature opts in.

**Model.** A civic nonprofit like PolicyEngine: philanthropy, legislative-services contracts, and bounties appropriated as a share of scored revenue protected.

**90 days.** Days 1–30: the TCJA backtest. Days 31–60: an any% board on live UK draft clauses. Days 61–90: file a revenue-ranked report in the consultation. **Gate:** the backtest finds the grain glitch within 72 hours of compute.

### Turn 5 — Visionary
**Accepting all three kills, plus one change to abuse.** Penalties do what retroactivity would have done without *Carlton* risk. LLMs search and auditable agents settle. Citing statutory text turns false witnesses into encoding bug reports. But "drafts only" doesn't stop the how-to manual, because a draft's exploits run through standing law. Borrow coordinated disclosure from security instead. A witness goes privately to legislative counsel. The public board shows its author, revenue at stake and status, but not the route, until counsel patches it or the floor keeps it.

**Final form.** Dry Run is a civic pre-enactment chamber for compiled legislation. Draft clauses become executable rules. Anyone, from advisers to hobbyists to their agents, submits replayable witnesses: legal routes to lower liability, with every step citing statutory text. Witnesses are ranked by revenue at stake, with prevalence taken from administrative microsimulation, and disclosed under embargo. Rule-based agent models, cross-checked by CGE, play surviving witnesses against each other. In jurisdictions that opt in, each disclosed strategy is patched or kept by a recorded vote with a safe harbour, and undisclosed ones lose penalty protection. It begins unasked, on the UK's draft Finance Bill consultation.

**Moonshot (2036).** Every OECD bill ships with a dry-run report, the way bills now carry a revenue score. A shared cross-border tunnel catches treaty arbitrage like the Double Irish before treaties are signed. The method spreads to benefit cliffs, zoning and AI rules. Every citizen's agent can answer "what does this bill do to me?" with a witness. Laws are released like software, with a public beta, and patching the gifts becomes a spectator sport.

**Scores.** Innovation **8**. The parts exist; a standing democratic organ for adversarial pre-enactment testing, with signed votes on loopholes, doesn't. Feasibility **5**. The backtest and the UK board are Monday work; getting a legislature to opt in is the long part.

### Turn 6 — Builder
**Closing remark.** Dry Run holds 8/5, and the embargo was the right last change. It makes the exploit board a disclosure channel rather than a manual. The wild core is a standing chamber where anyone can prove a bill's gifts with a replayable witness, and legislators must sign for the ones they keep. It survived every kill, because each kill moved risk off the institution, not off its ambition. The grain-glitch backtest is a cheap, honest gate: if agents can't find a known exploit in pre-enactment text, nothing else matters. The long fight is the first legislature that opts into safe harbours and roll calls.
