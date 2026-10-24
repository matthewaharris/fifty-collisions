# Idea 32 — Ascension
> Seed: 32. [bold] **Roguelite × personal finance × behavioral economics.** Your actual month, played as a run.

### Turn 1 — Visionary
**Working name:** Ascension

**Hook:** Your month as a Slay the Spire run on a real debit card: relics are enforced rules, the death screen names the bias that killed you, and every win raises your savings rate.

**The isomorphism.** Behavioral economics already designed a roguelite. The **fresh start effect** (Dai, Milkman & Riis) is the run reset. **Commitment devices** are relics. **Save More Tomorrow** (Thaler & Benartzi) is the Ascension ladder: people refuse to save more today but accept automatic escalation later, exactly how players opt into harder Ascension levels one at a time.

**Core loop. The verb is draft.**
1. **Draft (payday).** Pick one of three relics. Each is a rule enforced at card authorization. *Payday Moat:* discretionary swipes over $50 wait 48 hours after payday. *Windfall Jar:* refunds and gifts get swept to savings, which fights mental accounting. *Latte Tithe:* every third café swipe sends the same amount to your goal. Relics stack like Balatro jokers.
2. **Map.** The calendar becomes the map. Rent is the boss. Irregular bills mined from last year's transactions are elites. Social invites are "?" nodes where you choose your path.
3. **HP is runway in days**, not dollars. Each purchase costs days of freedom.
4. **Death screen.** Finish below your floor and you get a cause of death with evidence: "Payday effect: 41% of discretionary spending within 72 hours of the deposit." That diagnosis unlocks a counter-relic for the next run.
5. **Ascend.** Each run you survive adds 1% of pay to your automatic savings. Losing never moves you down.

**Why now.** Slay the Spire 2's record 574K concurrent players made relics and Ascension a mass literacy (§4). AP2 mandates (§1) mean relics can bind the purchases your shopping agent makes for you too.

### Turn 2 — Builder
**What exists.** The relics exist already, without a game around them. Qapital's "Guilty Pleasure" rule is the Latte Tithe, and it has run since 2015. YNAB's "Age of Money" is HP measured in days. 401(k) auto-escalation is Save More Tomorrow in production. Long Game gamified savings with prize draws and was absorbed by Truebill. Digit was fined $2.7M by the CFPB in 2022 because its automatic transfers overdrew customers. That fine is the warning label on "losing never moves you down."

**Hardest problem: the game can't bind anyone.** A relic enforced at card authorization covers one card, and a present-biased player taps the other one. Commitment devices with free exits do little, and the people who need them most are the least likely to pick binding relics. Issuing your own card also means a sponsor bank, and banking-as-a-service has been under regulators' scrutiny since Synapse collapsed in 2024.

**Twist: enforce at the source.** Every card is paid from the paycheck, so the game should sit upstream of the cards. A deposit switch (Atomic or Pinwheel) routes pay into the game's FDIC-insured account. The run's gold is what the game releases into checking over FedNow, on the schedule the relics define. *Weekly Stipend* cuts pay into four draws, which flattens the payday effect for every card at once. Escape stays instant and costs no money: an emergency draw puts a **Curse** in your deck for the rest of the run. That hits score, never fees, and the death screen counts every curse. The game never sweeps a dollar it can't prove is surplus, because the Digit lesson is a hard rule.

**MVP.** Plaid read-only plus a deposit split of 20% of pay into a partner account. Three release relics, one curse type, and a month-end death screen.
**Kill metric:** fewer than 40% of players start a second run.

### Turn 3 — Visionary
**Taking the twist: the game is a reservoir.** Once pay flows through the game, the model isn't a budget but dam operation (unseeded: hydrology), a century of practice releasing lumpy inflow under uncertainty. A paycheck is the snowmelt pulse, checking is the river downstream, and the game account is the reservoir. Releases follow a **rule curve**, which keeps flood space empty ahead of forecast elites: car registration, insurance renewal, December. Like Lake Mendocino's forecast-informed operations (FIRO), releases adjust to your mined bill calendar. That also answers Digit precisely: "surplus" is whatever sits above the rule curve, and nothing is swept below it.

**Binding, paid for with yield instead of willpower.** Each run opens with a Neow-style trade. *Lock this tranche for 21 days and it earns the account's full yield. Leave it liquid and it earns nothing.* Emergency draws stay instant but carry a Curse. The evidence supports this: in the SEED trial (Ashraf, Karlan & Yin, 2006), only about 28% took the locked account, yet the group offered it saved 81% more than controls. Not everyone needs to bind; binding needs to be a draftable option with a price.

