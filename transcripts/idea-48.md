# Idea 48 — Taper
> Seed: 48. [mild] **EV charging × queue etiquette × micro-markets.** Fix the charger-line problem with mechanism design.

### Turn 1 — Visionary
**Working name:** Taper

**Hook:** Voluntary bumping for fast chargers. When a line forms, the car charging slowest gets paid to leave, and the driver at the front of the line pays.

**The isomorphism.** Charger lines are an airline overbooking problem in disguise. DC fast charging tapers: 80% to 100% can take as long as 10% to 80%. A car at 88% holds a scarce stall while drawing a fraction of its rated power. The etiquette ("unplug at 80% if people are waiting") is unwritten, unenforced and resented on both sides. In 1968 Julian Simon proposed the fix for overbooked flights: auction the right to leave. US airlines adopted it after 1978, which is why gate agents offer vouchers instead of arguing.

**Core mechanic. The verb is *yield*.**
1. **Line detection.** Drivers check in by geofence when every stall is taken. With no line, there is no market and nobody pays anything.
2. **Taper ranking.** The charger session (OCPP 2.0.1 with ISO 15118 state of charge) shows which plugged-in car is drawing the fewest kW. That car is offered first.
3. **Rising offer.** A clock auction: the offer to leave climbs in small steps until a charging driver accepts, for example "unplug within 3 minutes for $2.40." The first waiting driver pays, up to a cap they set. They can always decline and wait for free.
4. **Settlement.** A per-session micropayment through the operator's app. The departing driver also gets a pass to come back and finish later.

**Why the collision works.** The etiquette already exists. Taper only puts a price on it, so a favour becomes a trade where both sides gain.

**Why now.** NACS opened Superchargers to other brands, which mixed charging curves in the same lines. x402 shows micropayments between strangers are routine (87% are under $0.10). AP2 mandates let a car's agent bid within limits its owner sets.

### Turn 2 — Builder
**What exists.** Tesla already caps cars at 80% at busy Superchargers and charges congestion fees of up to $1 a minute above 90%. Electrify America charges idle fees after a grace period. ChargePoint's Waitlist queued workplace chargers years ago. Nobody pays a driver to leave; Simon's auction never reached a charger.

**Hardest problem: pricing the favour kills it.** When an Israeli daycare fined late pickups, lateness rose (Gneezy and Rustichini, 2000): the fine turned a duty into a price. Taper does the same. A driver who used to unplug at 80% out of courtesy now sits at 88% waiting for an offer, and lingering becomes a way to earn money. Airline passengers can't manufacture overbooking.

**Twist: rule the taper, price the bulk.** Split the session at each car's knee: the point where its live kW drops below half the stall's rating, read from DC charging-status messages. *Past the knee*, the etiquette becomes a rule. When a line exists, the car gets a three-minute nudge, then a Tesla-style congestion fee, and no payment ever. That removes the windfall for lingering. *Before the knee*, the market runs. The waiting driver bids for a car still charging efficiently to cut its session short, at 60% instead of 80%. That's a real sacrifice worth pricing, and the bumped driver also gets a priority pass.

**The operator already wants this.** A car tapering from 80% to 100% might draw 40 kW while the car in line would take 150 kW or more. Faster turnover roughly triples the stall's revenue per hour, so the operator can co-fund the bids from the extra kWh.

**MVP shape.** OCPP 2.0.1 middleware on an open CPMS such as CitrineOS, piloted at one network's two or three sites with the worst queues. Measure stall kWh per hour and queue wait.

### Turn 3 — Visionary
**Taking the knee.** The knee rule fixes Gneezy: past the knee, lingering earns nothing, and the courtesy is backed by a rule rather than replaced by a price. The bulk market still has two problems. It's thin, because it needs a car that's still charging efficiently *and* a waiting driver willing to pay. And it's face to face: a stranger at the next stall buying your last 20% feels like a shakedown. The cheapest car to move isn't in the stall at all. It's forty kilometres up the road.

**Unseeded domain: air traffic flow management.** Airports stopped stacking planes in holding patterns in the 1990s. Ground Delay Programs hold flights at the origin with assigned arrival slots, and under Collaborative Decision Making airlines swap and substitute those slots. Nav systems already know which cars are routed to Kettleman City and when they'll arrive, so the Thanksgiving line is predictable half an hour out. Taper meters upstream. A car heading into a forecast line is offered a quieter site 15 km earlier, plus a **slot credit**: priority at its next congested stop. It's a seven-minute detour instead of a 25-minute wait, and nobody negotiates at a plug.

