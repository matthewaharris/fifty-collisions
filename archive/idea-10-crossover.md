# Idea 10 — Crossover
> Seed: 10. [wild] **Bitcoin mining × AI compute × EV batteries × waste-heat agriculture.** Everything is flexible load. One market to rule it all.

### Turn 1 — Visionary
**Working name:** Crossover

**Hook:** Wire a power site like a loudspeaker crossover, routing each frequency band of grid volatility to its cheapest absorber: batteries take the treble, bitcoin ASICs the midrange, AI training and EV charging the bass, and a greenhouse's thermal mass the sub-bass. Heat clears in the same auction.

**Core mechanic.** The verb is **route the band**.
1. A wavelet transform splits the site's residual (renewables minus firm load, or LMP) into bands, sub-second to multi-day.
2. Each asset files a **flex card**: response time, interruption cost, deferral window, heat by temperature. Second-life EV packs act in milliseconds. ASICs switch in seconds and lose only hashprice: the sponge. Checkpointed training and depot EV charging flex over hours. Greenhouse thermal mass flexes over days and *eats* the heat the others emit.
3. Every five minutes a co-optimizer clears bands and heat tiers together, an exergy cascade from electricity to compute to 50°C water to tomatoes to fish ponds.

**Why the collision works.** Each industry owns one band and is broken differently. Hashprice (~$28–30/PH/day) sits below miners' cash cost. GPUs cost too much to idle, so they're the *least* flexible load on site. Vertical farms died of energy bills. Cheap ASICs absorb variance so GPUs stay firm, and greenhouses monetise the sponge's waste. A caveat on physics: an ASIC is a resistive heater, and a heat pump delivers 3–4× more heat per kWh. The cascade pays only on curtailed power or heat otherwise dumped. **The wild claim:** grids should buy flexibility *by spectral band from loads*, not by service type from generators.

