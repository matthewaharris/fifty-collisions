# Idea 25 — Offtaker
> Seed: 25. [mild] **Urban food × data-center/mining waste heat × local subscriptions.** The greenhouse next to the GPU.

### Turn 1 — Visionary
**Working name:** Offtaker

**Hook:** Germany now makes new data centres reuse their heat, and nobody is buying it. Sell the compliance to the data centre, sell the vegetables to the neighbourhood, and let the subscription pay for the glass.

**The honest collision.** Vertical farms died of electricity for LEDs and of financing (80 Acres and Infinite Acres in 2026, brief §14). Waste heat fixes neither, because heat isn't light. So: a sunlit greenhouse, not a vertical farm. And unlike idea 10 (rigs sold to growers as boilers), the *data centre* is the customer with a problem. Germany's Energy Efficiency Act requires new data centres to reuse 10–15% of their heat from July 2026 (sources disagree on the figure) and 20% from 2028. District-heating offtakers aren't showing up (*Data Center Knowledge*: "No One Is Buying It"). Liquid-cooled AI racks return water at roughly 40–55°C. Poor for district heating, fine for low-temperature greenhouse pipes.

**Core loop. The verb is offtake.**
1. **Sign.** A 2–10 MW urban or edge data centre signs a metered heat-offtake contract with a modular greenhouse next door or on its roof. My rough estimate: a 5 MW site at a 15–20% quota needs 1.5–2.5 ha of glass, about Lufa Farms' largest Montreal rooftop.
2. **Fund.** Following Lufa's model, presell weekly basket subscriptions to the neighbourhood and to the data centre's tenants for their staff. Preorders finance the build; the operator pays a per-MWh offtake fee, cheaper than a district-heating link.
3. **Grow.** Heat-loving crops run through winter, when heat is most useful and imported tomatoes most carbon-intensive. Summer falls back to dry coolers.
4. **Prove.** Heat meters and harvest logs produce the operator's mandatory reuse report.

**Why now.** The quota bites this year, AI racks are going liquid-cooled, and the brief flags data-centre heat as urban farming's missing energy fix (§16.12).

### Turn 2 — Builder
**What exists.** Lufa Farms has proven rooftop glass funded by baskets since 2011. Windcloud in Schleswig-Holstein grew spirulina on its data centre's roof. Green Mountain in Norway heats a trout and lobster farm. QScale in Quebec designed its campus around greenhouse offtake. Deep Green heats English pools with edge nodes. The warning is Middenmeer, where hyperscalers sit beside the Agriport greenhouses and the heat mostly goes unused.

**Hardest problem: the counterparty has to outlive the contract.** The data centre's reuse quota runs for 15–20 years, and its compliance would rest on a startup greenhouse funded by vegetable preorders. If the grower folds, so does compliance, and no lender finances glass on that. Siting makes it worse: 2.5 ha next to a Frankfurt data centre is the dearest industrial land in Germany. Data centre roofs are already full of dry coolers. And winter, when heat is most needed, has the least light: tomatoes yield little without lamps.

**Twist: the greenhouse is the cooling plant.** The data centre builds and owns the glass as part of its heat-rejection system, on the yard or roof where dry coolers would have gone, with a reduced dry-cooler bank for summer peaks. It's compliance capex, financed with the building. The grower is a tenant operator on a five-year lease, and baskets fund operations, not glass. If the grower fails, the glass still cools and a new tenant moves in. The reuse report becomes an engineering document, not a bet on a farm.

**MVP.** One German data centre opening in 2026–27 with no offtaker. Run a 2,000 m² pilot bay on its return loop with a grower partner. In winter, grow low-light greens and herbs, and start seedlings for allotment gardeners. Presell 300 baskets.
**Kill metric:** December–February yields can't cover the grower's operating costs without lamps.

### Turn 3 — Visionary
Twist taken. Glass owned by the data centre turns the farm from a counterparty into a tenant, and that's what makes it bankable. Now the kill metric: winter light.

**Unseeded domain: horticultural history.** Nineteenth-century Paris already ran this deal. The *maraîchers* of the Marais heated hotbeds with manure from the city's cab horses, the waste of its transport system, and sold winter salads back to Paris under glass cloches. The GPU is the new horse. Their lesson was **bottom heat**: cool-season crops in low light need a warm root zone, not warm air.
- **Cascade the loop.** The 45–55°C return water heats the air in the sunlit bays. Then, at 25–30°C, it warms root-zone benches for lettuce, spinach, radish and herbs, crops that grow in January light. Lower-grade water means the greenhouse absorbs *more* heat per litre.
- **The forcing shed.** Under the benches or in a windowless annex, run the crops that need warmth and no light at all. Witloof chicory is forced hydroponically in the dark on 18–20°C water, rhubarb by the Yorkshire method, and mushrooms too. Peri-urban farms grow the chicory roots in summer under subscription contracts and send them in for forcing in winter. The CSA links the city to its edge.
- **Buffer tanks.** Inference load peaks in the daytime, and the greenhouse needs heat at night. Dutch growers already shift heat with insulated tanks.

