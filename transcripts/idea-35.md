# Idea 35 — Lookahead
> Seed: 35. [mild] **Construction sites × digital twins × neighbors.** Transparency and compensation markets for people living next to a build.

### Turn 1 — Visionary
**Working name:** Lookahead

**Hook:** A noise forecast for every window next to a build, with automatic payouts whenever the site breaks it.

**The isomorphism.** Neighbours of a build live in an unpriced delay market, like air passengers before EU261. EU261 worked because compensation was automatic, priced in advance and triggered by data the airline already logged. Construction already logs that data. UK Section 61 consents require noise and vibration monitors at the site boundary. BS 5228 tabulates the noise of almost every piece of plant. The contractor's 4D model already knows which rig works where next Tuesday. Nobody joins the three for the person behind the window.

**Core loop. The verb is forecast, then settle.**
1. **Forecast.** Combine the contractor's three-week lookahead schedule, BS 5228 plant levels and a simple propagation model. Each registered address gets a forecast like a weather app's: "Tue 08:00–11:00, sheet piling, about 76 dB at your bedroom façade."
2. **Measure.** Existing boundary monitors, plus a few cheap façade sensors, check the forecast hour by hour.
3. **Settle.** The contractor pre-funds an escrow at mobilisation. Every hour over the consent limit, or over the forecast by more than 5 dB, pays the affected addresses automatically. The BS 5228 insulation-and-rehousing trigger, which fires on 10 days in any 15, becomes a hotel voucher you can see coming, not one you have to fight for.
4. **Price the exceptions.** A night concrete pour needs a dispensation. Instead of leaving it to a council officer's judgment, the contractor posts a price per household inside the forecast contour, and those households accept or decline.

**Why now.** Bedrock's retrofit kits target operator-less excavation in 2026 (brief §13), so night work gets cheap to supply. The binding constraint becomes neighbour consent, and that is the thing Lookahead prices.

### Turn 2 — Builder
**What exists.** Megaprojects already do this, slowly. HS2, Crossrail and Thames Tideway ran noise-insulation and temporary-rehousing policies with BS 5228 triggers and published monitor data, and Heathrow pays insulation by noise contour. Monitoring vendors such as Sonitus and Pulsar sell live boundary dashboards. Camden and other London boroughs require Construction Management Plans with community liaison and charge contractors monitoring fees. Commonplace runs the consultation. What's missing is the mid-size site, the five-storey block next door, where none of this reaches the neighbours.

**Hardest problem: the forecaster is the payer.** If payouts fire only when actual noise beats the forecast by 5 dB, the contractor forecasts 85 dB every day and never pays. An honest forecast is the expensive one. Façade sensors add a second problem: buses, dogs and the neighbour's drill all register. Attribution needs a correlation with the site's boundary monitor and a classifier for impulsive plant. Recordings at someone's window stay as levels and spectra only, never audio.

**Twist: pay on the forecast, settle the error.** Borrow the electricity day-ahead market. The contractor *buys* its forecast noise at a modest price per dB-hour per household, so padding the forecast costs money now. Then it pays an imbalance penalty, several times that price, for every dB-hour measured above the forecast. Truth-telling becomes the cheapest strategy. Neighbours are paid for planned disruption, not only for broken promises. The contractor gets something it values: a council can treat a clean imbalance record as grounds for faster Section 61 consents and dispensations.

**MVP.** One London borough and three mid-size sites. Inputs are lookahead exports from Primavera P6 or Asta, the BS 5228 plant tables, and ISO 9613-2 propagation. Thirty façade sensors, daily SMS forecasts, and an escrow paying per dB-hour.
**Kill metric:** under 70% of forecast hours land within ±5 dB.

### Turn 3 — Visionary
**Taking the day-ahead market, and letting households post the price.** A flat price per dB-hour assumes every hour hurts every household equally. A night-shift nurse sleeping at 2 p.m. and a family out at work and school are not the same. So each household chooses a tier for each block of the week: *don't care*, *standard*, or *protected*. Protected hours cost the contractor three times as much. Defaults apply when no one picks, which keeps it simple. The second-order effect is the real payoff. The 4D model stops being a forecasting display and becomes a **scheduler**: it sees a cost surface over time and moves the breaker to Thursday morning, when the street is empty. Neighbours get quieter hours where it matters, and that is worth more than any payout.

**Attribution without audio: let the machines confess.** Most mid-size plant already reports engine state through ISO 15143-3 (AEMP 2.0) telematics, and a $40 vibration tag covers the rest. Façade excess counts against the site only when its own plant is running and the boundary monitor rises with it. Buses and dogs don't line up with the breaker's duty cycle, and that correlation is cheap to compute and hard to argue with.

