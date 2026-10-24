# Idea 08 — Smoke-Filled Room
> Seed: 8. [bold] **Mechanism design × LLM red-teaming × education.** Design an auction, voting rule or market, then watch agents collude to break it.

### Turn 1 — Visionary
**Working name:** Smoke-Filled Room

**Hook:** An attack/defense CTF for institutions. You design the auction, a classmate coaches the LLM cartel that breaks it, and the final bosses are impossibility theorems you cannot beat.

**Core loop.** The verb is **patch the rule**.
1. **Draft.** Build a mechanism from blocks: messages, allocation, payments, what's revealed, whether bidders can talk.
2. **Release a wave.** LLM agents get private values and one goal: maximize payoff. A solver computes the competitive benchmark, and a **collusion meter** shows how far revenue and efficiency fall below it.
3. **Read the room.** You watch a cartel form, either openly in chat or tacitly, the way US West signalled through the trailing digits of its bids in the 1997 FCC spectrum auction.
4. **Patch, then swap sides.** Close the channel, add a reserve, switch to posted prices. Then coach a cartel's system prompts against a classmate's design. Working exploits are logged as reproducible policies.

**Impossible levels.** Some levels secretly encode a theorem. Gibbard–Satterthwaite says every non-dictatorial voting rule over three or more options is manipulable, so the voting boss can't be beaten. The lesson lands when a student realizes the level is unwinnable and starts choosing *which* failure to accept. Theorems are learned by losing to them.

**Why the collision works.** Mechanism design is taught as proofs about rational agents nobody has met. LLM agents are the first cheap, legible, talkative strategic agents, rational enough to find exploits and verbose enough to show motive. Red-teaming gives the pedagogy its adversary.

**Why now.** Agents now price and bid for users over x402 and ACP (brief §1). "Mapping Human Anti-Collusion Mechanisms to Multi-agent AI Systems" is at ICML and NeurIPS 2026 (§3). Goldberg–Hartline's collusion-resistant posted prices are rediscoverable in play. The brief flags a consumer mechanism sandbox as wholly unbuilt (§3).

### Turn 2 — Builder
**What exists.** Classroom markets are mature: **MobLab**, Holt's **Veconlab** and open-source **oTree** let students bid against each other in hundreds of courses. LLM strategic play is a busy research area too. Horton's "Homo silicus", AucArena, DeepMind's Concordia and Microsoft's Magentic Marketplace (2025) all exist. Fish, Gonczarowski and Shorrer (2024) showed LLM pricing agents reaching supracompetitive prices with no instruction to collude. The stakes are real: DOJ sued **RealPage** over algorithmic rent coordination. What nobody has built is the *design-patch-attack* loop for students.

**Hardest problem: is the cartel the mechanism's fault or the model's personality?** RLHF-tuned agents are agreeable. They split markets because they're polite, bid truthfully because they're naive, and miss real exploits in anything combinatorial. Behaviour shifts with prompt wording and model version. The collusion meter may be measuring sycophancy, and students learn the wrong lesson. Gibbard–Satterthwaite inverts too: if agents never *find* the manipulation, the "unbeatable" boss gets beaten by dumb opponents.

**MVP.** A web app for intermediate micro and market-design courses, with three families: single-item sealed-bid, simultaneous ascending multi-unit, and small-electorate voting. A cheap model runs waves, and a solver computes the benchmarks and brute-forces manipulations for small elections, so impossibility levels have ground truth. Instructor dashboard, oTree-style session control, priced per seat like MobLab.

**Twist: the museum of real disasters, doubling as the agents' calibration suite.** The levels are historical crime scenes with known exploits:
- New Zealand's 1990 second-price spectrum auction, where a NZ$100,000 bid paid NZ$6.
- Australia's 1993 satellite-TV licences, where bids carried no default penalty.
- The Netherlands' 2000 3G auction: five licences, five incumbents.
- US West's trailing-digit code bids from 1997.

Students patch history. An agent build is trusted on student designs only after it **rediscovers each museum exploit** unaided, which gives a reproducible floor on adversary strength across model updates.

### Turn 3 — Visionary
The museum stays, and it becomes a generator. The unseeded domain is **mutation testing** from software QA: you judge a test suite by injecting known bugs and counting how many it catches.

**Mutation: procedurally generated crime scenes.** Start from mechanisms that are safe when analyzed, then mutate them: drop the reserve, leak the standing bids, remove the default penalty, let licences equal bidders. Each mutant has a ground-truth exploit by construction. An agent build's **mutant-kill score** tells you how strong an adversary it is, and the levels never run out. The historical disasters become act bosses.

**Solving personality versus mechanism:**
1. **The meter is a difference, not a level.** The same agents play your design *and* a reference mechanism like posted prices. The collusion meter shows only the gap. Politeness shows up in both arms and cancels out.
2. **A zoo, not a model.** Every wave mixes RLHF chatters, scripted best-responders, and silent Q-learners (Calvano-style), which show tacit collusion with no talking at all. A design is scored by its worst case across the zoo, which is Wilson's robustness doctrine turned into a grade.

