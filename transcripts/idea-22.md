# Idea 22 — Conserved
> Seed: 22. [bold] **Battery passports × provenance games × consumer curiosity.** Follow a lithium atom from mine to car to grid to recycling.

### Turn 1 — Visionary
**Working name:** Conserved

**Hook:** You can't follow a lithium atom, because atoms are fungible. So the game has you follow the *claim* and check whether the atoms add up.

**The stranger second idea.** The seed's premise is physically false; that's the opening. Once a refinery mixes Chilean brine, Australian spodumene and recycled black mass in one tank, provenance stops being a fact and becomes an accounting rule. Under mass balance, "16% recycled" is a share of the books, not of the atoms. Bitcoin forensics fights the same war over tainted coins (poison vs. haircut rules). A battery passport is a ledger of claims about fungible stuff, and conservation of mass is the one rule no claim can break.

**Core loop. The verb is balance.**
1. **Scan.** Scan the passport QR on your EV, e-bike or home battery. You get its claimed lineage as a flow graph, mine → refiner → cathode → cell, with carbon and recycled-content claims.
2. **Stack.** Every scan adds claims to one shared graph. Each node is a nonogram: public totals (UN Comtrade customs flows, company reports, refinery capacity) are the row sums, and every passport's claims on that node must fit inside them.
3. **Leak.** When a refiner's declared recycled or low-carbon output exceeds any plausible input, the node glows. Players play evidence cards: Sentinel-2 brine-pond area, shipping records, isotope assays. Desaulty et al. (*Nat. Commun.*, 2022) showed the ⁷Li/⁶Li ratio separates brine lithium from hard-rock lithium, though not yet mine by mine.
4. **File.** Confirmed leaks go to a public ledger, the issuer and market-surveillance authorities.

**Why now.** The EU battery passport becomes mandatory in 2027 (brief §12), and recycled-content floors begin in 2031. China's export-control lists and the G7's target of under 60% from any one supplier by 2030 make origin geopolitical. Consumer curiosity becomes the largest unpaid audit in the world.

### Turn 2 — Builder
**What exists.** Circulor traces cobalt and lithium for Volvo and Polestar, and the EX90 shipped the first EV battery passport in 2024. The BatteryPass consortium wrote the data model. ISCC PLUS certifies mass balance for chemicals, and Oritain does isotope origin forensics for cotton. The closest precedent is Global Fishing Watch, which turns public AIS and satellite data into catch-versus-landings balances that expose dark fleets. Nobody lets consumers audit the claims.

**Hardest problem: the nonogram has no sharp sums.** Under Regulation 2023/1542, the public layer of the passport shows chemistry, carbon footprint and recycled-content shares. Refiner and mine lineage go only to parties with a "legitimate interest." Comtrade sums are national HS codes, refinery output is mostly private, and "plausible input" is a wide band. Fuzzy rows yield almost no contradictions. Worse, 10,000 scans of the same Model Y return the same claim 10,000 times.

