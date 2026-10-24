# Idea 19 — Hyphal Bourse
> Seed: 19. [wild] **Mycelial networks × distributed sensing/compute × agriculture.** Biology as infrastructure.

### Turn 1 — Visionary
**Working name:** Hyphal Bourse

**Hook:** Every field runs an underground exchange where plants pay mycorrhizal fungi carbon for phosphorus. Read the exchange rate as a live price of soil scarcity, and the farm trades in that market instead of flooding it.

**The isomorphism is literal, not poetic.** Kiers et al. (*Science*, 2011) showed plants reward the fungi that supply more phosphorus with more carbon, and fungi move nutrients toward better-paying roots: biological market theory. In 2025 her lab robot-imaged fungal networks and found trade organized as travelling waves (*Nature*). Heavy phosphorus fertilizer is known to suppress mycorrhizal colonization. Conventional farming is quantitative easing that kills the underground market it ignores. The contested "wood wide web" claims (Karst et al., 2023) stay out; trade is the robust finding.

**Core loop.** The verb is **quote**.
1. **Tap.** A field "trade window" is a minirhizotron camera plus electrodes on hyphae, calibrated against occasional ¹³C pulse labels. It estimates the carbon paid per unit of phosphorus. Electrical spiking (Adamatzky; Cornell's 2024 fungus-driven robot) is the cheap proxy to be tested.
2. **Quote.** Each window publishes a local exchange rate. A rising price means the crop struggles to buy phosphorus. A soil test measures stock; this measures availability *as the plant experiences it*.
3. **Trade.** An agronomy agent intervenes like a market maker: a micro-dose of phosphorus, cover crops to raise carbon supply, inoculation, or nothing.
4. **Index.** Windows across farms aggregate into a regional soil-function index.

**Why now.** Hawkins et al. (2023) estimate plants send about 13 Gt CO₂e a year to mycorrhizal fungi, the largest carbon flow nobody meters. Frontier approved an MRV methodology for enhanced rock weathering in June 2026, and only 6% of farmers trust AI advice (brief §14). A price from the crop's own roots is advice farmers can check.

### Turn 2 — Builder
**What exists.** SPUN, Kiers's nonprofit, is mapping mycorrhizal networks worldwide. Pattern Ag and Biome Makers sell soil DNA tests, and Ward Labs' PLFA test estimates fungal biomass. Loam Bio raised $73M in 2023 for fungal inoculants that build soil carbon. All of them measure stock: who lives there. Nobody measures flow, the trade itself.

**Hardest problem: there is no field price feed.** ¹³C pulse labelling needs chambers and an isotope-ratio mass spectrometer; it's a research campaign, not a sensor. Minirhizotrons cost $10–20K each and barely resolve hyphae a few microns wide. Fungal spiking in wet, noisy soil has never been tied to trade. A price needs both flows, carbon out and phosphorus in, and no field instrument sees either.

**Twist: the crop reports its own price.** Plants pay arbuscular fungi largely in lipids, through arbuscule-specific genes (RAM2, STR), and receive fungal phosphate through a transporter expressed only in colonized cells (PT4). Engineer sentinel plants with a reporter on each promoter. The minirhizotron then doesn't need to resolve hyphae. It reads a colour ratio in glowing root cells: carbon paid per phosphorus received. A third reporter, on a shoot phosphate-starvation gene (SPX1), maps hunger across the whole field by drone, calibrated against the root windows. InnerPlant already grows soybean sentinels that fluoresce under stress, readable from the air, with John Deere as a partner. One sentinel strip per field turns the crop into a ticker.

**MVP.** Greenhouse first, with a university lab. Use published PT4 reporter lines in Medicago or tomato, vary phosphorus across pots, and calibrate the colour ratio against ¹³C and ³³P labelling. On three farms, run plain minirhizotrons plus PLFA as the baseline the sentinels must beat. Sentinel strips are never harvested, which eases USDA review.
**Kill metric:** the reporter ratio fails to track isotope-measured exchange.

### Turn 3 — Visionary
Sentinels taken: making the buyer report its own price solves the feed. One caveat defines the calibration job. Promoter activity shows a pathway is switched on, not how much flows through it, so the colour ratio is an *index* the isotope campaign anchors, not a meter.

Now push past the ticker: this market moves goods.

**Unseeded domain: Hayek's price system as computation.** Hayek (1945) argued that prices compute an allocation no planner could, compressing knowledge spread across many actors. The mycorrhizal network already runs that computation. Whiteside et al. (*Current Biology*, 2019) showed fungi move phosphorus across networks from rich patches to poor. Don't build compute *out of* mycelium: Adamatzky-style fungal logic gates run at the speed of growth. **Program the market the fungi already run.**
- **Bidder strips.** Plant high-paying genotypes in the phosphorus-poor zones the drone map flags. Start with strongly mycorrhiza-responsive cultivars and move to engineered lines later. A higher carbon bid should pull fungal phosphorus sideways from rich patches. This is open-market operations in soil, a way to move nutrients across a field with no machine.
- **Vendor scoring.** Pair each window's ratio with root eDNA, and a price per strain shows which fungi trade fairly. Audits find that many commercial inoculants carry few viable propagules, so sell them pay-on-performance: vendors are paid only for measured gains in the exchange rate.

**Second-order effects.**
- **Goodhart, in biology.** Fungi evolve. Lineages that bleed a permanent high bidder, taking carbon and delivering little, are the cheaters Kiers's sanctions work describes. Rotate the bidder strips. The feed flags cheating on its own as a rising carbon cost with no phosphorus behind it.
- **A term structure of scarcity.** The regional index becomes a seasonal curve of soil scarcity that fertilizer buyers can hedge against.

### Turn 4 — Builder
**Kills and fixes.**
- *Moving phosphorus across a field.* Whiteside showed transport over centimetres in culture. Tillage shreds the network every year. Shrink the distance, not the idea: bidders become **intercrop rows** 20–30 cm from the crop, on no-till farms only.
- *Sentinels vs. the customer.* The growers who care most about fungi are regenerative and organic, and they reject GMOs. Put sentinels in **trading posts**: buried in-growth cores whose 35 µm mesh lets hyphae through but not roots, with sentinels cut before flowering. Drone-read shoot reporters stay on conventional fields.
- *Scarcity futures.* With no liquidity and no counterparty, it moves to the moonshot.

**Business.** Vendor scoring is the wedge. Salomon et al. (2022) found that many commercial AMF inoculants failed to colonize roots. Buy products off retail shelves, test them blind against the trading posts, and publish scorecards. Vendors pay for a "trade-verified" label and pay-on-performance trials. Later: per-acre phosphorus agronomy, then licensing the carbon-to-fungi flow to soil-carbon MRV.

**Stack.** Promoter–fluorophore constructs (RAM2/STR, PT4, SPX1 plus a constitutive reference); fluorescence minirhizotrons; AMF eDNA (18S primers); ¹³C and ³³P calibration with a university lab; an agronomy agent.

**Failure modes.** Reporter silencing across generations, soil autofluorescence, and a near-zero signal in high-phosphorus fields. The last is a finding in itself: you're overpaying. Vendors will game trials with hand-picked batches, so sample only from retail.

**Safety.** Engineered bidder lines that outcompete local fungal partners need containment rules before any field release.

**90 days.** Weeks 1–4: a lab partner, existing Medicago PT4 reporter lines, a fluorescence rhizotron. Weeks 5–8: greenhouse phosphorus gradient × three shelf-bought inoculants, with isotope labelling. Weeks 9–12: calibration, the first blind scorecard, and three no-till farms signed for trading posts.
**Kill metrics:** the index fails to track isotope exchange, or it can't tell a dead inoculant from a live one.

### Turn 5 — Visionary
All kills accepted; intercrop rows keep bids within Whiteside's observed range. The trading post is the thread's best move, and more than a workaround. Every post holds the same genotype paying a standard bid, so quotes are comparable across farms for the first time: a **benchmark rate for soil**, the SOFR of the underground. One refusal: the scorecard is the wedge, not the identity; as Consumer Reports for inoculants, this is mild.

**Final form.** Hyphal Bourse reads and steers the plant–fungal market.
- **Quote.** Mesh trading posts carry standard sentinel buyers with lipid-paid, phosphate-received and reference reporters, anchored by isotope campaigns. They publish a benchmark exchange rate per farm. Drone-read SPX1 hunger maps cover conventional fields.
- **Steer.** Bidder intercrops on no-till rows pull fungal phosphorus toward hungry zones: open-market operations at root scale.
- **Police.** A rising carbon cost with no phosphorus behind it flags cheater fungi and dead inoculants. Blind retail scorecards and pay-on-performance trials fund the network.
- **Meter.** The carbon-to-fungi flow is licensed to soil-carbon MRV.

**Moonshot (2036): a monetary authority for soil.** Trading posts on SPUN's map publish the underground exchange rate across biomes, as weather stations do for the atmosphere. Agronomists stop prescribing fertilizer and start setting rates: bidder lines and micro-doses tuned to hold each field's soil rate in band. The roughly 13 Gt CO₂e a year flowing to fungi becomes metered and creditable. Contained bidder lines route nutrients across whole fields. A non-human market's price system becomes infrastructure, as GPS did.

**Scores.** **Innovation 8.** Farming becomes participation in a market between kingdoms, read through engineered reporters and steered by bids. Nothing like it exists. **Feasibility 4.** The reporter lines exist. Field calibration, reporter silencing, GMO rules, and whether bids move phosphorus beyond centimetres are all unproven.

### Turn 6 — Builder
The benchmark rate is the right identity. A standard buyer paying a standard bid makes the trading post an instrument rather than a gadget, and the scorecard just pays for it. Innovation 8 stands: sentinel plants, soil DNA and inoculants all exist, but reading a farm as a market between kingdoms and steering it with bids is a new frame, and the strange core held through every fix. Feasibility 4 also stands. The greenhouse calibration can be done in a season with existing reporter lines. But promoter activity isn't flux, and nobody has shown a bid moving phosphorus beyond centimetres in a real field.
