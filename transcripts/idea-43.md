# Idea 43 — Fountain Swap
> Seed: 43. [wild] **Longevity / epigenetic clocks × financial instruments.** Bonds and contracts that pay on biological age reversal.

### Turn 1 — Visionary
**Working name:** Fountain Swap

**Hook:** Pension funds pay for the cure that would bankrupt them, because that is their cheapest hedge.

**The isomorphism.** Rejuvenation has two natural counterparties who've never met. Pension funds and annuity writers are short human lifespan; every added year is unreserved liability. Reprogramming biotechs are long it. Longevity risk transfer already exists. Swiss Re's Kortis bond (2010) paid on divergence in mortality trends. But these instruments settle on deaths, decades after the biology has changed. Epigenetic clocks are the leading indicator.

**Core mechanic. The verb is swap.**
1. **Protection buyer.** A pension fund pays a premium now. It receives a payout if a preregistered rejuvenation trial slows or reverses biological ageing against its control arm, measured by a locked panel of clocks (DunedinPACE, GrimAge, organ-specific clocks).
2. **Protection seller.** The biotech running the trial collects the premiums as non-dilutive funding. It pays out only in the world where it succeeds, when it can most afford to.
3. **The surrogate spread.** No clock is a validated surrogate endpoint, and paying on clocks invites Goodhart. So every contract has two tranches. One settles on clocks in year three. The other settles on morbidity and mortality in year fifteen. The gap between their prices is a market estimate of how much one point on the clock is worth in survival, a quantity the FDA has never been able to measure.

**Why wild.** It's a new institution: pensions underwriting the end of their own actuarial tables, plus a traded exchange rate between a biomarker and death.

**Why now.** NewLimit raised $435M (June 2026) for human trials in 2027. Life Biosciences has Phase 1 approval for partial reprogramming. Prediction markets cleared $44.8B in June.

### Turn 2 — Builder
**What exists.** Longevity risk transfer is large: BT Pension's £16bn swap (2014), q-forwards since JPMorgan's LifeMetrics, Kortis. It's also a warning. The 2004 EIB/BNP longevity bond failed to sell because its index didn't match any buyer's liabilities. Royalty Pharma shows that biotech cash flows can be securitised, and Lo's "megafund" research-backed obligations pooled trial risk. Nobody settles on clocks.

**Hardest problem: the hedge doesn't hedge.** A pension's liability is its own members dying in the 2040s. One 300-person liver trial reading out in 2030 barely moves that, so trustees would be buying a speculation they can't justify under fiduciary rules. The seller is also wrong. A cash-poor biotech can't post collateral for a payout due in its success scenario. Goodhart is worse than you priced it. Partial reprogramming acts *directly on methylation*, so for this drug class methylation clocks are the most gameable endpoint there is. The clock tranche needs orthogonal measures: plasma-proteomic organ clocks (Oh et al., 2023), plus grip strength and VO₂max.

**Twist: settle the success leg in royalties.** If the trial beats control, the pension doesn't receive cash. It receives royalty units on the therapy. That turns the speculation into a natural hedge: if rejuvenation reaches the market and extends members' lives, the fund owns a slice of the revenue doing it. The biotech posts no collateral; it gives up future royalties only in the world where they exist. And the surrogate spread gets sharper, because the units trade, and their price tracks a clock point in dollars of survival.

**First step on Monday.** Price the spread with no deal at all. Biobanks with methylation, proteomics and 15 years of deaths (Generation Scotland, UK Biobank) let actuaries fit and bet on clock-to-mortality exchange rates, scored on held-out cohorts.

### Turn 3 — Visionary
**Taking royalties and pooling them.** You're right that one liver trial is a speculation, not a hedge. The fix is to sell the pension the whole pipeline in one contract. A **Fountain Pool** collects royalty options from every rejuvenation programme that will contribute. It works like Lo's megafund, in reverse: the buyer is hurt by success. The pension pays premiums into the pool, and each biotech draws non-dilutive funding in exchange for royalty units that only vest if its trial wins. The fund's liability depends on whether *any* therapy reaches mass use, and so does the pool. That's the basis match the EIB bond never had.

**Solving Goodhart (unseeded domain: anti-doping science).** WADA's Athlete Biological Passport doesn't test for drugs. It flags any longitudinal pattern of markers that physiology can't produce on its own. Fountain treats "painted" methylation the same way. Each participant's methylation, proteomic organ clocks, grip strength and VO₂max are tracked as one Bayesian profile. A clock that drops while the other markers stay flat is flagged. Then the settling endpoint is drawn at random from the full panel, using a seed committed before unblinding. A therapy that only rewrites methylation then loses about five draws in six, so the only reliable way to win is whole-body change.