**Twist: count the cars, not the atoms.** A recycled-content claim under mass balance is a *double-spend* problem, not a taint problem: one recycled tonne gets sold to three OEMs. That sum is sharp. Multiply each passport's claim by the kWh, the chemistry's lithium per kWh and the public registrations (EAFO, Germany's KBA). That gives claimed recycled tonnes per OEM per year, and the industry sum must fit under what recyclers actually produce: Redwood, Umicore, Brunp, GEM, and Glencore's Li-Cycle assets. An overdrawn ledger is the signal. Scans matter because they supply the hidden column: which cell supplier sits in which VIN range, read from the label and the passport, the one join the public data lacks.

**MVP.** Before 2027, use voluntary passports, OEM sustainability reports, registrations and recycler disclosures. Ship an "Overdrawn" ledger, plus a puzzle mode where players resolve VIN-to-cell joins and chemistry.
**Kill metric:** the industry-wide claimed total can't be bounded tightly enough to ever go overdrawn.

### Turn 3 — Visionary
Double-spend taken. It moves the game from forensics on single shipments to bookkeeping on a whole industry, which is where mass balance actually lies. But the kill metric is probably right for any single year. Recycler disclosures and lithium per kWh carry perhaps ±30% error, so no one-year ledger goes cleanly overdrawn.

**Unseeded domain: nuclear safeguards.** The IAEA has bounded noisy mass balances for fifty years. It divides facilities into *material balance areas* and computes *material unaccounted for* (MUF) along with its uncertainty, σMUF. It never needs one damning period: near-real-time accountancy runs sequential tests (Page's CUSUM), so a small, persistent diversion crosses the threshold even when every single period looks like noise. Take the whole toolkit.
- **Balance areas.** Each OEM-and-cell-maker pair is an area with a quarterly MUF.
- **A two-sided bound.** Squeeze recycled supply from below as well. Recycler feedstock is capped by deregistered EVs (KBA/EAFO data, plus player scans at scrapyards) and declared gigafactory scrap. So claims ≤ recycler output ≤ feedstock × yield.
- **CUSUM, not contradiction.** An honest OEM's MUF wanders around zero, and a double-spender's drifts. The game's meter is each area's cumulative drift line. Players score by tightening σ: finding the VIN join or the disclosure that shrinks an area's error bar.

**Second-order effects.**
- **Pressure to register.** Global Fishing Watch's public maps preceded Indonesia and Peru publishing their vessel-tracking data. An OEM with a rising drift line has the same reason to pre-register its recycled offtakes in a public retirement registry. Book-and-claim grows out of the game instead of being imposed.
- **A price for unspent recycled lithium.** Once supply and claims are both visible, unclaimed recycled tonnes become a scarce, observable asset, years before the 2031 floors bite.

### Turn 4 — Builder
**Kills and fixes.**
- *Lithium first.* Few OEMs claim recycled lithium yet (the 2031 floor is 6%). Cobalt and nickel claims are live now, and cobalt's floor is 16%. Start there.
- *Scrapyard scans.* Trespass, and thin data: most recycler feedstock is gigafactory scrap, and many deregistered EU cars are exported, not shredded. Bound feedstock from scrap declarations and recycler permits.
- *Absolute CUSUM.* MUF error here is systematic (a wrong cobalt-per-kWh constant), not random, so it drifts for an honest OEM too. Borrow the IAEA's other trick, shipper–receiver differences: compare OEMs sharing a chemistry and cell maker, so the shared bias cancels. Teardown assays from players pin the constants.
- *Price for unspent tonnes.* No market yet. Moonshot.

**Business.** Run the public retirement registry the drift lines pressure OEMs into, I-REC style: fees per retired recycled tonne. Data licences go to market-surveillance authorities, insurers and ESG analysts. The app stays free.

**Stack.** Ingest registrations (EAFO, KBA), passports (BatteryPass schema; the EU registry QR from 2027), recycler disclosures and Comtrade. A hierarchical Bayesian MUF model in PyMC, with CUSUM on relative balances. A cell-label OCR scanner and a hashed-photo evidence store.

**Failure modes and abuse.** Defamation is the big one: say "unaccounted," never "double-spent," publish σ next to every line, and give OEMs a right of reply before release. Short sellers will weaponize drift lines and OEMs will astroturf joins, so every join needs photo evidence and reputation-weighted independent confirmation. Honestly, the players are EV-forum enthusiasts, journalists and students, not the public.

**90 days.** Weeks 1–4: model three chemistries and ten OEMs across EU registrations 2023–26, and compute σMUF. Weeks 5–8: 300 enthusiasts resolve VIN-to-cell joins, plus five teardown assays. Weeks 9–12: publish the first quarterly accountancy report with replies, and partner with Transport & Environment.
**Kill metric:** relative σMUF can't get below the claimed recycled share.

### Turn 5 — Visionary
All kills accepted. Shipper–receiver differencing is the right fix: it cancels the systematic error. Two changes. Rename it **Unaccounted**: the defamation-safe word is also the most accurate one, so let it be the brand. And I refuse to drop the consumer entirely. Enthusiasts do the work, but the door is still *your* car. Curiosity brings people in; tightening σ keeps them.

**Final form.** Unaccounted is public safeguards accountancy for recycled-content claims, starting with cobalt and nickel.
- **Scan.** The passport and cell label join your VIN to a cell maker: "your car's cobalt claim sits in area 14, drift +0.4σ."
- **Balance.** Quarterly MUF per OEM-and-cell-maker area (hierarchical Bayesian). CUSUM runs on shipper–receiver differences between OEMs that share a chemistry and a cell maker.
- **Tighten.** Players score by shrinking σ, through photo-confirmed joins, teardown assays and recycler disclosures.
- **Report.** A quarterly accountancy report says "unaccounted," publishes σ next to every line, and carries the OEM's reply.
- **Retire.** The I-REC-style registry that the drift lines push OEMs toward funds the operation.

**Moonshot (2036): an IAEA for the circular economy.** Cobalt, nickel, lithium, rare earths and nodule metals each have public balance areas from mine to recycler. A passport claim is valid only if it was retired in the registry. Mine-by-mine isotope libraries let physical fingerprints check the books. Unspent recycled tonnes trade at a public spot price. Every car is a sensor in the ledger of its own materials.

**Scores.** **Innovation 5.** Global Fishing Watch already audits the sea from public data, and the IAEA already runs this math. Joining them into a citizen accountancy for recycled claims is a non-obvious fusion that holds the bold floor. **Feasibility 6.** The 90-day plan runs on public data. But the kill metric is real: the relative σ may never get below the claimed shares.

### Turn 6 — Builder
"Unaccounted" is the right name because it's the claim the data can actually support, and keeping the scan as the front door is fair as long as nobody pretends scanners are the auditors. Innovation 5 stands: safeguards math applied to recycled-content claims, joined to Global Fishing Watch-style public auditing, is a real fusion, and the move from following atoms to catching double-spent tonnes kept it from shrinking into a passport viewer. I'm marking feasibility down to 5. Everything in the 90-day plan can be built, but the product only works if relative σ beats the claimed shares, and the passport's public layer stays thin until 2027.
