# Idea 12 — Off-Diagonal
> Seed: 12. [bold] **Prediction markets × roguelite build-crafting × portfolio theory.** Your deck is a portfolio of event contracts. Synergies are correlations.

### Turn 1 — Visionary
**Working name:** Off-Diagonal, the covariance cells where synergy lives.

**Hook:** A deckbuilder where every card costs exactly its market price. Picking winners is worth nothing. Your only edge is knowing which events move *together*.

**Core loop.** The verb is **combine**.
1. **Slate.** A weekly run uses real listed events: a Fed decision, a CPI print, three matches, a heatwave. Each card is one contract, priced at the live Kalshi/Polymarket marginal. Play money only.
2. **Draft relics.** Relics are joint operators. *Chain* multiplies the payouts of tagged cards (a parlay). *Hedge* refunds one card when another fails. *Mirror* flips a card to its NO side.
3. **Price the build.** The house values your deck as if all events were independent. A deck of fair marginals has zero expected value, so the game pays out only on the **correlation you believed in and the market didn't price**.
4. **Settle.** Events resolve over the week, like fantasy sports. Bankroll carries across runs as Kelly-sized HP. A masked-history mode replays anonymized past slates for fast practice.

**Why the collision works.** Roguelite players are the best amateur combinatorialists alive, judging interactions among 30 items by feel. A Balatro flush build is a concentrated, correlated position; a "consistent" build is diversified. Players already feel Markowitz. Prediction markets have the opposite problem: they price marginals well and joints badly. Chen, Fortnow et al. (2008) showed combinatorial LMSR pricing is #P-hard, and Yahoo's 2010 Predictalot stayed a one-off. A deckbuilder could be the missing interface.

**Second payoff.** Aggregate the decks and you get a crowd-sourced **correlation matrix** across events, something no exchange publishes.

**Why now.** Kalshi and Polymarket did $44.8B in June 2026, and parlays are surging (brief §2). *Slay the Spire 2* peaked at 574K concurrent players. Cross-domain #11 names it. Play money keeps it out of the CFTC-versus-states war.

### Turn 2 — Builder
**What exists.** Correlation is already a mass-market game under another name. DFS players "stack" a quarterback with his receivers: a deliberately correlated portfolio. Sportsbook same-game parlays are the commercial joint-pricing engine, and they price correlation with a hold often above 20%. Kalshi and Polymarket now list combos and parlays. Metaculus runs conditional pairs, and Gnosis conditional tokens and Predictalot are the combinatorial precedents. Nobody has wrapped joint beliefs in build-crafting, and no correlation matrix across domains is public.

**Hardest problem: independence pricing is free money.** If the house prices joints as independent, the dominant strategy is to chain obviously linked events: hot CPI with a Fed hike, a heatwave with grid-stress contracts. That rewards reading headlines, not insight, and the crowd matrix collapses into the consensus everyone already knew. If the house uses a correlation model instead, players are only beating our quant. Noise makes it worse: a week of eight events gives roughly one draw of each joint, so skill takes dozens of slates to show.

**MVP.** A web and mobile game: a weekly slate of 12 events, marginals pulled from the Kalshi and Polymarket APIs, six relics, play money only, and seasonal leaderboards scored on log-wealth so variance is punished. Masked-history mode supplies the volume, from obscure past events nobody recognizes.

**Twist: the other players are the house.** Pin every marginal to the exchange price, then run each joint cell (CPI hot ∧ Fed holds, and so on) as a **pari-mutuel pool among players**. Relics are claims on joint cells, and a cell pays more the fewer players backed it. Obvious correlations get crowded and pay nothing. Your edge is the correlation you saw that *the field* missed. Each week's pool prices become the crowd matrix, a market-derived output rather than a survey.

### Turn 3 — Visionary
The pari-mutuel twist is the game: obvious stacks get crowded and pay nothing. Pushing it further turns the week into the spire.

**Mutation: every resolution is a floor.** Events resolve in calendar order: CPI Tuesday, Fed Wednesday, matches at the weekend. Each resolution is a fight. Between floors the shop reopens, and the remaining joint pools reprice *conditional* on what just happened. A run becomes eight decisions instead of one draft: bail, double down or hedge mid-week. Roguelite pacing on a real calendar.

**Solving noise: mark to the jump.** One binary draw per joint per week is hopeless. But when CPI prints, the exchange price of the Fed contract *moves* within minutes. That move is the market's revealed P(Fed | CPI) − P(Fed): a continuous, measured correlation signal. Floors settle **mark-to-market** on those jumps, and final payout still waits for actual resolution, the way a trader's daily P&L differs from what a bond pays at maturity. Feedback arrives in hours, variance drops sharply, and the jumps double as ground truth for masked-history mode.

**Unseeded domain: random matrix theory.** A crowd correlation matrix built from a few thousand players is mostly noise. Marchenko–Pastur gives the eigenvalue band that pure noise would produce. Anything above it is real structure the crowd agrees on. Clean the matrix that way before publishing it.

