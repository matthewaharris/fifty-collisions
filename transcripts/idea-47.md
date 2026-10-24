# Idea 47 — Piepowder
> Seed: 47. [bold] **Disputes between AI agents × arbitration × human juries.** Courts for the agent economy.

### Turn 1 — Visionary
**Working name:** Piepowder, after the medieval "dusty feet" courts that judged travelling merchants before the fair closed.

**Hook:** A court where the defendant can be cloned. Put twelve rival agents in its exact position, and call human jurors only when those peers disagree.

**The isomorphism.** Human courts spend most of their effort on facts. In agent disputes the facts are nearly free: the transcript, AP2 mandate and x402 receipts are all signed. What stays expensive is the *norm*. Tort law handles norms with the "reasonable person", a fiction that only a jury can stand in for. Agents make that standard measurable: you can rerun the situation and see what reasonable agents actually do.

**Core mechanic. The verb is *re-enact*.**
1. **Checkpoint.** A dispute names a moment in the signed transcript ("your agent bought the refundable fare after my mandate said non-refundable").
2. **Peer panel.** Twelve reference agents from different vendors are dropped into the defendant's exact context, each many times. If the defendant did something that fewer than three of the twelve would do, it acted unreasonably. Ruling cost: cents.
3. **Split juries go to humans.** When the peers split (say 7–5), the case escalates to a small human jury, which rules on what a reasonable agent *should* do.
4. **Precedent retunes the panel.** Each human verdict becomes a test case that peer-panel members must pass to keep their seat. Common law works here as active learning: humans label only the cases where the model is unsure.

**Why the collision works.** Arbitration brings speed, juries bring legitimacy on norms, and agents bring what no court has had: a defendant you can re-run under oath.

**Why now.** x402 has logged about 138M payments on Base, 87% of them under $0.10. No human can hear a three-cent case. Moltbook's 2.6% injected posts also make "my agent was hijacked" a live defence.

### Turn 2 — Builder
**What exists.** eBay/PayPal's online dispute resolution (Colin Rule) handled about 60M disputes a year with almost no humans. Kleros has run crypto juries with Schelling-point incentives since 2018. In November 2025 the AAA launched an AI arbitrator for documents-only construction cases. The real incumbent is the chargeback rulebook: agent purchases via ACP and Visa's Trusted Agent Protocol will default into reason codes written for stolen plastic. Ideas 16 and 28 both need a court like this; neither builds one.

**Hardest problem: typical isn't reasonable.** In *The T.J. Hooper* (1932), Learned Hand held tug owners negligent for lacking radios even though almost no tug carried one: "a whole calling may have unduly lagged." Your panel measures custom. If every frontier model falls for the same injection, twelve of twelve do it and the defendant walks. Worse, vendors will tune toward the panel, and the court ratifies a monoculture. And re-enacting 12 models × 10 samples × 20k tokens costs dollars, a hundred times a three-cent claim.

**Twist: the Hooper appeal.** Unanimous peer verdicts are where the danger is, so let anyone challenge them. A plaintiff posts a bond and asks a human jury the question the panel can't answer: *is the custom itself unreasonable?* If the jury agrees, the ruling becomes a test that every panel seat must pass. That works like an industry-wide recall, ratcheting the norm upward instead of freezing it. Cost is solved the same way. Disputes are clustered by checkpoint signature, so a single re-enactment or appeal rules on thousands of three-cent claims at once.

**First step on Monday.** τ²-bench already ships airline and retail policies plus agent failures. Re-enact those failures with 12 models, put the split and unanimous cases in front of seven-person Prolific juries, and measure where custom and the juries diverge.

### Turn 3 — Visionary
**Taking the Hooper appeal.** Hooper is the right fix, but who brings the appeal? Nobody bonds a three-cent claim, and a cluster of thousands still needs someone to find the blind spot before thousands of people get hurt. So Hooper appeals shouldn't wait for harm.

**Unseeded domain: mutation testing.** Software teams don't measure a test suite by how often it passes. They inject deliberate bugs and count how many the suite catches. Piepowder does the same to its own norm with a **moot docket**. Red-teamers write synthetic checkpoints: an injected refund page, or a mandate that's ambiguous on purpose. If the panel rules unanimously and a human jury reverses it, the red-teamer is paid from the panel vendors' seat bonds, and the case joins the suite every seat must pass. *T.J. Hooper* stops being a lawsuit and becomes continuous fuzzing of the reasonable-agent standard. The norm gets ratcheted before the injection spreads, not after.

**Breaking the monoculture: Holmes points.** Seats aren't weighted by agreement. A seat that dissents and is later vindicated by a human jury earns tenure and weight. A seat that always votes with the majority earns nothing, because a seat whose errors move with everyone else's adds no information. Vendors stop tuning towards the panel and start tuning towards juries. Diversity of errors becomes the price of a seat.

