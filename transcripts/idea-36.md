# Idea 36 — Settlement
> Seed: 36. [bold] **Psychedelic-assisted therapy × VR × integration/measurement.** The infrastructure around the trip, not the trip itself.

### Turn 1 — Visionary
**Working name:** Settlement

**Hook:** A session issues IOUs. Integration is the clearing house that settles them, and its ledger becomes the trial's most honest endpoint.

**The isomorphism.** Psychedelic insights arrive with high "noetic" certainty and an unknown default rate; McGovern et al. (2024) propose a whole theory of *false insights* under psychedelics. Zen dealt with this long ago: a kensho claim isn't accepted until the teacher's **checking questions** test it. Forecasting dealt with it another way: confidence means nothing until it's scored. Settlement combines the two.

**Core loop. The verb is settle.**
1. **Reinstate.** At the first integration visit, a clinic VR headset rebuilds the session's context (same playlist, room and drawings) to cue recall. VR serves memory, not the experience.
2. **Issue.** With the therapist, each insight becomes a dated, checkable note ("by week 6 I'll have phoned my sister twice"), and the patient states a confidence.
3. **Settle.** Weekly check-ins settle notes against what happened, confirmed by a person the patient names. Vague insights get a checking question rather than a verdict.
4. **Rate.** Over 12 weeks, patients learn which kinds of their insights pay out. Clinics get **settlement yield**, a behavioural measure of durability. The same check-in logs everyone's guess of trial arm, so blinding is measured continuously.

**Why these collide.** Mouse work suggests psychedelics reopen a weeks-long critical period for social learning (Nardou et al., 2023). If that holds in humans, integration is when the work happens, yet it's the least measured stage. Clinician-run, inside trials and licensed programmes only.

**Why now.** Lykos's MDMA application failed in 2024 over functional unblinding, and Compass's 26-week psilocybin data is due this half, so blinding and durability are the questions regulators now ask. The FDA's January 2026 guidance (§9) and ambient scribes (§10) make structured integration buildable.

### Turn 2 — Builder
**What exists.** The settlement mechanic is 58 years old: **Goal Attainment Scaling** (Kiresuk & Sherman, 1968) has patients write personal goals and score them later. Osmind already gathers real-world outcomes from ketamine and psychiatry clinics. Wavepaths already scores sessions, so "same playlist" is one API. Bristol's Isness-D got psychedelic-level MEQ scores from VR alone, so VR is a plausible cue and also a confound. Blinding indices (Bang, James) exist, but trials ask the guess once, at the end. One correction: §9's January 2026 guidance covers wellness devices. The relevant FDA document is the 2023 draft guidance on psychedelic investigations, which explicitly names expectancy and functional unblinding.

**Hardest problem: suggestion, not measurement.** The plasticity window that makes integration valuable is also when patients are most suggestible, and the therapist co-writes every note. Yield could end up measuring therapist influence: easy notes settle and rich ones default. A new outcome measure also needs years of validation, so it starts as an exploratory endpoint.

**Twist: chain of custody for insight.** Seal both ends. Before the session, the patient writes a sealed **expectancy forecast** of what they think will change. Within hours after it, in the clinic, they record a **raw account** in their own words, sealed before any therapist interpretation. Every note issued later is stamped with its provenance: *forecast*, *own words*, or *co-authored*. Yield is reported for each. McGovern's false insights and the FDA's expectancy worry become a measured variable: **whose insights pay?**

**MVP.** A tablet and SMS tool for Oregon service centres (SB 303 already requires them to report client data) and one investigator-led trial. At one site, VR reinstatement is randomized against audio-only at the first integration visit. An independent rater collects weekly arm guesses.
**Kill metric:** week-12 yield adds nothing to symptom scores in predicting 6-month relapse.

### Turn 3 — Visionary
**Taking chain of custody, and handling suggestion with two tools: a coin and a bookmaker.** GAS is the right ancestor and the §9 correction stands. But provenance labels only *describe* therapist influence; they don't measure it.

**The coin: randomize the pen.** For each insight in the raw account, a coin decides whether the therapist co-writes the note or the patient writes it alone from a fixed template. That's a micro-randomized trial inside every patient (Klasnja et al.). Comparing yield across the two arms gives a causal estimate of what co-writing does, per therapist.

**The bookmaker: handicap every note (unseeded: racing handicapping).** "Easy notes settle" goes away if each note carries odds. The independent rater who collects arm guesses also prices each note's chance of settling, blind to arm and to who wrote it. Yield becomes **settled above the handicap**. Phoning a sister you call weekly earns nothing; ending a twenty-year silence earns a lot. Easy notes stop paying, and the rater's calibration is scored too.

**Echo score.** Seal the expectancy forecast before the session and the raw account after it, then compare them. If the raw account mostly repeats the forecast, the experience was shaped by expectancy. That gives a per-patient expectancy covariate, which the FDA's 2023 draft guidance asks for.