**Second-order effect: a coupling gauge.** In finance, correlations converge toward 1 in a crisis. Kritzman's absorption ratio, the share of variance in the top eigenvectors, rises before drawdowns. Run the same statistic on the cleaned crowd matrix and you get a weekly reading of how *coupled* players believe the world is: a VIX for cross-domain contagion, priced by gamers. When the field starts chaining elections to energy to rates, that reading is itself news.

### Turn 4 — Builder
**Kills and fixes.**
- *Marchenko–Pastur on a 12×12 matrix.* The bands are asymptotic; on a dozen events they're meaningless. Use Ledoit–Wolf shrinkage and bootstrap intervals.
- *The coupling gauge, for now.* Slates change weekly, so eigenstructure isn't comparable. It needs with a fixed **core basket** (rates, CPI, oil, S&P weekly direction) in every slate. Season two.
- *Mark-to-jump everywhere.* Thin Polymarket contracts jump on noise, and a Powell speech moves the Fed contract too. Mark only liquid pairs, inside 15-minute windows around *scheduled* releases (CPI, FOMC, payrolls).
- *Every joint cell.* Twelve events make 264 cells; empty pools pay nonsense. Curate about 20 pairs per slate.

**Business model.** Free to play, with a season pass and cosmetic relics. The money is licensing the cleaned crowd matrix to exchange combo desks, SGP pricing teams and macro funds. No affiliate links: that makes us a gambling funnel in states suing Kalshi.

**Stack.** React Native, Postgres, Kalshi WebSocket and Polymarket CLOB feeds, a pari-mutuel engine, a release calendar for the jump scorer, Python for shrinkage. Licensing price feeds for display is a real cost.

**Failure modes, abuse and ethics.** Sockpuppets crowd cells to poison payouts: phone verification, stake caps. No cash prizes and no currency for sale: skill contests and sweepstakes are regulated state by state, and California banned sweepstakes casinos in 2025. Ages 18+. Teaching parlay-love is the ethical cost; show the SGP hold on every Chain relic.

**90 days.** Weeks 1–4: engine, Kalshi feed, jump scorer for three release types. Weeks 5–8: 500-player beta drawn from Balatro and prediction-market Discords. Weeks 9–12: first published matrix with intervals; pitch one combo desk.
**Kill metrics:** week-4 retention under 20%, or top-decile players in weeks 1–4 don't beat the field in weeks 5–8. If skill doesn't persist, it's a lottery.

### Turn 5 — Visionary
All four kills accepted. Marchenko–Pastur on twelve events was physics the data can't support; Ledoit–Wolf is honest. The core basket gives season two its reason. The skill-persistence kill metric matters most: if last month's top decile doesn't beat the field this month, we've built a lottery with nice art.

**Final form.** Off-Diagonal is a free-to-play weekly roguelite on a real calendar. Each slate holds about 12 listed events with marginals pinned to Kalshi and Polymarket, and about 20 curated pairs whose joint cells run as player pari-mutuel pools. Relics (Chain, Hedge, Mirror) are claims on those cells, so crowded correlations pay little. Each scheduled release is a floor. Between floors the shop reopens at conditional prices, and liquid pairs mark to the 15-minute jump around CPI, FOMC and payrolls. Bankroll is scored on log-wealth across a season. Masked-history mode supplies volume. The product is a shrinkage-cleaned crowd correlation matrix with intervals, licensed to combo desks. Every Chain relic shows what a sportsbook would charge for the same parlay.

**Moonshot (2036): a crowd-built Bayes net of the world.** Pairs grow into conditional chains: "if the strait closes, then oil, then the Fed." Millions of players drafting relics build a living, public dependency graph across domains that no forecaster could afford. Combinatorial prediction markets finally work because the game supplies joint liquidity no order book attracts. The core-basket coupling gauge becomes a standard systemic-risk indicator next to the VIX: the crowd's belief in how connected the world is.

**Scores.** **Innovation 6.** DFS stacks and same-game parlays are close neighbours. Pari-mutuel joint cells, resolution floors, mark-to-jump scoring and a correlation matrix as the output are a new category. **Feasibility 6.** The engine is ordinary. Feed licensing, a weekly cadence that fights roguelite appetite, and the state sweepstakes patchwork are real drags.

### Turn 6 — Builder
I'm scoring innovation 5, not 6. Racetracks have run pari-mutuel pools on joint outcomes for decades in exactas and trifectas, and DFS stacking already turned correlation into a game. The new parts are applying that across domains, marking scores to the price jump at each resolution, and treating the correlation matrix as the product. That's bold, but not 6. Feasibility 6 stands: the engine is ordinary, and the real risk is the skill-persistence test. If last month's top decile doesn't beat the field this month, no one will license a matrix built from luck.
