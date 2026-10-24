# Idea 11 — Washout
> Seed: 11. [mild] **Personal n-of-1 trials × GLP-1/CGM × real statistics.** Experiments on yourself, not dashboards.

### Turn 1 — Visionary
**Working name:** Washout (the gap between arms of a crossover trial).

**Hook:** The anti-dashboard. While the trial runs, you can't see your own glucose. You get a sealed randomization schedule, and the only output is a permutation test on the day you unseal.

**Core loop.** The verb is **randomize the day**.
1. **Ask one question.** Pick an on-label question: a 10-minute walk after dinner vs. none, protein-first vs. carb-first meal order, a morning vs. evening oral GLP-1 dose. You pre-specify one primary outcome, such as the 2-hour post-meal glucose area, and the smallest effect you'd act on.
2. **Run-in.** Two weeks of plain CGM data estimate *your* day-to-day noise. The app tells you how many blocks you need, or that it can't be answered at your noise level.
3. **Seal.** The app generates a block-randomized ABBA schedule and commits to it with a timestamped hash. Each morning you learn only today's arm. Missed days stay missed (intention-to-treat), and the charts stay hidden, because the analyst you're blinding is you.
4. **Unseal.** A Fisher randomization test runs over the schedule actually used and stays valid despite CGM autocorrelation. You get an effect size with an interval, and "inconclusive" counts as a result.

**Why the collision works.** Dashboards invite forking paths: stare at spikes, find a story. CGMs give the rare self-tracking outcome that is dense, objective and hard to placebo. Randomization inference is the statistics that fits one person: it needs no population and no normality, only a coin you didn't control.