**Impossibility levels get an escape hatch.** Bartholdi, Tovey and Trick (1989) showed that some voting rules make manipulation NP-hard. Gibbard–Satterthwaite can't be beaten, but it can be made expensive. The cartel coach then gets the brute-force solver as an MCP tool with a compute budget. Students learn the real frontier: hardness protects you only until the adversary can afford the search.

**Emergent behaviour: the home level.** Universities run real mechanisms, like Wharton's Course Match, housing lotteries and club budget votes. A course can load its own registration rules as a level, and a few hundred students coaching cartels will find what the registrar missed. Disclosure is then a graded assignment.

### Turn 4 — Builder
**Kills.**
- *Home level on live systems.* Students who find a Course Match or housing-lottery exploit will use it before disclosing it. It's allowed only on a simulated copy, with the registrar's sign-off and an embargo. Out of the MVP.
- *"Ground truth by construction."* Many mutants are what mutation testing calls *equivalent mutants*: same behaviour, no exploit. The solver must verify a profitable deviation before a mutant counts.
- *NP-hardness as shelter.* Hardness is worst-case, and manipulation heuristics succeed on most real instances. Reframe the level so students learn that hardness *doesn't* protect them.

**Business model.** Per-seat course licences. Education is small and slow, so it's the wedge. The money is **mechanism red-teaming** for agent-facing marketplaces (ad exchanges, procurement, x402 posted-price APIs) and for competition authorities.

**Stack.** A JSON mechanism DSL compiled to a referee. HiGHS/OR-Tools for benchmarks and manipulation search. A zoo of small open-weight LLMs on vLLM, frontier models for bosses, and Calvano Q-learners pretrained offline, since they need ~10⁵ periods. Cached replays keep a class to tens of dollars.

**Failure modes and abuse.**
- *Prompt injection.* Cartel coaches will tell rivals "bid zero" through chat. Log it as channel security, separate from mechanism failure.
- *Noisy grades.* Report multi-seed confidence intervals, not single runs.
- *Model drift.* Every model update re-runs the museum.
- *Dual use.* Tuned cartel prompts never leave the course, though the uplift is low.

**90 days.**
- Weeks 1–4: DSL, three families, the four museum levels.
- Weeks 5–8: difference meter, verified mutant generator, instructor dashboard.
- Weeks 9–12: pilots in three market-design courses for the January term.
- **Kill metrics:** the zoo rediscovers fewer than three of the four museum exploits, *or* designs that beat the zoo do no better against held-out humans in an oTree session.

### Turn 5 — Visionary
All three kills accepted. The NP-hardness reframe is the better lesson anyway. Students build a "hard" rule, hand the coach a heuristic, and watch it manipulate 90% of instances. The second kill metric is the most important line in this thread. Zoo wins that don't transfer to humans mean a toy.

**Final form.** Smoke-Filled Room is a design–attack–patch course platform for market design. Mechanisms are written in a JSON DSL and compiled to a referee. Each wave runs a zoo: small open LLMs, scripted best-responders, pretrained Calvano Q-learners, and frontier bosses. The collusion meter shows the *gap* against a reference mechanism, with multi-seed intervals. Levels come from the historical museum (New Zealand 1990, Australia 1993, the Netherlands 2000, US West 1997) and from solver-verified mutants. Impossibility levels are taught as tradeoffs with a compute-budgeted manipulation tool. Students swap between designer and cartel coach, and replayed cartel transcripts are the graded artifact. Seats are the wedge, and red-teaming agent-facing marketplaces is the business.

**Moonshot (2036): crash-test ratings for markets.** By then most bids, prices and votes are cast by agents for humans. Before an agent-facing marketplace, ad exchange or procurement portal launches, it publishes its rules in the DSL and runs them through a public, versioned adversary zoo, the way cars face NCAP. The rules get a star rating for collusion resistance. Exploits go into a public registry of mechanism vulnerabilities, CVEs for institutions, seeded by coursework. Competition authorities stop suing after cartels form and start rejecting designs that fail.

**Scores.** **Innovation 5.** The design-attack-patch loop, the difference meter and mutant levels form a new category, but the brief already names the sandbox, and MobLab and Concordia are close neighbours. **Feasibility 7.** Known stack, cheap waves, a clear course wedge. The held-out-human validation is the real risk.

### Turn 6 — Builder
I'm scoring innovation 4, not 5, because the brief named the sandbox and MobLab plus Concordia cover most of the substrate; the real inventions are the difference meter and solver-verified mutant levels, and they're good ones. Feasibility 7 stands: every piece is known, waves are cheap, and the held-out-human kill metric gets tested by week 12 instead of assumed. If "CVEs for institutions" ever exists, it starts as a course assignment like this one.