**One currency: priority, not cash.** Bulk yields at the stall also pay in slot credits. Waiting drivers fund them, topped up by the operator from the extra turnover. Credits are minted only when someone was actually waiting, so you can't farm them. They decay after 90 days and can't be transferred. A favour returns as a favour from the network. That's reciprocity, which Gneezy's daycare fine destroyed.

**Second-order effect.** What waiting drivers pay to fund credits, by site and hour, is a demand map the operator can't get any other way. Where yields cost most is where stall nine should go. The queue market ends up doing the siting.

### Turn 4 — Builder
**Correction to my own knee.** Against the stall's rating, a 55 kW Bolt is "past the knee" at 15%, and so is any cold January pack. The knee must be relative to the car's *own* session peak, first ten minutes exempt.

**Kill: cash bids from waiting drivers, for now.** That's your shakedown, plus payments licensing. The operator posts clock-rising yield offers in credits, funded from turnover. Credits are closed-loop, can't be transferred and are never redeemable for cash, so they're loyalty points, not stored value.

**Hard part you skipped: honouring priority at an unattended site.** A slot credit is worthless if someone pulls into the free stall. So the stall authorises only the car at the head of the queue, through Plug & Charge identity or an app start. Walk-ins join the queue but can't jump it. Enforcement stays in software, never in a confrontation.

**Upstream metering, honestly.** Tesla's navigation already routes around busy Superchargers; other networks get no OEM nav feed. MVP signals come from in-app "on my way" check-ins and an ABRP integration.

**Abuse.** *Collusion:* a friend joins the queue so I can yield. Credits mint only if the queued car plugs in within five minutes, never between linked payment methods, with a weekly cap. *Concentration:* ride-hail hoarding is blunted by decay and the cap.

**Model.** SaaS per stall from operators, priced against the extra kWh per stall-hour at peak.

**Stack.** CitrineOS (OCPP 2.0.1), ISO 15118 Plug & Charge, OCPI 2.2.1, a queue service and a credit ledger.

**90 days.** Days 1–30: replay a partner's session logs to measure knees and simulate the turnover gain. Days 31–60: a single site, alternating weeks on and off. Days 61–90: upstream offers between two corridor sites 15 km apart. **Gate:** peak kWh per stall-hour up 20%, and median wait down 30%.

### Turn 5 — Visionary
**Accepting all three changes, and one gap in the third.** A knee relative to the car's own peak is right, and operator-funded credits keep payments licensing out of the MVP. Head-of-queue authorisation is what makes priority real. But NEVI-funded stalls must accept payment without an app or a membership, so the queue can't depend on Plug & Charge alone. A walk-in joins by tapping a contactless card at the kiosk, and the card becomes the queue token. Same rule, no account needed.

**Final form.** Taper is queue middleware for fast-charging operators on OCPP 2.0.1. When every stall is taken, a software queue forms, joinable in the app, via ABRP or by tapping a card at the kiosk. Only the car at the head of the queue can start a session. Past each car's own knee, courtesy is a rule: a three-minute nudge, then a congestion fee. Before the knee, and upstream on the corridor, the operator posts rising yield offers paid in slot credits. Credits are closed-loop, decay, are capped weekly and are minted only when a real queued car plugs in. They buy priority at the next congested stop. Faster turnover pays for the credits, and what yields cost shows where to build.

**Moonshot (2036).** A neutral charging flow-management layer on OCPI spans networks along every holiday corridor, the way Eurocontrol spans airspaces. Credits are honoured across operators. Car agents negotiate arrival slots under AP2 mandates before the driver leaves home, so Thanksgiving lines become ground holds nobody notices. Stall siting is planned from ten years of yield prices.

**Scores.** Innovation **3**. Simon's bumping auction, ground delay programs and congestion fees all exist. The new part is assembling them at the plug, with priority as the currency. Feasibility **7**. It's middleware on open standards; the main obstacles are winning an operator and integrating nav.

### Turn 6 — Builder
**Closing remark.** Taper holds 3/7, which is the right place for a mild seed. It's assembled from known parts, and the pieces carry real weight. The knee rule keeps courtesy from being bought out, head-of-queue authorisation makes priority enforceable, and using a tapped card as the queue token keeps NEVI stalls open to anyone. The operator pays because turnover pays it, and that's why this ships. The real risk is distribution, not mechanism: winning one network willing to let software decide who plugs in next. Upstream metering beyond app check-ins depends on nav partners that may never open up.
