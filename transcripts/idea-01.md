# Idea 01 — Precedent
> Seed: 1. [mild] **Healthcare AI × agent-to-agent negotiation × insurance.** Payers are deploying agents. Who builds the *patient's* agent that fights back?

### Turn 1 — Visionary
**Working name:** Precedent

**Hook:** A patient-side appeals agent whose real weapon is shared case law: every appeal anyone wins against a payer becomes a citation for the next person.

**Core mechanic:** Not "an LLM that writes your appeal letter." A payer's denial pipeline is a *classifier*, and denial-by-attrition only pays because almost nobody appeals (KFF analyses find well under 1% of denied ACA-marketplace claims get appealed). The verb is **counter-cite**. The loop:
1. Snap the denial letter or EOB, or pull it through the payer's FHIR Patient Access API.
2. The agent decodes the CARC/RARC reason codes, fetches the plan's own medical policy, and drafts the appeal.
3. It queries the **Precedent ledger**, an anonymized record of outcomes keyed by payer × CPT/drug × denial reason × policy clause: "This payer reversed 14 of 17 appeals for this code when the letter cited clause 4.2b and attached the step-therapy history."
4. It tracks deadlines and escalates to state external review automatically.
5. The outcome flows back into the ledger, so the next appeal is better armed.

**Why these domains collide productively:** Insurance denial is an asymmetric-cost game. The payer's marginal cost to deny is close to zero; the patient's is hours of misery. An agent drives the patient's cost toward zero too, and the pooled ledger turns individual grievances into something like common law. That is where the equilibrium shifts: if wrongful denials get appealed at 30% instead of 0.5%, blanket denial stops paying.

**Why now:** PrescriberPoint shipped an autonomous prior-auth agent in April 2026, and 93% of physicians report care delays from prior auth (brief §10). Investment flows to providers and payers, not patients. CMS's interoperability and prior-auth rule (CMS-0057-F) requires payer prior-auth APIs by 2027: a real machine endpoint for agent-to-agent negotiation instead of a fax line.

### Turn 2 — Builder
**What exists.** The appeal-letter generator is already a crowded, cheap category. **Claimable** (AI appeals, roughly $40 each, strong on specialty drugs), **Counterforce Health** (free AI appeals), **Fight Health Insurance** (open-source, free), and **Sheer Health** all ship it. Goodbill and Resolve negotiate bills. Nobody owns the **ledger**. That's the real product.

**Correction.** CMS-0057-F's prior-auth API is provider-facing. Patients get PA *status* through the Patient Access API starting in 2027, not a negotiation endpoint. For now, "agent vs agent" means our agent against a fax machine and a portal.

**Hardest problem: the ledger starts empty and stays sparse.** Payer × code × denial reason × clause splits into millions of cells, most with n<5. Outcomes come back late or never: the patient stops answering once they get the drug. Payers will also rewrite clause 4.2b the moment it starts losing. And a winning appeal usually needs the physician's letter of medical necessity, which the patient can't produce alone.

**MVP (90 days).** One wedge: **GLP-1 and specialty-drug prior-auth denials** on commercial plans in three states. That's high volume, the denials are formulaic, and patients are motivated. Flow: upload denial → decode → policy fetch → appeal draft → a **one-click letter-of-medical-necessity packet sent to the prescriber's office** → deadline tracker → external-review filing. Outcome capture works by asking the user to forward the decision letter, with a $10 refund as the reward.

**Twist: seed the ledger from public law instead of our users.** California DMHC has published thousands of Independent Medical Review decisions since 2001 (searchable, with the rationale), and New York DFS publishes its external appeal decisions. Those are real adjudicated precedents with reviewer reasoning. Add CMS-0057-F's required public prior-auth approval/denial metrics, which payers have posted since 2026. That gives Precedent real case law on day one, with no cold start, and makes "citation" literal.

### Turn 3 — Visionary
Correction accepted: our opponent is a fax machine for now. The twist changes what the ledger stores: **arguments**, not payer × code × clause cells.

**Mutation: a Shepard's citator for health insurance.** Lawyers don't search for identical cases. They check whether an *argument* is still good law. So Precedent breaks every DMHC and NY DFS decision into argument units, for example "step therapy satisfied by a prior trial under a different plan" or "BMI criterion measured at initiation, not continuation", and each unit carries a status flag: *followed*, *distinguished*, or *superseded*. Sparsity largely goes away because arguments transfer across payers and drugs where exact cells don't. Hierarchical Bayesian shrinkage (the pharmacovigilance trick for rare adverse events) estimates overturn odds for thin cells by borrowing strength from the drug class and the payer family.

**Payers rewriting clause 4.2b becomes a signal.** We crawl and version every payer medical policy. Appeals are judged against the policy in force on the date of service, so a rewrite doesn't touch pending cases. A rewrite right after a cluster of losses shows what the payer feared.

