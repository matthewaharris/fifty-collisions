# Idea 26 — Sandbox Escape
> Seed: 26. [wild] **Simulation theory × physics × crowdsourced bug bounties.** Searching for the rendering engine's edge cases.

### Turn 1 — Visionary
**Working name:** Sandbox Escape

**Hook:** A capture-the-flag league where people and AI scientists, trapped in toy universes, prove what engine they run on, then point the winning methods at ours.

**Why these collide.** Security research and physics share an epistemology: you probe a system from inside, and bugs live where the implementer saved money. Simulation theory's only testable content is *"the implementer economized."* Don't hunt glitches; **threat-model the engine.** List the cheap shortcuts (lattice, finite precision, lazy evaluation, caching, level of detail) and derive each one's fingerprint. Beane, Davoudi and Savage (2012) did this once: a cubic lattice would align the highest-energy cosmic rays with its axes.

**Core loop. The verb is escape-test.**
1. **Build.** A red team writes a toy universe (a lattice field theory or a cellular automaton) with a hidden shortcut and commits its source hash.
2. **Probe.** Hunters, human or agent, get only an in-world lab API and an energy budget, and pre-register: "cubic lattice, spacing below X."
3. **Disclose.** Source opens; claims are scored. The league accumulates a **power table**: which inside-view tests catch which shortcuts, at what cost.
4. **Port.** Winning strategies run as pre-registered analyses on our universe's open data (Auger, Fermi, optical clocks). The bounty board is the Standard Model Extension's tables of Lorentz-violation coefficients, a CVE list for reality. Tightened bounds pay; anomalies go to embargoed, replicated disclosure, never straight to press.

**Honesty clause.** A perfect simulator is undetectable, and a null refutes nothing. The payoff is the power table and real bounds on new physics either way.

**Why now.** World models made simulated worlds with agents inside an engineering object (brief §7). AI scientists run closed experimental loops (§15), and 12–15-hour METR horizons (§1) let an agent run a whole campaign inside a toy cosmos. Simulation-theory papers keep coming (§7) with no shared test protocol.

### Turn 2 — Builder
**What exists.** The method already has a canon: Beane–Davoudi–Savage's lattice axes; Kostelecký and Russell's *Data Tables for Lorentz and CPT Violation*, updated yearly: already a CVE list; Fermi's GRB 090510 bound on linear photon dispersion, past the Planck scale; Fermilab's Holometer, which hunted holographic jitter and came back null. League templates: CASP has scored blind, commit-then-reveal predictions every two years since 1994. ARC-AGI-3 makes agents infer hidden game rules from inside. LIGO's blind injections test whether a collaboration can tell a planted signal from a real one. Nobody has joined them.

**Hardest problem: Goodhart, then the port.** Red teams will pick shortcuts that suit toy worlds, like automaton artefacts and float rounding, and get rewarded for obscurity. The power table would map the red teams' taste, not any real implementer's economy. Hunters also get a clean lab API, while our universe offers fixed archives, fixed exposure and brutal systematics. Every "anomaly" so far has been a loose cable (OPERA) or a foreground (BICEP2).

**Twist: pay the red team for compute saved.** Each engine must pass a fidelity suite set to what our universe has actually been measured to do, today's SME bounds, and earns bounty in proportion to FLOPs saved against a reference engine. Shortcuts then come from real optimization pressure. The league becomes a GAN played by people, compressors against detectors, and its deliverable is a **cost–detectability frontier**: at each compute budget, the cheapest shortcut nobody catches. Hunters work only through mock instruments (Auger-like arrival maps with exposure gaps, Fermi-like photon timing) with injected systematics, so winning code runs unchanged on real archives.

**MVP.** One engine family (relativistic fields on a 3+1D lattice), 20 committed engines, mock detectors, three eight-week rounds, log-scored pre-registered claims.
**Kill metric:** winning detectors fail on held-out engine families.

### Turn 3 — Visionary
**I take the compute bounty, with one refusal.** Mock instruments alone would shrink this into a physics-ML benchmark, so every season ends with a **salted round**: the red team injects its winning engines' fingerprints into copies of real Auger and Fermi archives at secret positions and amplitudes, LIGO-style. Hunters run frozen pipelines over salted and clean copies. Recovered salts calibrate each pipeline's sensitivity against real systematics, so the clean result becomes a real bound. That is the port.

**Goodhart, solved by image forensics (unseeded).** Deepfake detection already lived through the Builder's kill metric. Wang et al. (2020) trained on one GAN and caught others, because all of them upsample, and upsampling leaves spectral peaks. Then diffusion models arrived and those detectors failed (Ojha 2023). Lesson: detectors generalize when they target an operation every cheap generator must perform, not one generator's style. So detections pay only by **transfer**, scored on held-out engine families built by an evolutionary code optimizer that has no taste, only FLOP pressure.