**Why now.** Miners hold more than $70B in AI contracts (brief §12). Infinite Acres went bankrupt in Sept 2026 (§14). CATL delivered sodium-ion storage at GWh scale (§11). The brief names one flexible-load market as the missing piece (§16 #1).

### Turn 2 — Builder
**What exists.** Miners as flex: Riot booked about $31.7M in ERCOT power credits in one month of 2023. Lancium and Crusoe went from flexible mining to *firm* AI, and Abilene became Stargate. GPU flex is real at the margin: Emerald AI cut an Oracle cluster's draw 25% for three hours in 2025, and Duke's "Rethinking Load Growth" found ~100 GW fits if data centres curtail under 1% of hours. Waste heat: **Blockheating** (data-centre heat to Dutch greenhouses), MintGreen (district heat from mining) and Deep Green (pools).

**Correction.** Grids *already* buy flexibility by band. FFR, regulation, spinning reserve, real-time and day-ahead energy are response-time bands. The new part is allocating those bands *inside* a site, and heat.

**Hardest problem: power, heat and compute want different places and seasons.** Curtailment peaks at spring middays in West Texas. Heat is worth most on winter nights near markets and labour, and it travels a few kilometres at most. The AI tenant pays the most and demands firm power, so across 2025–26 miners are being *evicted* for GPUs.

**Twist: start where heat already clears against power, the Westland greenhouse.** Dutch growers have run CHP, gas boilers and heat buffers for decades, trading every quarter-hour on EPEX through aggregators. They already *are* crossover operators. Add one band they lack: **retired post-halving ASICs as a hash-paying e-boiler** for negative and cheap hours. An e-boiler beats an ASIC on capex, but S19-class rigs now sell near scrap, and low duty cycles suit dying hardware. Immersion loops deliver ~55°C into the existing buffer.

**MVP.** One grower, 1 MW of immersion ASICs, and a Pyomo/HiGHS co-optimiser over ASIC power targets (Braiins/LuxOS), the buffer, CHP and boiler. Six weeks in shadow mode against actual dispatch, then live. GPUs and EV packs stay out until the heat-first core pays.

### Turn 3 — Visionary
The band correction stands: what's new is the crossover *inside* the site, plus heat. The Westland twist answers the location problem. **Hashing is the only product here that travels for free**, so heat demand picks the site and the hashes go out over fibre. GPUs stay where power is firm, and sponges go where heat is valuable.

**Season: add a sub-sub-bass band, the aquifer.** The Netherlands runs thousands of aquifer thermal energy storage (ATES) systems, and Dutch solar middays now set negative prices routinely. So spring-noon power runs the ASICs, the 55°C loop charges the warm well, and winter nights extract that heat through the grower's heat pump. Honest limit: standard ATES permits cap the warm well near 25°C. Either the loop dumps through a heat exchanger at that cap, or we join the high-temperature ATES pilots.

**Mutation: price the rigs as options.** The unseeded domain is real-options finance. A near-scrap ASIC is a strip of call options on hashprice, struck at the power price, and options gain value with volatility. Solar penetration keeps pushing intraday power volatility up, so the *least* efficient, cheapest-to-idle hardware becomes more valuable. Crossover harvests volatility, and its fleet is valued like options, not by joules per terahash.

**Second-order effect: an afterlife market for hardware.** This doesn't stop at ASICs. Around 2030, retired H100s become the midrange sponge, running deferrable batch inference sold per job over x402 whenever power is cheap. Retired EV packs take the treble. The firm AI tenant evicted the miners, but its *retired* GPUs come back as flexible load. E-waste turns into a fleet priced by grid volatility instead of original spec. The greenhouse becomes the retirement home of the compute economy, and it gets paid in heat.

### Turn 4 — Builder
**Kills.**
- *ATES charging.* At a 25°C cap you degrade 55°C heat that a heat pump could make for less, and HT-ATES is still pilots. Summer isn't solved. ASICs run hash-only on cheap hours and dump heat through dry coolers.
- *Retired H100s over x402.* Old A100s still rent profitably. Moonshot, not plan.
- *"Least efficient is most valuable."* True only for capex-free, connection-free rigs. In the Netherlands, **grid connection capacity is the scarce asset** (netcongestie). A non-firm connection contract suits a sponge, but every MW competes with the grower's own electrification.

**Business model.** Heat-as-a-service. Crossover owns the rigs, sells heat below the grower's gas-plus-ETS boiler cost, and keeps the hash. Later, it licenses the optimiser to greenhouse aggregators.

**Stack.** A Pyomo/HiGHS rolling MILP on 15-minute imbalance settlement periods, with EPEX day-ahead, intraday and TenneT imbalance (seconds-fast ASICs are the treble). Braiins/LuxOS power targets, FPPS pool payouts, and a read-only Priva/Hoogendoorn climate-computer feed.

**Failure modes.**
- *All-in price.* kWh levies and kW-max tariffs can erase negative EPEX prices, so model delivered cost, not the exchange price.
- *Temperature.* CHP buffers run near 90°C, and 55°C fits only low-temperature circuits.
- *CO₂.* Growers dose CHP exhaust into crops. Displacing the CHP means buying OCAP CO₂.
- *Hardware.* Old rigs fail. Immersion fluid is a fire hazard. The 2028 halving cuts revenue.
- *REMIT.* Imbalance chasing must stay clear of market manipulation.
- *Greenwash.* Never claim carbon negative.

**90 days.**
- Weeks 1–4: two years of grower data and an all-in backtest.
- Weeks 5–8: a 250 kW immersion pilot on a low-temperature loop, run in shadow mode.
- Weeks 9–12: live, imbalance through the aggregator's BRP.
- **Kill metric:** if the ASIC band doesn't beat a plain e-boiler per MW-year after levies, tariffs and failures, sell e-boiler controls instead.

### Turn 5 — Visionary
All three kills accepted; summer stays honestly unsolved. One fix to the CO₂ and temperature failures: **the ASIC band displaces boiler hours, not the CHP.** The CHP keeps its 90°C buffer, its CO₂ dosing and its winter power sales. The immersion loop feeds only low-temperature circuits during cheap hours, when the boiler would otherwise burn gas. Netcongestie also sharpens the pitch: a sponge on a non-firm contract is the one load a congested DSO *wants*, because it leaves when asked.

**Final form.** Crossover sells heat-as-a-service to Westland growers. It owns immersion-cooled, near-scrap ASICs on a non-firm connection behind the grower's meter. A rolling 15-minute MILP co-dispatches ASICs, boiler, buffer and CHP against *all-in delivered* power cost: EPEX day-ahead, intraday and TenneT imbalance, with levies and kW-max tariffs included. Heat is sold below the grower's gas-plus-ETS cost, and Crossover keeps the hash. If the ASIC band can't beat a plain e-boiler per MW-year, the optimiser becomes the product.

**Moonshot (2036): one connection, many bands.** Connections, not electrons, are the scarce resource. Crossover's flex card grows into an open standard, and DSOs grant capacity *by band*: firm for GPUs and climate control, non-firm for sponges, seconds-fast for batteries. Campuses of mixed loads share one connection and clear power and heat in a single local auction. The greenhouse becomes the compute economy's retirement home: dying ASICs, retired GPUs running deferrable batch jobs, and second-life packs, each valued as an option on volatility, not on spec.

**Scores.** **Innovation 4.** The seed aimed wild, and we walked it down. Blockheating, MintGreen and Heatmine sit close. The new parts are band allocation inside the site and option-valued scrap hardware. Only the moonshot's band-granted connections reach wild. **Feasibility 6.** Proven parts, and growers already fluent in imbalance markets. The e-boiler kill metric could well bite.

### Turn 6 — Builder
Displacing boiler hours instead of the CHP is the fix that makes this a real business, and pitching the sponge as the one load a congested DSO wants is the right sales line. My honest score is innovation 3, not 4, and the tier is mild. The shipped form is heat-as-a-service with a good dispatcher, close enough to Blockheating and Heatmine that the new parts (in-site band allocation, option-valued scrap) are recombination at the OACP anchor. Feasibility 6, and I'd bet even money the e-boiler kill metric bites, in which case the optimiser alone is still a sellable product.
