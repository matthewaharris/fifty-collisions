# Idea 15 — Yarrow
> Seed: 15. [bold] **Mysticism (tarot/I Ching) × game theory × decision science.** Divination reframed as structured randomness and reflection for real decisions.

### Turn 1 — Visionary
**Working name:** Yarrow

**Hook:** An oracle that diagnoses which *kind* of randomness your decision needs, then casts it: a binding mixed strategy or a mirror for your gut.

**The isomorphism.** Divination was never about the future. Omar Khayyam Moore (1957) argued that Naskapi hunters reading cracks in a burnt caribou scapula to pick a direction were playing a **mixed strategy**, unpredictable to game and immune to their own over-hunting habits. And the yarrow-stalk I Ching is a *biased sampler*. Its line probabilities are 1/16, 5/16, 7/16 and 3/16, unlike the coin method's 1/8, 3/8, 3/8, 1/8. The ancients built weighted RNGs and wrapped them in ritual so people would obey them.

**Core loop.** The verb is **cast**.
1. **Triage.** An LLM interviews you about the decision and classifies it.
   - *Adversarial or exploratory:* which supplier to audit, which route to patrol, which idea to test next. The app solves for the optimal mix (game solve or Thompson sampling).
   - *A true tie:* Buridan's ass. Levitt (2021) found that people whose coin flip told them to make a big change were happier six months later.
   - *A hidden preference:* Piet Hein's grook says flip the coin and notice which side you hope for.
2. **Cast.** For **Bind** decisions, the hexagram *is* the sample, weighted to the optimal probabilities, shown openly. The ritual stops you overriding. For **Mirror** decisions, you log your hope before the reveal and your reaction after. The gap is the reading.
3. **Journal.** Overrides are logged. Ninety days later, the app asks how it went.

**Why the collision works.** Humans randomize badly and obey randomness worse. Game theory says when to randomize, mysticism supplies obedience, decision science the follow-up.

**Why now.** AI astrology apps reportedly earn millions monthly (Starla ~$2.2M/month in 2025, vendor blog, soft). Delegated agents that behave predictably get exploited (brief §3).

### Turn 2 — Builder
**What exists.** Eno and Schmidt's *Oblique Strategies* (1975) is structured randomness for creative choices, and Rhinehart's *The Dice Man* (1971) is the warning. Bind mode already runs at institutional scale: Tambe's Stackelberg security games randomized LAX checkpoints (ARMOR, 2007), Federal Air Marshal flights (IRIS) and Coast Guard patrols (PROTECT). On the consumer side, Co-Star, The Pattern, Nebula and Labyrinthos sell meaning without math, and wheel-spinner apps sell math without meaning. Nobody joins the two.

**Hardest problem: two products in one robe.** The people who want the ritual rarely have adversaries, and the people who have adversaries (auditors, patrol planners, security teams) won't wear the robe. Bind also lacks enforcement. Tambe's schedules bind because an agency follows its own roster, while a consumer overrides a hexagram for free. An LLM triage tuned to sell Bind will find adversaries in a lunch order.

**MVP.** iOS, with three modes behind an LLM interview. Content is public domain: Legge's 1882 I Ching and the 1909 Rider-Waite-Smith deck. The sampler assigns hexagrams to options so their yarrow-weighted mass matches the solved mix, and the cast is drawn from the drand public beacon, so anyone can verify it wasn't rigged. Start with frequent, low-stakes choices (which task, which lead, which habit to try), where Thompson sampling actually beats gut and outcomes come back within days, not 90.

**Twist: the oracle earns its authority.** Overrides are already logged. Rate the outcomes of both obeyed casts and overrides, then compare them per decision class. Where your overrides lose to the cast, the oracle shifts toward Bind for that class. Where your gut wins, it backs off into Mirror. Obedience is earned from your own ledger rather than demanded by ritual, and the reading tells you, with receipts, where your instincts are noise.

### Turn 3 — Visionary
Earned authority is right, but the ledger is confounded: you override when you know something, so overrides look smart. The fix is in the cast. **The cast is randomized, so it is an instrument.** Mirror mode logs your hope before the reveal. Whether the cast *happened to agree with your gut* is pure chance, so comparing outcomes when it agreed versus disagreed gives an unbiased estimate of whether your gut beats chance in that decision class. That is Levitt's design, run on one life. The reading becomes: "for hiring calls your gut is worth +0.4 points; for restaurants it's noise."

**Solving the hardest problem: the robe and its adversary.**
- *The consumer's adversary is their own habit.* That was Moore's point: the Naskapi weren't outwitting caribou so much as their own ruts. Bind for individuals means exploration (which friend to call, which book, which route), where Thompson sampling beats routine and outcomes return in days.
- *Triage gets a hard rule against overselling.* Bind is allowed only for repeated decisions, declared ties, or groups. A one-off life decision always defaults to Mirror, whatever the LLM thinks.