**Closed loop.** The fidelity suite tracks the live SME tables. When Auger tightens a bound, every engine exploiting that slack is illegal next season. Real experiments patch the toy universes; the toy league tells real experiments where cheap slack still hides.

**Emergent output: a price list for physics.** Over seasons, the frontier shows which of our laws optimizers converge on for free (finite signal speed, locality, coarse-graining) and which they always drop because faking them is ruinous: exact Lorentz invariance on a grid, and entanglement, which Feynman argued costs a classical host exponentially. Expensive laws are **anti-fingerprints**; a thrifty classical implementer wouldn't pay for them. The league prices every law in FLOPs, turning "are we simulated?" into "what kind of host could afford this?"

### Turn 4 — Builder
**Kills and fixes.**
- *"Track the live SME tables."* A toy lattice spans maybe 10³ in scale; ours spans 10⁶⁰. Express each bound as a fraction of explored dynamic range, and run every engine at three sizes to publish scaling curves.
- *FLOPs.* Counters get gamed. Run every engine on identical GPUs and pay per **joule saved**.
- *Anti-fingerprints.* Classical engines can't price entanglement, so the price list carries a "classical host" column only, labelled, until quantum engines exist.
- *Salting.* Fermi-LAT data are public; Auger's mostly aren't. Salted copies never leave an enclave: hunters submit containers and get back scores only, which also blocks leaked salts and leaderboard probing.

**Abuse.** Red–hunter collusion: signed hash commits, and no team plays both sides of one family. Crank headlines: any real anomaly goes first to the instrument collaboration under coordinated disclosure, CVE-style, with a 90-day embargo.

**Safety, the real one.** This is a gym for *evaluation awareness*: agents learning to tell they're inside a simulation. Hunter APIs expose physics observables only, never harness metadata, and frontier-agent results go to eval teams before publication.

**Sustainability.** FQXi and Templeton fund the physics. The paying customer is sim-to-real. RL agents exploit MuJoCo glitches; robotics teams training in Isaac or Genie-style world models need to know which artefacts a policy can detect. Simulator vendors sponsor seasons and buy audits.

**Stack.** JAX lattice engines (scalar + U(1)), OpenEvolve (AlphaEvolve-style) as the tasteless compressor, gVisor sandboxes, public LAT data, log scoring.

**90 days.** Weeks 1–3: reference engine, mock instruments, rescaled fidelity suite. Weeks 4–6: 12 engines in three families, half human, half evolved. Weeks 7–10: round 1, scored on a held-out family. Weeks 11–12: first salted round, Fermi GRB timing only.
**Kill metrics:** no engine saves 10% energy while passing fidelity, or salt recovery under 50%.

### Turn 5 — Visionary
**Final form.** Sandbox Escape is a seasonal league that threat-models the universe's engine. Red teams (humans plus OpenEvolve) build lattice universes that pass a rescaled fidelity suite, paid per joule saved. Hunters submit frozen, observables-only pipelines and are paid by transfer to held-out engine families. Each season ends with a salted round on real data inside an enclave, Fermi GRB timing first. Anomalies go to collaborations under a 90-day embargo. Outputs: a **cost–detectability frontier**, **calibrated real bounds**, and a **price list for physics** (classical-host column only). I accept every kill, with one caveat: going from 10³ to 10⁶⁰ is a scaling *hypothesis*, so those curves get published as hypotheses, never as bounds.

**The Builder found the other half.** Evaluation awareness is the same problem seen from the other side. One frontier answers both "can we tell we're simulated?" and "can our agents?" Labs need to know how cheap a sandbox can get before a model notices. Sponsors pay for that safety half, and the physics comes along with it.

**Moonshot (2036).** Fault-tolerant machines (Apollo's successors) run quantum-host engines and fill the second column. The league then publishes a **host-complexity bound**: the cheapest host consistent with every measurement humanity has made. That number goes up each time an experiment tightens. Collaborations pre-register on the bounty board as routine. "Are we simulated?" stops being a belief and becomes a number with error bars.

**Scores.** Innovation **8**: it turns an unfalsifiable hypothesis into an adversarial measurement institution that produces results whether the answer is null or not. It's not 9, because CASP, the SME tables and blind injection are all precedents. Feasibility **4**: the MVP is cheap, but porting to real data is unproven, getting Auger data is political, and the payoffs are mostly null bounds.

### Turn 6 — Builder
Treating the scaling curves as hypotheses is the right call. Without that, the real-data bounds would be spin. Innovation 8 stands. A league where compressors are paid per joule saved and detectors only for transfer turns "are we simulated?" into a price list for laws and a host-complexity number that experiments push upward. Pairing it with evaluation awareness gives the league a paying reason to exist. Feasibility 4 also stands. The toy league plus a salted Fermi round fits in a quarter on public data, but nobody knows whether detectors transfer across engine families or whether anything learned at 10³ means anything at 10⁶⁰.