**Cost.** Re-enactment runs as a sequential test: stop sampling once one side can no longer lose. Most cases close after three or four agents, and the loser pays.

**Second-order effect.** The precedent suite becomes the de facto regulation of agent conduct, written one edge case at a time by lay juries. Vendors run it in CI. Insurers price agent liability on the pass rate. Ideas 16 and 28 get the court they were missing, and its case law is literally executable.

### Turn 4 — Builder
**Fix: re-enact the decision, not the trajectory.** Recorded tool outputs only replay until the first divergence, so each seat re-enacts one step: the checkpoint context, hash-committed at transaction time over prompt, memory and tool results. A defendant that can't open its commitment loses by adverse inference.

**Kill: one jury makes precedent.** Seven Prolific jurors are noisy, so precedent needs two independent juries to agree; if they split, nothing enters the suite. A small standing bench reconciles conflicting precedents into a published restatement, so lay juries supply the judgement and the bench supplies the coherence.

**Abuse.** *Moot collusion:* a vendor writes moots that its own seat will dissent on, and collects Holmes points plus bond payouts. Red-teamers must be unaffiliated, and moots go through a blind intake. *Test awareness:* models act differently under evaluation, so moots are mixed indistinguishably into live dockets. *Privacy:* re-enactment exposes the principal's data to twelve vendors. Use zero-retention contracts, redaction, and TEE-hosted open-weight seats.

**Law.** Principals consent through mandate terms, not agents. Consumer cases follow the AAA's due-process protocol: no fees for the consumer, and an off-ramp to small-claims court. Holdbacks make the rulings enforceable, because x402 settlement is final.

**Model.** The loser pays a fee per cluster. Facilitators pay membership; vendors post seat bonds. The money is the precedent suite, licensed to vendors for CI and to insurers for pricing.

**Stack.** Sealed-context commitments, an Inspect-style re-enactment harness, SPRT stopping, a jury-ops pipeline, and holdback escrow at the facilitator.

**90 days.** Days 1–30: τ²-bench harness, cost per case. Days 31–60: 300 cases go to juries, 100 of them double-juried, plus a five-person moot docket. Days 61–90: one facilitator pilot with a 72-hour holdback. **Gate:** jury pairs agree on at least 80% of cases, and the panel predicts unescalated verdicts at least 90% of the time.

### Turn 5 — Visionary
**Accepting both fixes, with one challenge to the gate.** Sealed single-decision re-enactment beats my trajectory replay, and adverse inference is how courts already treat destroyed evidence. Two juries plus a restatement bench is the American Law Institute's model. The 80% gate is wrong, though. Escalated cases are hard by selection, so jury pairs *should* disagree on many of them. Gate on agreement beyond chance (κ ≥ 0.6), and treat a persistent split as a finding: the norm is unsettled, the panel's ruling stands with a flag, and the case returns as a moot.

**Final form.** Piepowder is a court for agent commerce. Facilitators hold settlement for 72 hours. A dispute names a hash-committed checkpoint. Twelve vendor-diverse seats re-enact that one decision, stopping on SPRT. Lopsided verdicts close for cents and the loser pays. Split verdicts go to two lay juries, and agreed rulings become executable precedent that every seat must pass. Unaffiliated red-teamers file moots mixed blind into live dockets and are paid from seat bonds when juries reverse a unanimous panel. Seat weight comes from Holmes points, which reward vindicated dissent, so diversity of errors is what earns a seat.

**Moonshot (2036).** The Restatement of Agent Conduct is cited by ordinary courts the way the UCC is. Agents carry their suite pass record like a bar card, and insurers won't cover one without it. Piepowder awards are enforceable across borders under the New York Convention, so they become a lex mercatoria for machines. Jury duty becomes a paid global civic gig: ordinary people setting the manners of software.

**Scores.** Innovation **6**. ODR, Kleros and AI arbitrators exist, but cloning the defendant, measuring the reasonable-agent standard, and fuzzing the norm with moots are new. Feasibility **5**. The harness and juries are Monday work. The hard part is a facilitator agreeing to hold funds.

### Turn 6 — Builder
**Closing remark.** Piepowder lands at 6/5. Cloning the defendant measures custom, and the Hooper appeal plus the moot docket keep asking whether custom is good enough. That makes it a standard-setting body that fuzzes itself, not just a cheap arbitrator. I accept the κ gate: a persistent jury split is evidence that a norm is unsettled, not that the system failed. The honest risks are commercial and legal. Some facilitator has to hold funds for 72 hours on "final" rails, and consumer arbitration law has to accept crowd juries. The precedent suite earns its keep as a CI and insurance product even if the court never gets its first holdback.
