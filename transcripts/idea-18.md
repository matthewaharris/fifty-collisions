# Idea 18 — Table Stakes
> Seed: 18. [mild] **Small-group futarchy × households/teams/clubs.** "Vote on values, bet on beliefs" for a family or startup.

### Turn 1 — Visionary
**Working name:** Table Stakes

**Hook:** Futarchy for the kitchen table. The household votes once a quarter on what "better" means, then settles every big decision by staking chores on what each option will do to that score.

**Why the collision works.** Futarchy has two problems: thin markets and fuzzy welfare metrics. A small group solves the second, since a family can agree on three numbers. Hanson already solved the first: his logarithmic market scoring rule (LMSR) works with a single trader because the market maker always takes the other side. And chores are a currency every household already argues over. They carry real stakes but no money, so no gambling law, and every member, kids included, is equally rich in them.

**Core loop.** The verb is **stake**.
1. **Values (the only vote).** Each quarter the group picks two to four metrics and their weights. A household might pick savings rate, a weekly happiness pulse and evenings together; a startup, MRR, retention and burn.
2. **Propose.** Anyone can post a decision with options and a horizon: dog or no dog, Lisbon or stay, hire A or B.
3. **Bet.** Each member trades a conditional LMSR market per option ("the scoreboard in six months if we get the dog"), paying in chore-hours or team points. The house pool supplies the subsidy.
4. **Decide.** The branch with the best forecast wins unless it breaks a hard constraint set in the values round. Trades in the losing branches are refunded.
5. **Settle.** At the horizon, the chosen branch settles against the measured scoreboard, and dish duty changes hands.

**Why now.** MetaDAO proved conditional markets run but saw revenue fall about 70% and pivoted, and the brief names small-group futarchy as largely unbuilt (§2). Kalshi parlays have made conditional bets a mass-market idea.

### Turn 2 — Builder
**What exists.** Google, Ford and HP ran internal prediction markets in the 2000s, and Cultivate Labs still sells them. Most died of thin trading and managers who ignored the prices. Manifold lets friends spin up play-money markets in minutes. Optimism ran a play-money futarchy pilot for grant allocation in 2025. Chore apps like OurHome and Sweepy already price chores in points. Nobody has joined the three.

**Hardest problem: the traders are the outcome.** Public futarchy works because traders can't move the metric. In a family of four, everyone can. Whoever bets against the dog can skip the walks, and if happiness is a self-reported pulse, she can just report lower happiness. Aggregation is thin too: four people's beliefs are already in the conversation. The market's real value is commitment, and a running score of who was right.

**MVP.** Start with startups, as a Slack bot. Metrics come from Stripe and the product-analytics API, never self-report, and decisions arrive weekly. Points can't be converted, so there's no bonus or gambling question. Households follow as a WhatsApp bot that pays in chore-hours, using passive metrics where possible: savings rate via Plaid, evenings together from the shared calendar.

**Twist: the pessimist owns it.** When a branch wins, the member who priced it lowest becomes its steward: they walk the dog or run the new hire's onboarding. The steward earns a bonus for every point the score rises, sized to exceed the most their bet can lose. Sabotage now costs them, and making it work pays. It's "disagree and commit", with a price on it. The doubter ends up with the strongest reason to prove the market right, and the group learns whether a committed skeptic can beat a forecast.

### Turn 3 — Visionary
Pessimist-steward taken: it turns moral hazard into a job offer. Two gaps remain.

**The exploit is self-limiting.** If stewardship pays more than the bet can lose, members will lowball a branch to win the job. Measure the steward's bonus against the *market's* price, not against their own forecast. Lowballing then becomes an honest bid: "I'll beat consensus, and my stake pays if I don't." It also lowers that branch's price, so it makes the job you want less likely to exist. Sealed positions, revealed only after the decision, stop people betting to please Dad and stop sabotage aimed at a known doubter.

**Unseeded domain: peer-prediction psychometrics.** The happiness pulse is the one metric each member authors. Prelec's Bayesian Truth Serum scores a report against your prediction of everyone else's reports, rewarding answers that are "surprisingly common", and makes honesty the best strategy with no ground truth. Witkowski and Parkes's robust variant (2012) works with as few as three respondents, which is a family. Each Sunday you give your own score and guess the others'. Honest pulses earn chore-hours. A sandbagged pulse after the dog arrives loses.

