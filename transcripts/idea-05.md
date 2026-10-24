# Idea 05 — Raid Night
> Seed: 5. [bold] **EV V2G × co-op guild games × stablecoin payouts.** A neighborhood virtual power plant run like a raid guild.

### Turn 1 — Visionary
**Working name:** Raid Night

**Hook:** Your street's EVs, water heaters and thermostats form a guild. Grid peaks are raid bosses, and the payouts are split by **DKP**, the attendance currency MMO guilds invented to solve exactly the problem virtual power plants can't.

**The isomorphism:** Not leaderboards. A VPP's hard problem is *dispatchable commitment*. The aggregator promises the utility 200 kW at 6pm, and then Dave drives to soccer practice. Around 1999, EverQuest raid guilds hit the same wall: 40 unreliable volunteers had to show up at a set time for a high-stakes event, and the loot had to be shared fairly. Their answer was **Dragon Kill Points**. You sign up, you show up, you earn points; bail mid-raid and you lose them; loot goes by points. Two decades of guild politics have stress-tested it.

**The loop:**
- **Boss spawns:** forecast peaks are scheduled raids posted 24 hours ahead. Frequency events are surprise world bosses.
- **Sign-up:** a member commits a window and a state-of-charge floor ("plugged 5–8pm, never below 40%").
- **Classes:** *Tanks* are big packs (Cybertruck, Lightning) that hold sustained discharge. *DPS* are fast responders. *Support* is smart thermostats and water heaters shedding load, which lets **renters without EVs join**.
- **Loot:** the utility's event payment is split by DKP in stablecoin within minutes. Battery degradation is paid from a guild repair pool.

**Why the collision works:** Utilities treat customers as interchangeable kilowatts; guilds treat members as accountable people with roles. And DKP is a reliability score the aggregator can show the utility to back its capacity bids.

**Why now:** PG&E approved the Cybertruck Powershare V2X pilot in April 2026, and Tesla Grid Support launched in Texas. Stablecoins sit at roughly $315B under the GENIUS Act. The brief calls V2G "utility pilots with no consumer layer."

### Turn 2 — Builder
**What exists.** Gamified demand response exists. **OhmConnect** (now merged into Renew Home) ran "OhmHours" with streaks, teams and prizes. **Octopus Saving Sessions** (UK) paid millions of households in OctoPoints per session. Tesla's VPP in PG&E's ELRP pays $2/kWh, and Sunrun CalReady, Leap, ev.energy and WeaveGrid are the aggregator plumbing. Nobody models **commitment accounting** for that plumbing, so DKP is the genuinely new piece.

**Hardest problem: the loot is small, slow and not ours to split.** Utilities and ISOs settle against baselines using meter interval data, days to weeks later, so "stablecoin in minutes" is fiction. A thermostat shed earns about $1–3 per event, roughly $50–150 a year. You also have to be the registered aggregator of record (a CAISO DRP, an ERCOT ADER participant) or ride on one. And V2G-capable cars on a single street today number zero to two. PG&E's pilot has 1,000 slots statewide.

**MVP (90 days).** A B2B2C layer on top of an existing aggregator API (Leap or ev.energy), in California under ELRP.
- Three street-level guilds, about 60 homes.
- Mostly thermostats, water heaters and V1G smart charging; V2G Tanks are a rare class.
- Instant DKP at the event, cash monthly at settlement.
- Raid chat plus a real-time "boss HP" bar of guild kW shed against commitment.
- Stablecoin optional, never the default. Neighbours want ACH.

**Twist: the guild is a distribution transformer, and the boss is its loading.** EV clustering is overloading neighbourhood transformers, and each replacement costs utilities tens of thousands of dollars. A guild sharing one transformer that keeps the peak under its rating is a **non-wires alternative**: the utility defers the capex and pays the guild a share. That makes "neighbourhood" physically meaningful instead of social decoration, turns DKP into evidence a utility can bank on, and replaces $2 events with a seasonal raid that pays.

### Turn 3 — Visionary
The transformer twist makes the loot the *scarce resource itself*, which is what DKP was for: guilds allocated one sword among 40 people, not gold.

**Mutation 1: loot is headroom.** Transformer overload is mostly a *timing* problem, with everyone charging at 6pm. The main lever is V1G staggering, not V2G, so the two-car problem goes away. Members *spend* DKP to bid for priority charging windows ("80% by 6am, I'm driving to Tahoe") and *earn* it by yielding headroom or shedding load. It's an internal, non-transferable currency, so stablecoins and money transmission drop out of the core loop entirely. Cash arrives only once a season, as the guild's share of deferred capex.

**Mutation 2: the boss's HP is real physics.** Transformer insulation ages according to hot-spot temperature. Under IEEE C57.91, the aging rate roughly doubles for every ~6°C above the 110°C reference. From AMI interval data plus ambient temperature, the guild sees a live **loss-of-life meter**: hours of transformer life burned tonight compared with a no-guild baseline. It's a defensible basis for the non-wires payment, and more satisfying than kW.