**Why now.** Orforglipron was approved April 1, 2026 at $149–349/month cash with no fasting rule, so dose timing becomes a free variable. OTC Stelo and Lingo make the outcome cheap. The Jan 6, 2026 FDA wellness guidance leaves room for behavioural experiments. The brief names this gap (§9, #9).

### Turn 2 — Builder
**What exists.** Levels and Nutrisense popularized CGM "experiments", but they are unrandomized before/after comparisons of single meals: the forking-paths dashboard Washout rejects. Signos and the Stelo and Lingo apps are dashboards with coaching. Zeevi et al. (*Cell*, 2015) and ZOE's PREDICT showed glycemic responses are personal, at population scale. Real n-of-1 tooling lives in research: StudyU, an open-source trial designer, and Trialist from the PREEMPT trial (Kravitz et al., *JAMA IM* 2018). PREEMPT is the honest warning: feasible, yet patients did no better than usual care. Nobody ships randomization inference to consumers.

**Hardest problem: the blindfold kills retention.** People buy a Stelo to look at it. Hiding glucose for six weeks fights the product the user paid for, and the vendor app shows the curve anyway, so the blind is fictional.

**Kill: drug-timing arms.** An app that schedules prescription dose times leaves wellness guidance for clinical decision support. Orforglipron's half-life of over a day also means flipping dose time daily barely moves exposure. Keep the GLP-1 as *context*: does protein-first still blunt your spike once the drug already slows gastric emptying? Nobody knows that per person.

**MVP.** iOS, glucose via HealthKit. Three templates (post-dinner walk, meal order, fibre preload), each on a fixed anchor meal so meal variance doesn't swamp the effect. Randomize *within* each sensor's wear period, because sensor-to-sensor offsets can rival the effects. Hash-commit the schedule and analysis plan; a push gives today's arm.

**Twist: seal the analysis, not the data.** Swap the blindfold for an anytime-valid test: a betting e-process over the known coin flips, valid however often you look. The user watches an **evidence wallet** that grows only if the effect is real, and the trial stops early at 20× (α = 0.05). Looking becomes the game, and the dashboard comes back.

### Turn 3 — Visionary
Both kills accepted; GLP-1 as *context* is the better question. The evidence wallet replaces the blindfold, and here it becomes the game.

**Mutation: you place the bet.** The unseeded domain is **Kelly betting**. At seal time, the user predicts their effect: "the walk cuts my 2-hour area by 15%." That prediction sets the betting strategy of the e-process. The coin flips are known, so the bet stays valid. Bold and right grows the wallet fast, bold and wrong bleeds it, timid crawls. Kelly growth is a log score, which makes honest prediction the optimal strategy. Users end up learning **calibration about their own body**: each finished trial plots what you guessed against what you got.

**Solving the retention problem.** Looking is now safe and fun. Ville's inequality allows optional stopping, so the user can open the app every day without inflating error. The vendor's curve stops mattering, because the wallet is the score. Caveat: peeking can still change behaviour, like trying harder on walk days. The trial measures "you doing X", motivation included, which is the real-life effect anyway.

**Answering PREEMPT: replication runs.** Stopping early at 20× inflates the effect estimate (the winner's curse). So every win comes with a short, freshly sealed **replication** three months later. Only effects that survive it become a **standing rule** on your profile. PREEMPT's patients got a report; these users get rules that keep being tested.

**Second-order effect: heterogeneity becomes measurable.** An ordinary RCT puts each person on one arm, so it can't separate "works for some" from "works a little for everyone." A thousand sealed n-of-1 trials on the same template can. A hierarchical model over them estimates the *spread* of responses, for questions like "does protein-first still help on orforglipron, and for whom?" No RCT can produce that dataset.

### Turn 4 — Builder
**Kills and fixes.**
- *The user's guess as the whole bet.* People are overconfident: a 40% guess against a real 10% effect bleeds the wallet into an inconclusive trial. Use a mixture, half the stake on the guess and half on a default grid. Still valid; the guess still gets scored.
- *Calibration as the headline.* At three or four trials a year, a personal calibration curve is noise; it's a footnote.
- *"No RCT can produce that dataset."* Repeated crossover series (Senn) can. And the pool is self-selected enthusiasts, so the spread it estimates is theirs.
- *Replication across titration.* Orforglipron escalates dose roughly monthly, so trials and replications must sit inside one dose plateau.

**Business model.** Stelo users already pay about $100/month; Washout is $9/month or $29 per sealed trial. The real channel is GLP-1 telehealth (Ro, Hims): discontinuation is their biggest cost, and personal rules are a bundleable retention product.

**Stack.** SwiftUI and HealthKit, Dexcom API as backup. A Python e-process engine, with schedules committed through OpenTimestamps. Stan for the opt-in hierarchical pool.

**Failure modes and safety.** Validity comes from the coin, not a glucose model, so autocorrelation and carryover can't break the null. Detect meals from the glucose rise when users forget to tap. Exclude insulin and sulfonylurea users, since meal-order arms risk hypoglycemia. Screen for eating disorders: GLP-1 plus glucose fixation is a real risk. Results read as "your result", never treatment advice. The FTC Health Breach Notification Rule applies, and pooling needs IRB-backed opt-in.

**90 days.** Weeks 1–4: app, run-in noise model, three templates, mixture e-process, validity simulations. Weeks 5–10: closed beta, 100 Stelo users, half on a GLP-1. Weeks 11–12: seal replications, pitch one telehealth partner.
**Kill metrics:** under 40% finish a trial, or under 25% of finished trials are conclusive.

### Turn 5 — Visionary
All four kills accepted. Senn is right: repeated crossovers already estimate response spread, so the pool's real edge is scale and cost, not novelty. The mixture bet scores the guess without letting it sink the trial. The 25%-conclusive kill metric is partly in our hands: the run-in power check should refuse unanswerable questions before anything is sealed.

**Final form.** Washout is a consumer n-of-1 trial app for CGM users, built for people on GLP-1s. You pick one on-label behavioural template (post-dinner walk, meal order, fibre preload) tied to a fixed anchor meal. A two-week run-in measures your noise and either sizes the trial or declines it. The app then seals a block schedule inside each sensor's wear period and one dose plateau, and commits it through OpenTimestamps. Each day's arm comes by push notification. The **evidence wallet** is a mixture e-process (half on your predicted effect, half on a default grid), safe to check daily and stopped at 20×. Wins earn a sealed replication three months later. Survivors become standing rules. GLP-1 telehealth bundles it for retention.

**Moonshot (2036): prescriptions that come with a test.** Utah already lets AI renew chronic prescriptions. A decade on, any renewal can carry an optional sealed personal trial, run under a clinician and an IRB-style protocol: dose timing, deprescribing, the add-on you've taken for years without knowing if it works. Templates and their pooled e-values sit in a public registry. "Who responds" becomes living evidence, not one subgroup table in a 2019 paper.

**Scores.** **Innovation 3.** This is a fresh assembly of known parts: crossover design, randomization inference, e-processes and OTC CGMs. The wallet is the one mechanic that is truly new to consumers. **Feasibility 7.** Simple stack, clear safety screens. Completion and conclusiveness are the real risks, and the kill metrics test both within 90 days.

### Turn 6 — Builder
I agree with innovation 3: this assembles known parts, and the one new piece for consumers is the evidence wallet, which makes looking at your data safe. Feasibility 7 also stands, because the stack is small and the safety screens are clear. The risk I'd watch is PREEMPT happening again: if the run-in power check turns down half the questions people care about, the honest answer may be that one body is too noisy for most behaviours. The kill metrics will tell us by week 12.