**Second-order effects.**
- **A calibration ledger by domain.** After a year the household knows Mum is calibrated on money, Leo on fun and Dad on nothing. The app can weight forecasts by domain, since calibration isn't a value and needs no vote.
- **The commitment premium.** Every steward's result minus the market price measures how much effort, not foresight, moved the outcome. Teams learn which decisions are forecastable and which are won by someone deciding to win them. That may be the best evidence on futarchy anyone has, gathered in a kitchen.
- **Proposals as recruitment.** Members start posting decisions to draw a particular skeptic into stewardship.

### Turn 4 — Builder
**Kills and fixes.**
- *Continuous LMSR.* With four traders, every price move tells everyone who traded, so sealed positions can't coexist with a live market. Replace it with **sealed forecast rounds**: each member submits a probability per branch, scored by the log rule and pooled by mean log-odds. The incentives are the same, and a nine-year-old can follow it.
- *The steward's benchmark.* One lowballer drags a four-person price down, then beats the low bar. Benchmark against the pooled forecast *excluding the steward's own*.
- *BTS on a 1–10 scale.* Robust BTS works for three respondents only with binary signals, and it collapses if the family coordinates. Ask "better or worse than last week?", pay a token amount, and never let the pulse carry more than a third of the scoreboard.
- *Calibration from ten decisions a year.* Too few. Add weekly practice questions ("Will we leave by 8:15 Monday?"), which also bring people back between decisions.

**Abuse and ethics.** Chore debt is a tool of coercive control in the wrong household. Hold a **quarterly jubilee** that clears all balances, never let minors go negative, and let any adult member dissolve the group with one tap. Goodhart: a team pulls MRR forward to win. Pair every metric with a lagging one, such as 90-day retention against MRR.

**Business.** Teams pay $3 per seat per month, priced like Geekbot. Families get a free tier, since household apps monetize badly.

**Stack.** Slack Bolt, the WhatsApp Business API, Stripe, PostHog, Plaid and Postgres.

**90 days.** Weeks 1–4: a Slack bot with sealed rounds and metric connectors for 10 startups. Weeks 5–8: stewardship, practice questions, and 30 families on WhatsApp. Weeks 9–12: measure.
**Kill metrics:** under half of teams post a second decision within 30 days, or families report more conflict.

### Turn 5 — Visionary
All kills accepted; the jubilee belongs in the core, not the safety appendix. One pushback: "pool excluding the steward" breaks for couples, the commonest household, leaving one forecast the partner can sandbag. With fewer than three other forecasters, benchmark against the group's **base rate**, its median past outcome on that metric. Nobody can lowball history.

**Final form.** Table Stakes is futarchy for groups of three to thirty.
- **Values.** A quarterly vote on two to four metrics, each paired with a lagging guard, filled from Stripe, PostHog, Plaid and the calendar. The pulse ("better or worse?") is capped at a third of the score.
- **Beliefs.** Sealed forecast rounds per branch, scored by the log rule and pooled by mean log-odds, with weekly practice questions in between.
- **Commitment.** The member who priced the winning branch lowest becomes its steward, benchmarked against the others' pool or the base rate.
- **Currency.** Points for teams, chore-hours for households, a quarterly jubilee, and no negative balances for minors.
- **Ledger.** Calibration by member and domain, plus a running **commitment premium**: outcome minus forecast for each steward.

**Moonshot (2036): a map of which decisions are forecastable.** Ten years of commitment premiums across a million teams and households answer a question futarchy never could: which decisions are settled by foresight, and which by someone deciding to win them? Co-ops, school boards and municipal budgets then send forecastable choices to markets and will-driven ones to stewards. Futarchy grows from the kitchen up, not from the nation-state down.

**Scores.** **Innovation 3.** Known parts (log scoring, chore points, Slack bots), but the pessimist steward and the commitment premium are new mechanics. **Feasibility 7.** Every connector exists today. The open risk is whether groups keep posting decisions after the novelty fades.

### Turn 6 — Builder
Base rate for couples is right, and it also gives every new group a benchmark before any forecasts exist. Innovation 3 stands. The parts are textbook, but the pessimist steward, paid to beat a forecast they doubted, is the one mechanic here I haven't seen anywhere, and the commitment premium is a real measurement. Feasibility 7 stands too. It's a Slack bot with connectors. The risk is retention, not engineering: most groups make too few big decisions for a habit, so the practice questions carry the product between them.