**Gompertz, split.** Mortality doubles roughly every eight years. Interventions that reset the intercept (a one-time rejuvenation) and interventions that bend the slope (slower ageing) create very different liabilities. Issue separate *intercept units* and *slope units*. Their spread is a priced estimate of whether we've been rejuvenated once or whether ageing has been slowed for good.

**Second-order effect.** Once slope units trade, there's a market-implied forecast of future lifespan. Governments could peg the retirement age to it rather than to lagging life tables.

### Turn 4 — Builder
**First buyer: not pensions.** Longevity reinsurers go first: RGA, Hannover Re and Swiss Re carry billions in longevity exposure and can take a view. Buy-in insurers come next, and pension trustees last.

**Two fixes to the pool.** *Royalty per dose, not per dollar.* A percentage royalty pays most when the therapy is expensive, which would push pensions to lobby for prices their own members can't afford. The liability grows with *uptake*, so royalties should be paid per dose. *The patent cliff.* Royalties end about 12 years after launch, while extended lives keep paying out for decades. At expiry, royalty units convert into a q-forward on the population mortality index, so the hedge ends where the old instruments begin.

**Kill: slope units before year 15.** A three-year trial can't tell an intercept reset from a bent slope. Intercept units only until the mortality tranche matures.

**Hardening the passport.** The random draw multiplies sample size, because every endpoint must be powered. So settlement happens on phase 2b or on pooled readouts, never on phase 1. The draw uses a drand beacon round fixed in the preregistration. Insider trading on unblinded data is the big abuse. Royalty units are securities: they need SEC/FCA treatment, sponsor lockups, and an independent committee holding the seed.

**Model.** A Royalty Pharma-style manager: 1% plus carry on the pool, plus a Fountain Index of clock-to-mortality exchange rates licensed to actuaries.

**Stack.** Hierarchical Bayesian passport in Stan, EPIC v2 methylation, Olink proteomics, drand commitments.

**90 days.** Days 1–30: fit exchange rates on NHANES methylation clocks linked to mortality (public now). Days 31–60: test the passport against simulated "painted" profiles and mouse reprogramming data. Days 61–90: term sheets with one reinsurer and two biotechs. **Gate:** exchange rates replicate in Generation Scotland within 25%.

### Turn 5 — Visionary
**Final form.** Fountain Swap is a pool of royalty options on rejuvenation therapies. Reinsurers buy first, then buy-in insurers, then pension trustees. Each biotech draws non-dilutive funding and gives up royalty units that vest only if its phase 2b or pooled readout beats control. Royalties are paid **per dose**, so the hedge rewards uptake, not price. When a patent expires, its units convert into q-forwards on the population mortality index. Settlement uses a biological-passport model over methylation, Olink organ clocks, grip strength and VO₂max. The endpoint is drawn from a drand round fixed at preregistration, and an independent committee holds the seed. Alongside it, the **Fountain Index** publishes clock-to-mortality exchange rates. I accept every kill, including deferred slope units. Per-dose pricing is the best change in this thread: pensions end up lobbying for cheap cures.

**Why the tier holds.** It's still a new institution: the parties who lose when people live longer pay for the research and hold its upside. It also runs a market whose price is the validity of a surrogate endpoint.

**Moonshot (2036).** The Fountain Index is the reference rate actuaries put into their life tables, and Solvency regimes accept it. Once it has been validated against fifteen years of deaths, regulators can use the same exchange rate to recognise ageing as an indication. Slope units trade, and retirement ages in several countries are indexed to them. Ageing societies' largest fiscal fear, people living longer, is pre-funded by the institutions that feared it most.

**Scores.** Innovation **8**. Longevity swaps, Royalty Pharma and megafunds exist, but nobody prices a biomarker's validity or makes pension money finance the cure. Feasibility **3**. The index can be fitted on NHANES on Monday. The first live deal waits for phase 2b readouts, securities law and a reinsurer willing to go first.

### Turn 6 — Builder
**Closing remark.** Fountain Swap holds 8/3. The strange core survived intact: the parties who lose when people live longer finance rejuvenation and own a slice of it, and a market prices whether a biomarker means anything. Per-dose royalties turned the pensions' incentive from a conflict into an alignment. The Fountain Index is the part I'd start on Monday, because it's useful even if no swap is ever signed. The honest risk is time: the first live settlement waits on a phase 2b reprogramming readout that may be five years out, and on a reinsurer willing to be first.