**Unseeded domain: sortition.** Enforcement comes free in groups. Athens filled offices with the kleroterion. New Zealand's HRC and the Swiss NSF now pick among shortlisted grant proposals by lottery. Groups honour lots because a lot is fair and **absorbs blame**. So add a **Casting Table**: housemates splitting chores, a team choosing which of three good bets to fund, co-parents taking turns on holidays. Everyone sees the weights, the drand beacon draws, and nobody is the villain.

**Second-order effect: blame becomes a budget.** Groups deliberately choose which decisions to hand to chance, and that list is revealing. A team that casts too much is avoiding responsibility. One that never casts burns trust on every tie.

### Turn 4 — Builder
**Kills and fixes.**
- *The instrument, as stated.* Sound as intent-to-treat, but the outcome isn't blinded: you rate knowing whether it was your gut's pick, so self-justification leaks in. Ask for ratings in neutral terms ("how did Tuesday's call go?") and never show the agree/disagree flag before the rating.
- *"Your hiring gut is worth +0.4."* Five hiring calls a year won't produce that. Per-person scores work only for frequent classes. For rare ones, pool across opted-in users and show a population prior with a wide interval.
- *Blame budget as a score.* Telling teams they "cast too much" turns an insight into a nag.
- *Bind with absent third parties.* A cast never decides for someone who isn't at the table, such as a candidate or a child. Casting Table requires every affected person present, and everyone must assent to the weights before the draw.

**Business model.** $6/month consumer premium for the ledger and gut reports. The real B2B line is partial lotteries for funders: the NZ HRC and Swiss NSF model for small foundations, hackathons and internal innovation funds, with a drand-backed audit trail.

**Stack.** SwiftUI. Triage hard rules in code, not the prompt. drand quicknet with options and weights committed before the target round. Beta-Bernoulli Thompson sampling. Local-first, end-to-end encrypted journal. A Slack app for Casting Table.

**Safety and abuse.** Health, medication, self-harm and large money decisions are Mirror-only, with crisis routing. Re-casting by rewording is logged and shown; it's a Mirror signal itself. Rate-limit Bind so nobody becomes the Dice Man.

**90 days.** Weeks 1–4: Mirror mode, triage, drand cast, Legge text. Weeks 5–8: three exploration templates, three-day outcome pings, Casting Table. Weeks 9–12: 1,000-user beta, pitch three small funders.
**Kill metrics:** under 30% of casts get an outcome rating, or day-30 retention under 15%.

### Turn 5 — Visionary
All four kills accepted. Blinded ratings fix my instrument; pooling is the honest answer to five hiring calls a year. The absent-third-party rule is the thread's key line: a lot is legitimate only when everyone it binds agreed to the weights, which is sortition's own ethic. The blame score goes; what a group casts stays private.

**Final form.** Yarrow is a decision app that triages before it casts.
- **Triage.** An LLM interview under hard-coded rules. Bind only for repeated exploration choices, declared ties, and Casting Tables where everyone is present and assents. Everything else is Mirror; health, money and crisis are Mirror-only.
- **Cast.** Options and weights committed before a drand round, mapped onto yarrow-weighted hexagrams with Legge's text, verifiable afterwards.
- **Mirror.** Your hope is logged before the reveal, and re-casting by rewording is shown back to you as a signal.
- **Ledger.** Blinded outcome ratings treat the cast as an instrument. Frequent classes get personal "gut versus chance" reports, and rare ones get pooled priors.
- **Business.** Consumer premium, plus partial-lottery tooling for small funders.

**Moonshot (2036): an atlas of intuition.** Millions of instrumented casts answer a question psychology could never fund: across decision classes, where does human gut beat chance, and for whom? Published with intervals, it settles what Kahneman and Klein could only argue about. The same verified lot becomes civic infrastructure for school places, housing lotteries and grant tie-breaks. AI agents call it for mixed strategies no counterparty can predict and any auditor can check.

**Scores.** **Innovation 5.** Oblique Strategies, ARMOR and funding lotteries are all precedents. New: triaging the *kind* of randomness, the cast as a personal instrument, ritual as a compliance device. **Feasibility 7.** Small, cheap stack. Retention and outcome ratings are the real risk, and both kill metrics test them.

### Turn 6 — Builder
I agree with 5/7. Each piece has a precedent, but combining randomness triage, the cast as a personal instrument and the absent-third-party rule is new, and that's enough for a bold tier. My doubt is the audience, not the stack. Tarot users may not want a ledger, and the ledger crowd may not want a hexagram. The 30% outcome-rating metric will show whether the ritual actually buys compliance. If it fails, the partial-lottery tool for funders still stands on its own.