**Unseeded domain: Elinor Ostrom's commons governance.** A transformer's capacity is a textbook common-pool resource, and Ostrom's design principles map almost one to one. *Clear boundaries* are the houses on one secondary. *Graduated sanctions* are DKP penalties. *Local rule-making* means each guild votes on its own DKP variant (zero-sum, suicide-kings, loot council). *Nested enterprises* are guilds under a feeder-level alliance.

**Second-order effects:**
- Utilities get a map of which transformers have communities that can self-govern, and they can target non-wires programs there first.
- Guild health becomes a property attribute ("this street defers its upgrade").
- The ugly side: guild drama. Officers, favouritism and DKP disputes need an appeal path and an exit, or the commons fails.

### Turn 4 — Builder
**The money problem: utilities profit from replacing transformers.** Capex earns a rate-base return, so deferral pays only where regulators force it (California's DIDF, New York's non-wires programs). Even then a deferral is worth about 8% of a roughly $20K swap per year, so ~$1.6K split across 8 homes. **Business model:** sell Raid Night as the engagement layer for managed-charging programs (WeaveGrid, ev.energy, Optiwatt, utility EV programs) at $2–4 per home per month. Their real pain is **override rates**: DKP has to cut cancellations. That's the KPI.

**Kills.** Per-guild DKP variants come out of the MVP: one system, and loot council banned as a favouritism machine. The live AMI-based loss-of-life meter goes too. AMI lags 1–2 days, and transformer mapping lives in often-wrong utility GIS. Use device telemetry with uncertainty bands.

**Failure modes and abuse.**
- *Outsiders on the same secondary.* One non-member with a 19 kW charger blows the boss fight, and Ostrom's clear-boundaries rule fails below about 70% enrollment.
- *Baseline gaming.* Pre-charging inflates "yields"; score against commitments and a counterfactual.
- *Privacy.* "I'm driving to Tahoe" tells the whole street the house is empty. Keep bids private and show aggregates only.
- *Safety and equity.* Emergency overrides are always free, and SoC floors are inviolable. Non-EV Support members get cash, since charging priority is worthless to them.
- *Drama.* Free exit, no shame boards.

**Stack.** Smartcar plus the Tesla Fleet API, charger and thermostat APIs, a constrained charge scheduler, an IEEE C57.91 thermal model, Green Button settlement, React Native, Postgres.

**90 days.**
- Weeks 1–4: partner with Green Mountain Power or a co-op for connectivity and loading data on 5 EV-clustered transformers.
- Weeks 5–8: the DKP managed-charging app.
- Weeks 9–12: 5 guilds against control transformers. **Kill metric:** if overrides aren't measurably lower than plain managed charging, DKP is decoration.

### Turn 5 — Visionary
Rate-base point accepted; it kills "deferred capex loot" outside California and New York. Raid Night sells *reliability*, and override rate is the right KPI: that's what DKP was always for. On outsiders: below 70% enrollment the boss can't be fought, so **recruitment becomes the first quest**. A guild only "forms" once it reaches the threshold, and a new EV on the secondary triggers an invitation instead of a surprise.

**Final form.** Raid Night is the engagement layer sold to managed-charging programs at $2–4 per home per month:
- Each guild is the set of homes on one distribution transformer, and the boss is its evening loading, modelled from device telemetry with uncertainty bands.
- Members commit charge windows and SoC floors, earn one standard DKP for yielding headroom, and spend it on priority charging windows. Bids stay private.
- Non-EV Support members are paid in cash, emergencies are always free, and there are no shame boards.
- Success means measurably fewer overrides than plain managed charging. Where regulators reward deferral, the guild also splits a seasonal non-wires share.

**Moonshot (2036).** Every distribution transformer is a recognized **micro-commons**. The legal hook already exists in the EU's renewable and citizen energy communities. Guilds federate into feeder alliances that bid flexibility into distribution-level markets under performance-based regulation, where utilities earn for avoided capex instead of capex. DKP becomes a portable flexibility reputation that follows you when you move. The grid edge gets governed by Ostrom-style polycentric institutions instead of a central DERMS.

**Scores.** **Innovation 5.** Gamified demand response is old (OhmConnect, Octopus). Commitment accounting borrowed from raid guilds, plus the transformer as a bounded commons, is the non-obvious fusion. **Feasibility 6.** APIs and partners exist; the business is modest, and the kill metric is the honest test.

### Turn 6 — Builder
DKP as commitment accounting is the real find. It's the one thing OhmConnect and Octopus never had, and override rate makes it testable in a quarter. I'm recording feasibility at 5, not 6. The software is easy, but "6 of 8 neighbours on one secondary must enroll before the guild exists" is a door-knocking problem, and transformer mapping depends on a utility partner. Innovation 5 stands. It's a modest B2B business with an unusually good governance story, and the stablecoin was rightly the first thing cut.