**Push further: the real audience has irregular income.** Salaried players only flatten a payday spike. Gig drivers, freelancers and tipped workers get a rain-fed river. For them the reservoir pays a **synthetic salary** out of chaotic inflow, and the run is about keeping that salary alive. That game is bigger and more needed.

**Second-order effect: drought warnings that beat payday lenders.** The rule curve shows the low-water day weeks ahead. Instead of the player finding it at a payday-loan counter, the game releases a bridge early and shows it as a boss foreseen. Where the need for a payday loan is *predictable*, the reservoir replaces it.

### Turn 4 — Builder
**Reservoir and rule curve stay. Kills:**
- *Yield as the price of binding.* $800 locked for 21 days at 4% earns $1.84. Nobody drafts for that. Pool the yield into **prize-linked savings** instead: each locked tranche earns tickets in a monthly draw. Save to Win credit unions proved it works, and a 2014 federal act makes it legal with a sponsoring bank or credit union. Cents become a jackpot.
- *The bridge.* Releasing money before the inflow arrives is lending, and advance rules differ by state. In v1 the bridge comes only from the player's own locked tranches, unlocked early with a Curse.
- *Synthetic salary from day one.* An empty reservoir can't smooth anything. Gig players get a fill phase of six to ten weeks. Forecast errors cost asymmetrically: under-release means missed rent. So the rule curve sets a release floor, and an on-demand override is always instant.
- *Diagnosis from one run.* That's n=1 storytelling, so the death screen labels the cause a hypothesis until three runs agree.

**Gig is the better wedge.** Platform payouts already go to any account number, so there's no payroll switch to negotiate.

**Money.** Low-income players shouldn't pay. Sell to gig platforms as driver retention, and to employers as a financial-wellness benefit, per active player per month.

**Stack.** A sponsor bank with a direct ledger such as Column;, since Synapse died of reconciliation. Plaid for reading; FedNow/RTP for releases; quantile forecasts over mined bill calendars; React Native.

**Ethics.** No paid items, no streaks that punish essentials, no lending upsell.

**90 days.** Weeks 1–4: bank partner, ledger, rule curve. Weeks 5–8: 300 drivers through one gig cooperative, fill phase. Weeks 9–12: synthetic salary live, first prize draw.
**Kill metrics:** over 5% of releases overridden in emergencies, or fewer than 40% start a second run.

### Turn 5 — Visionary
**Final form.** Ascension is a roguelite on your real income, gig workers first. Payouts land in a sponsor-bank reservoir. A **rule curve**, forecast from your mined bill calendar, releases a **synthetic salary** to checking over FedNow. It keeps a release floor and instant override. Each month is a run: draft relics (release and lock rules); rent is the boss, forecast irregular bills are elites, and HP is runway in days. Locking a tranche earns **prize-linked tickets**, so pooled yield becomes a monthly jackpot. An early unlock or an emergency draw puts a Curse in your deck and never costs a fee. The death screen offers a cause of death as a hypothesis until three runs agree. Each run you survive climbs one Ascension and raises the locked share by 1%. Platforms and employers pay per active player; players never do. I accept every kill. Prize-linked savings is better anyway: lotteries are what present-biased people actually draft.

**Moonshot (2036).** Reservoirs join into a **watershed**. Gig workers' income shocks are only partly correlated, so a cooperative of linked reservoirs pays each member a steady salary on far less buffer per head than anyone needs alone. That is income insurance run as a member-owned water district, with the rule curve as its actuarial model. The game also becomes behavioral economics' largest consenting field lab: every new relic ships as a preregistered randomized trial, tested on millions of real months instead of student lab sessions.

**Scores.** Innovation **5**. Qapital rules, Steady-style smoothing and Save to Win all exist. New: a hydrology rule curve driving a roguelite on top of paycheck routing, with binding sold as a drafted, prize-priced choice. Feasibility **5**. The software is ordinary; the bank partner, state advance rules and a willing gig cooperative set the pace.

### Turn 6 — Builder
Innovation 5 and feasibility 5 stand. The pieces are familiar. What's new is putting the game upstream of every card, treating the paycheck as a reservoir run on a rule curve, and selling commitment as a drafted, prize-priced relic rather than as willpower. My one change to the plan: start with a credit union rather than a fintech sponsor bank. Credit unions already run prize-linked savings, often serve drivers, and would carry the bank partner and the jackpot's legality in a single deal.
