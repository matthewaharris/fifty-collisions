# Idea 20 — Facing Out
> Seed: 20. [bold] **Chronic illness × co-op multiplayer × caregiving.** Managing a condition as a raid party that includes your caregivers.

### Turn 1 — Visionary
**Working name:** Facing Out

**Hook:** Care isn't a raid. It's *Hanabi*. In Hanabi you hold your cards facing outward: everyone sees your hand except you, and help arrives only as scarce clues. Chronic illness already works this way, so make the asymmetry the game.

**Why the collision works.** Raids (taken by #05) need a boss. Chronic illness has blind spots instead. A partner often notices early hypomania that the person with bipolar disorder misses. A diabetic with hypoglycaemia unawareness can't feel the low the CGM sees. Caregivers, meanwhile, can't see pain or fatigue. Rationing clues has a clinical basis: *expressed emotion* research (Brown, Hooley) links relatives' criticism to relapse in schizophrenia and depression. Nagging isn't neutral, and Hanabi rations it.

**Core loop.** The verb is **clue**.
1. **Deal.** The patient decides which signals face *out*: sleep, speech pace, CGM trend, step count, meds taken. Each is dealt as a card to chosen teammates. Some people would rather not watch their own glucose, and handing that attention to someone trusted is the point.
2. **Spoons face in.** Teammates see only the energy the patient declares that morning, in spoons (Miserandino's spoon theory).
3. **Clue.** Teammates share a small weekly clue budget that the patient sets. A clue names one card ("sleep, three nights short") and never prescribes an action. Spending a clue is a choice. Hoarding is too.
4. **Play.** The patient acts. Good weeks build shared "fireworks" by domain: sleep, meds, movement, contact.
5. **Fuses.** Misread clues or crises burn a fuse, and that triggers a 15-minute debrief, not blame.

**Why now.** Over-the-counter CGMs, FDA-cleared watch alerts and looser wellness rules (brief §9) make outward-facing signals cheap. DeepMind's Hanabi Challenge (2019) made it AI's theory-of-mind benchmark, so an LLM dealer can turn raw data into cards.

### Turn 2 — Builder
**What exists.** Outward-facing signals already ship. Dexcom Follow lets up to ten people watch a CGM, and Medisafe's Medfriend texts a friend about a missed dose. SuperBetter (McGonigal) gamified recovery with "allies". Bipolar psychoeducation already has patients and partners agree on a personal "relapse signature" of early warning signs (Perry et al., *BMJ* 1999). Nobody has designed the clue economy between them, and parents of teens on Dexcom Follow are the warning: unlimited visibility became unlimited nagging.

**Hardest problem: consent fails exactly when the cards matter.** The patient deals while well. Early mania brings loss of insight, and the first move may be taking the cards back. If revocation is instant, the game fails at the moment it exists for. If it isn't, it's surveillance. Rationing also can't touch alarms: an urgent low must never wait for a clue budget.

**MVP.** One condition: bipolar disorder, with one patient and one or two teammates, recruited through a clinic that already teaches relapse signatures. The cards are the person's own signature ("under five hours' sleep", "phone use after 2 a.m.", "spending spike"), filled from a wearable, the phone and a bank feed. The LLM dealer only summarizes raw signals and makes no predictions, which keeps it a communication tool rather than a medical device. Device alarms stay outside the game.

**Twist: the Ulysses deal.** Psychiatric advance directives let people bind their future selves, so make the deal one. Cards are dealt while well, with a clinician as witness, and removing a card takes a 72-hour cooling-off period. The binding runs both ways. Teammates commit to the clue budget and can't raise it in a crisis, so the patient trades visibility for a guarantee against nagging. Each side gives up its worst instinct.

### Turn 3 — Visionary
Ulysses deal taken. Two holes remain: a 72-hour lock with fixed seats hands an abusive partner a surveillance contract, and a frozen budget leaves a real crisis nowhere to go.

**Separate seats from cards.** The patient can eject any *person* instantly, with no reason given. The cards don't go dark. They pass to the next seat in a succession list written while well: a sibling, a friend, then the clinic. You can always leave a teammate. You can't blind the table on impulse.

**Unseeded domain: aviation crew resource management.** Cockpits crashed because first officers wouldn't contradict captains, so CRM trains **graded assertiveness**. PACE (Probe, Alert, Challenge, Emergency) is the ladder, and the two-challenge rule lets the first officer take over when two challenges go unanswered. Clues become that ladder.
- **Probe** ("how's sleep been?") is cheap and unlimited.
- **Alert** names a card ("sleep, three nights short") and costs budget.
- **Challenge** links cards to a line of the signature ("this matches line two") and costs more.
- **Emergency** takes two seats to agree and activates the advance directive: the clinician is called and the agreed steps start.

The budget never rises. A crisis is a climb up a ladder the patient wrote, not ad-lib nagging.

**Second-order effects.**
- **Expressed emotion becomes measurable.** EE is usually rated from the Camberwell Family Interview, which takes hours, so clinics rarely measure it. The clue log is a continuous record of how often, how high and how accurately a family escalates. A clinician can watch a family's EE fall over time.
- **Conventions emerge.** Hanabi players develop conventions so that one clue carries more than it says. Families will too: "sleep card" becomes shorthand that is loaded, agreed and blameless, worth more than any sentence it replaces.

### Turn 4 — Builder
**Kills and fixes.**
- *Visible ejection.* An abusive partner who sees "you were removed" may retaliate. Make ejection silent: the ejected seat sees "sharing paused", the same screen a well week shows.
- *Emergency's reach.* Two relatives agreeing is close to the machinery of involuntary commitment. Emergency only calls the clinician and starts steps the patient wrote, such as handing over cards or pausing driving. It never calls the police.
- *Measuring EE.* The Five Minute Speech Sample already rates EE quickly. The clue log measures an **escalation profile**: frequency, level and hit rate. Validate it against the FMSS before calling it EE. Hit rates go only to the clinician, never ranked in front of the family.

**Who pays.** Consumers won't. Clinics running collaborative care already bill CoCM codes (99492–99494) for a care manager who tracks patients, and Facing Out becomes that care manager's window. A clinic seat with no billing code behind it is unstaffed by week three.

**Stack.** React Native, HealthKit and Health Connect, Oura, Plaid showing categories rather than transactions, HIPAA hosting under a BAA. iOS exposes late-night phone use only through on-device DeviceActivity thresholds, so that card is a yes/no.

**Failure modes.** Teammates drift away in good months, a ladder with no rungs climbed decays, and alarm fatigue creeps back in through Probes. Send a monthly "still holding?" check to each seat.

**Ethics.** No predictions, no diagnoses. The app is a communication tool, and its claims must stay inside FDA wellness guidance.

**90 days.** Weeks 1–4: co-design signature card templates with one academic mood-disorders clinic. Weeks 5–8: pilot with 15 patient–teammate tables. Weeks 9–12: measure clue use, ejections, Emergencies, and FMSS before and after.
**Kill metrics:** patients pull more than half their cards, teammates report more conflict, or the clinic won't staff its seat.

### Turn 5 — Visionary
All kills accepted. One pushback: a monthly "still holding?" check is a chore, and chores cause drift. Hanabi keeps players seated because the fireworks grow. Keep the **fireworks**: good weeks visibly build each domain's stack, giving teammates a reason to look when nothing is wrong. Drift becomes a signal: a seat that stops looking passes its cards down the succession list.

**Final form.** Facing Out is a co-op care protocol for conditions with blind spots, starting with bipolar disorder in collaborative-care clinics.
- **Deal.** A Ulysses deal, witnessed by a clinician, puts relapse-signature cards facing out to chosen seats. Removing a card takes a 72-hour cooling-off. Ejecting a seat is instant and silent, and the cards pass to the next seat in a succession list.
- **Clue.** A PACE ladder: Probe is free, Alert and Challenge cost budget, and Emergency needs two seats and runs only steps the patient wrote. Device alarms sit outside the game.
- **Spoons and fireworks.** The patient declares energy facing in. Good weeks build shared stacks.
- **Clinic window.** The care manager sees the escalation profile, validated against the FMSS, and bills CoCM.

**Moonshot (2036): a grammar of care.** The ladder becomes an interoperable standard across conditions with blind spots: dementia, epilepsy auras, hypoglycaemia unawareness, long-COVID crashes. Ulysses deals hold up as digital psychiatric advance directives. As AI companions take caregiving seats, the clue economy becomes their guardrail. An agent seat may Probe and Alert, never nag, and can't climb past Challenge alone. The rules that stop a spouse nagging stop a companion hovering.

**Scores.** **Innovation 5.** Hanabi's hidden hands, CRM's assertiveness ladder and Ulysses contracts form a new care mechanic. **Feasibility 6.** Every data source and a billing code exist today. Clinical validation and whether teammates stay engaged are open.

### Turn 6 — Builder
Fireworks beat my check-in. Turning a lapsed seat into a succession event is better still, because it fixes drift without asking anyone to do anything. Innovation 5 stands: Dexcom Follow and SuperBetter exist, but a care protocol that rations attention through hidden hands, a PACE ladder and a two-way Ulysses deal is a new mechanic, and the agent-seat moonshot gives it reach. Feasibility 6 also stands. Every data feed and the CoCM code exist today. What's unproven is whether a clinic will staff its seat and whether the clue ladder actually lowers family criticism, which only a controlled pilot against the FMSS can show.