**Second-order: an actuarial table for integration.** Across sites, sealed accounts and settled notes add up to base rates by insight class: relational, behavioural, bodily, cosmological. Integration currently runs on lore. A clinician might learn, hypothetically, that cosmological certainties settle at 8% and relational repairs at 40%. Integration aims where insight pays; low-yield classes are held gently, not argued with.

**Tier check.** Without the coin and the handicap this is GAS with a headset. With them, it's a causal instrument for the most-criticised confound in the field.

### Turn 4 — Builder
**Kills.** *Echo score as designed:* text overlap captures ongoing concerns, not expectancy. Seal a validated scale (SETS) instead, and keep echo exploratory. *Cosmological base rates:* metaphysical claims can't settle, and "8%" would teach clinicians to dismiss spiritual experience. Uncheckable insights go in a separate **unsettled** class that gets checking questions and is never assigned a yield. *Per-therapist yield as a ranking:* it gets gamed within a quarter. It stays in supervision.

**Coin, narrowed.** Therapists talk about every insight in session, so the coin can only measure the effect of *co-authoring the note text*, not therapist influence overall.

**Handicap.** Two raters see baseline behaviour but not arm or author. If their ICC is below 0.5 after 60 notes, the handicap goes.

**Failure modes and safety.** SMS check-ins create a duty to respond, so any risk language goes to an on-call clinician within hours. Patients see no yield before week 12, and never as failure. Confirmers are optional and screened for coercion.

**Data.** Raw accounts are subpoena bait: trauma disclosures and custody disputes. "Sealed" means an RFC 3161 hash commitment. Text stays encrypted at the clinic; only hashes and outcomes leave.

**Abuse.** Clinics advertising their yield, and payers using it to deny coverage. The licence bans both. Upside: a pre-therapist timestamped account is evidence if misconduct is alleged.

**Stack and model.** REDCap module, SMS, Quest 3 at one site, PyMC for the micro-randomized analysis, and Part 11 audit trail. Trials pay per enrolled patient, as with an eCOA vendor (Signant, YPrime). Oregon centres pay SaaS. Base rates sit with a nonprofit consortium.

**90 days.** Days 1–30: preregister and get IRB approval with one academic site. Days 31–60: three Oregon centres, without the coin. Facilitators aren't therapists, so this tests feasibility only. Days 61–90: week-6 settlements and the handicap ICC.

### Turn 5 — Visionary
**Final form.** Settlement is a chain-of-custody and measurement layer for the integration window, used only inside trials and licensed programmes. Before the session, the patient seals a SETS expectancy scale. Within hours after it, they seal a raw account in their own words; both are RFC 3161 hash commitments, with text encrypted at the clinic. Checkable insights become dated notes. A coin decides, per note, whether the therapist co-authors the text, so the effect of co-authoring is estimated causally and narrowly. Two blinded raters set each note's odds from baseline behaviour, and yield is **settled above the handicap**, kept only if their ICC clears 0.5. Uncheckable insights go to the **unsettled** class, which receives checking questions and is never scored. That kill is right; it's closer to Zen than my 8% was. Weekly check-ins log arm guesses, giving a continuous blinding curve. Patients see nothing before week 12, and per-therapist data stays in supervision. Kill metric: yield adds nothing to symptom scores in predicting 6-month relapse.

**Moonshot (2036).** Integration gets prescribed like the drug. Base-rate tables from the consortium show which integration practices move which insight classes, and a label specifies a validated integration protocol the way Spravato's REMS specifies observation today. The continuous blinding curve spreads to every trial whose treatment can't hide itself: surgery, exercise, and digital therapeutics. Patients carry a sealed ledger across clinicians, so a raw account from a 2031 session is still unedited, theirs, and evidence.

**Scores.** Innovation **6**. Goal Attainment Scaling, eCOA and blinding indices all exist. What's new is randomizing the pen, handicapping the outcome and sealing provenance, which together make "whose insights pay?" a causal question. Feasibility **6**. A REDCap module ships in 90 days. Validation as an endpoint takes years, and the handicap may fail its ICC test.

### Turn 6 — Builder
**Closing remark.** Settlement holds its bold tier because of three moves: sealing both ends of the session, randomizing who holds the pen, and handicapping each note. Together they turn the field's most-cited weakness, expectancy and therapist suggestion, into a causal measurement instead of a footnote. Feasibility is 5, not 6: the software takes 90 days, but micro-randomizing note authorship inside a psychedelic trial needs a sponsor willing to give up its own endpoint narrative, and the whole yield metric depends on two raters agreeing on odds, which nobody has tested. If the handicap fails its ICC test, what's left is a rigorous GAS-plus-blinding tool: useful, but a 4.