**Second-order effect: the basket as social licence.** Data centres face moratoria and local fights over power and water. A winter basket of endive grown on the data centre's own heat is the most legible thing it could give its neighbours. That's also the greenwashing risk, so the basket ships with a metered heat receipt (MWh reused this week) that the operator can't inflate.

### Turn 4 — Builder
**Kills and fixes.**
- *The forcing shed as a heat sink.* Witloof roots need cold storage near 0°C, and forcing at 18°C takes little heat. Mushrooms make heat. Keep chicory as basket filler; drop mushrooms and rhubarb. The root-zone cascade stays: a bigger ΔT per litre is real absorption.
- *A smaller dry-cooler bank.* Operators require N+1 cooling, and a greenhouse can't count as redundancy. Keep the full bypass: the pitch is compliance, not capex savings.
- *Who owns the heat.* If the data centre owns the glass, the EnEfG may treat its heat as internal use. Sell the heat to the tenant grower through calibrated meters at a nominal tariff, and get a legal opinion in week one.

**Business.** Offtaker designs, builds and operates. It charges the data centre per metered MWh reused and files the reuse report. Growers pay rent plus the nominal heat tariff. Baskets use Germany's Solawi bidding rounds, so low-income neighbours aren't priced out of the showpiece.

**Stack.** Plate heat exchangers (no shared fluid with the data centre), buffer tanks, root-zone heating, a Priva climate computer, MID-certified heat meters and a public receipt that shows heat dumped as well as heat reused.

**Failure modes.** Legionella breeds in 25–45°C water, so keep loops closed, irrigation cold and the exchanger isolated. Greenhouse humidity near air intakes. A grower quitting mid-winter: the lease has step-in rights.

**Ethics.** The basket can launder a data centre's grid and water impact, so the receipt states the reuse share honestly.

**90 days.** Weeks 1–3: legal opinion, heat model, three German data centres opening in 2026–27 without an offtaker. Weeks 4–8: letters of intent with one operator and one Solawi grower, and a 2,000 m² cascade bay designed. Weeks 9–12: a 300-basket bidding round and glass ordered for winter.
**Kill metrics:** no operator signs, or the heat doesn't count.

### Turn 5 — Visionary
All kills accepted. Forcing is a basket crop, not a heat sink. The best piece of turn 4 is the receipt that shows heat *dumped* next to heat reused. A basket that admits what it didn't use is the opposite of greenwashing.

**Final form.** Offtaker designs, builds and runs heat-offtake greenhouses for German data centres that have a reuse quota and no buyer.
- **Glass as cooling plant.** The glass is built alongside a full dry-cooler bypass. The heat goes through an isolated plate exchanger and is sold to a tenant grower on calibrated meters.
- **Cascade.** Return water at 45–55°C heats the sunlit bays, then the 25–30°C remainder heats root-zone benches for winter greens. Buffer tanks move daytime heat into the night.
- **Billing.** The data centre pays per metered MWh reused, and Offtaker files its EnEfG report. Growers pay rent plus a nominal heat tariff.
- **Baskets.** Solawi bidding rounds with a weekly receipt: MWh reused, MWh dumped.

**Moonshot (2036): the maraîcher belt, rebuilt around compute.** Every European data centre over the threshold has a heat tenant, and cities zone new ones where glass can go. Each site cascades by grade: district heating at 60°C, greenhouse air at 45°C, root zones at 28°C, aquaculture below that. A city's winter salad comes from its inference load, as Paris's once came from its cab horses. Heat receipts become a standard disclosure, so an AI provider's sustainability claim is checked in metered MWh.

**Scores.** **Innovation 3.** Every part exists. The new part is the deal: compliance glass owned by the data centre, a tenant grower and a basket with an honest receipt, level with the OACP anchor. **Feasibility 6.** It's buildable with standard horticultural kit, but it depends on one legal reading (does tenant heat count?) and on an operator signing.

### Turn 6 — Builder
The dumped-heat line is the right identity. It turns a PR basket into a disclosure, and that's the only version of this I'd want a data centre's name on. Innovation 3 stands, and only just: this is a deal structure rather than an invention, but compliance-owned glass with a tenant grower answers the counterparty problem that left Middenmeer's heat unused. Feasibility 6 also stands. The engineering is standard Dutch kit, and the whole thing rests on one legal reading of the EnEfG and one operator willing to go first.