**Outcome capture:** the patient names Precedent as their **authorized representative**, which ERISA and ACA appeal rules allow. Decision letters then come to our fax inbox. That closes the loop without nagging, and the payer now faces an entity that remembers everything. The LMN packet is pre-filled from the patient's FHIR record, so the prescriber only reviews and signs.

**Emergent behaviour:** with enough citations, the ledger shows the payer's decision boundary in public. Texas's gold-card law already exempts *physicians* with ≥90% approval rates from prior auth. Precedent can make the mirror-image case to regulators: any drug × criterion cell that's overturned more than 90% of the time on review shouldn't need prior auth at all. Patients' common law becomes rulemaking evidence.

### Turn 4 — Builder
**Kill: the "90% overturn means drop prior auth" pitch to regulators.** Appealed cases are self-selected strong cases, so overturn rates on review overstate how often denials are wrong. Make it an annual report later, not a feature. **Also kill user-facing odds.** "82% chance to win" is a liability. Show citations and use the Bayesian scores only for internal ranking.

**The real gap: ERISA self-funded plans**, roughly 65% of covered workers. They sit outside DMHC and DFS, and their IRO decisions are unpublished, so coverage there is thin, and users are told so.

**Business model.** Free for patients. The revenue is a **citator API licensed to the provider side**: specialty pharmacies, PA teams and agents like PrescriberPoint. Pharma hub services are the eager buyer and the danger, so they're firewalled from ranking and their revenue is disclosed.

**Stack.** OCR and structured LLM extraction, Postgres with pgvector, a versioned policy crawler, a HIPAA-BAA fax API, Flexpa for FHIR, and e-signature.

**Failure modes and abuse.**
- *Hallucinated citations.* Remember Mata v. Avianca: argument units are extractive only, each linked to its source decision.
- *Ledger poisoning.* Outcomes are accepted only as payer-faxed decision letters.
- *Regulatory status.* A non-covered entity holding PHI falls under the FTC Health Breach Notification Rule and Washington's My Health My Data Act. Representation at scale may trigger state claims-adjuster licensing.
- *False hope.* Weak citations mean saying so and routing the patient to cash options ($149 orforglipron).

**90 days.**
- Weeks 1–4: ingest CA IMR and NY DFS, extract GLP-1 and specialty argument units, and have a pharmacist and a lawyer audit 300.
- Weeks 5–8: authorized-representative flow and LMN packet for three payers.
- Weeks 9–12: 200 live appeals via an advocacy partner. **Kill metric:** under 50% of LMNs signed within 7 days means it's a provider tool, so pivot.

### Turn 5 — Visionary
Both kills accepted: overturn rates are selection-biased, and patients need reasons, not odds. One fix for the ERISA gap: as the authorized representative, Precedent can invoke the claimant's right under the ERISA claims regulation (29 CFR 2560.503-1) to a free copy of the whole claim file, *including any internal rule or guideline relied on*. Every ERISA appeal then pulls the payer's hidden criteria into the ledger. The unpublished corpus builds case by case, by legal mandate.

**Final form.** Precedent is a free patient-side appeals agent, and under it sits a citator. Argument units are extracted only from published IMR and external-review decisions plus ERISA claim files. Each unit is linked to its source, carries a Shepard's-style status, and is matched against versioned payer policies by date of service. The patient makes Precedent their authorized representative. The agent drafts the appeal, the prescriber signs a pre-filled LMN, and decision letters come back by fax to close the loop. Revenue comes from licensing the citator API to provider-side PA teams, with pharma firewalled off.

**Moonshot (2036).** Payer medical policies get published as **executable policy-as-code**. Every denial must then cite the exact rule version that fired. The patient's agent and the payer's agent argue over FHIR in seconds, citing a shared common law that no single side owns. That common law sits in a nonprofit trust, a "Westlaw of coverage" that is a public good. Unsettled disputes reach human reviewers already briefed. Denial-by-attrition stops working because nobody gets worn down anymore.

**Scores.** **Innovation 3.** Every primitive exists. The new parts are the citator framing and authorized-rep outcome capture, which are clever recombinations at the OACP anchor, not a new category. **Feasibility 7.** Public corpora, an ordinary stack, a sharp wedge. Licensing, PHI law and LMN sign-off friction keep it from scoring higher.

### Turn 6 — Builder
The claim-file request under ERISA's claims regulation is the best move in this thread. It turns a legal right nobody uses into a data pipeline for the hidden half of the market, and it's the part I'd ship first. My verdict: buildable, honest at innovation 3, and the moat is the citator corpus, not the letter-writer, which is already a commodity. It lives or dies on whether prescribers sign the LMN packets, and that gets tested in week 12, not assumed.