**Cumulative impact, borrowed from aviation's quota count (unseeded).** The real misery is three sites on one street at once, with each one individually within its consent. Heathrow gives every aircraft a Quota Count rating and the airport a seasonal budget. Do the same here: the borough sets a **street budget** of dB-hours, and concurrent sites draw on it and can trade allocations. That gives councils a cumulative-impact tool they currently lack. Over time, each address keeps a construction-exposure record, and contractors' imbalance histories become a considerate-contractor score built from measurements, not site-visit checklists.

### Turn 4 — Builder
**Kills and fixes.**
- *Self-declared protected hours.* When protection pays, everyone picks it for every hour. Give each household 20 **protected tokens** a week. Scarcity reveals real preferences, and equal allowances mean poorer streets aren't priced out of quiet.
- *Telematics as the confession.* The contractor controls the tags, and handheld breakers carry no telematics at all. So the burden shifts: an untagged or silent machine during a façade excess counts as the site's. Only the operator installs sensors, so nobody farms payouts with a speaker at the window.
- *A tradable street budget.* Councils set conditions site by site under COPA s61, so a traded cumulative cap has no clean legal basis yet. Year two brings linked consent conditions in one borough; v1 just maps concurrent sites together.

**Hard lines.** Payments never buy exceedance of consent limits. Taking a payout never waives nuisance or statutory rights, and that is written into the scheme. Money goes to occupants, not landlords. Exposure records stay private, since publishing them would hit property values.

**Money.** Developers already fund community impacts through Section 106 and Construction Management Plan (CMP) fees, so the escrow sits there. A 12-month mid-size build might put £30–60K through it. One week stopped by a Section 60 notice can cost about as much. Lookahead charges £1,500 per site-month plus 3% of payouts.

**Stack.** P6/Asta XML import; BS 5228 Annex C tables; NoiseModelling (open-source, CNOSSOS/ISO 9613-2); Class 2 façade sensors on LoRaWAN; AEMP 2.0 feeds; TimescaleDB; payouts through an e-money partner.

**90 days.** Weeks 1–4: forecast engine and backtest against one finished site's monitor logs. Weeks 5–8: three Camden sites, with forecasts but no money. Weeks 9–12: escrow live, tokens, first settlements.
**Kill metrics:** forecast hit rate under 70% within ±5 dB, or no contractor reschedules a single task because of the cost surface.

### Turn 5 — Visionary
**Final form.** Lookahead gives every occupant next to a mid-size build a noise forecast for each façade. The contractor's P6 or Asta lookahead runs through the BS 5228 plant tables and NoiseModelling to produce an hourly forecast per address, sent by SMS. Each forecast dB-hour is bought from occupants at a day-ahead price. Hours a household marks with one of its 20 weekly protected tokens cost three times as much. Measured hours above the forecast pay an imbalance penalty, judged against operator-installed sensors. Untagged or silent plant during an excess counts against the site. The escrow is funded through the existing CMP and Section 106 process. Payouts never buy a consent breach or waive a right. The product is the cost surface: the metric is the breaker moving to Thursday. I accept every kill. Tokens beat my tiers, because quiet becomes an equal allowance, not a function of income.

**Moonshot (2036).** **Lane rental for ears.** London already charges utilities by the hour to occupy busy roads. Lookahead generalises that to every urban disruption: roadworks, data-centre chillers, festivals and wind-farm construction all draw on one neighbourhood ledger of dB-hours. Autonomous night excavators' schedulers bid directly against the street's cost surface, and robot shifts land in the hours residents value least. Consent law catches up. A Section 61 consent becomes a forecast with an imbalance bond, and linked street budgets make cumulative impact enforceable.

**Scores.** Innovation **3**. Insulation-and-rehousing policies, monitoring dashboards and lane rental all exist. What's new is the mid-size scope, day-ahead pricing with an imbalance penalty, and scarce protected tokens feeding a scheduler. Feasibility **7**. The inputs are standard files and standard tables, and the legal home (CMP and Section 106) already exists. The pace-setter is one borough willing to make a forecast count toward consent.

### Turn 6 — Builder
Innovation 3 stands. Feasibility comes down to 6. The software is ordinary, but each site needs three parties to agree: a borough, a developer willing to fund the escrow through Section 106, and a contractor willing to forecast honestly in public. Section 106 negotiations run at the speed of planning. The idea's real strength is the day-ahead price plus the imbalance penalty: it makes an honest forecast the cheapest one, and it turns compensation into a scheduling signal rather than an apology. If no breaker moves to Thursday in the pilot, it is just a nicer complaints line.
