# Idea 07 — Recount
> Seed: 7. [mild] **Farming × mine tailings × carbon credits.** Enhanced rock weathering MRV for small farmers.

### Turn 1 — Visionary
**Working name:** Recount

**Hook:** Measure carbon the way Colorado audits elections. Pool hundreds of small farms, pick fields to core by public lottery, and stop sampling once the pool's claimed tonnage is statistically confirmed.

**The isomorphism.** ERW's bottleneck is MRV. Soil carbonate and cation losses vary so much across a field that verifying a 20-hectare farm can cost more than its credits earn. Election officials faced the same shape. Hand-counting every ballot is too costly, and trusting the machine count is too risky. Philip Stark's **risk-limiting audits** sample paper ballots one by one until the reported result is confirmed at, say, a 5% risk limit. So a pool's claimed tonnes are the reported margin and a soil core is a paper ballot. Audit cost then scales with *discrepancy*, not with the number of farms.

**The loop:**
1. **Match.** Each farm is paired with the nearest assayed feedstock, either mine tailings or quarry fines, within a haul radius where trucking emissions don't eat the credit. Every load carries an XRF passport covering mineralogy and a nickel and chromium screen.
2. **Spread and declare.** The farmer logs truck tickets, spreader GPS or phone photos, and the application rate. That's the "machine count".
3. **Lottery.** A public random draw picks fields and grid cells to core. A sequential test runs until the pool's claim holds or fails.
4. **Pay.** Credits are issued at the confidence lower bound and split pro rata. Fields that miss trigger targeted re-audits and clawbacks.

**Why now.** Frontier approved Puro.earth's ERW methodology on June 22, 2026. Climeworks and Lithos signed an offtake in March 2026, and credits trade around €200–500/t (brief §14, price soft). The brief names small-farm ERW MRV as an unbuilt gap (§14, §16 #7). Open problem for the Builder: unlike ballots, soil cores are read *with noise*.

### Turn 2 — Builder
**What exists.** Smallholder ERW pooling already won: **Mati Carbon** took the $50M XPRIZE Carbon Removal grand prize (2025) spreading basalt on Indian rice farms. **Terradot**, **InPlanet**, **UNDO**, **Eion** and **Lithos** all run farmer networks with in-house MRV. **Isometric** and Puro protocols decide which sampling counts. The cautionary tale is soil organic carbon: Nori shut down in 2024 amid sampling disputes. Nobody sells *the audit design itself*, and that's the opening.

**Hardest problem: a core reads biased, late and partial, not just noisy.** An RLA assumes the audited ballot reveals the truth. A soil core gives cation loss with a lab error bar on top of spatial heterogeneity that dwarfs the signal at 10–40 t/ha. It's read years before weathering finishes and can't see river and ocean losses. The "reported margin" is itself a model output, so a sequential test on raw cores either never stops or certifies a bias.

**Tailings reality check.** Ultramafic tailings carry nickel and chromium and often need waste-exemption permits. Basalt fines are the mainstream feedstock. Make tailings a second lane, not the hook.

**MVP.** Recount is not a registry. It's the sampling-design layer for one existing developer under the Isometric or Puro protocol, with one co-op of about 60 farms sharing one quarry. The draw is a public dice ceremony, as Colorado does it. Months 1–3: re-run published ERW trial data to show the lab analyses saved at equal uncertainty, and bank this season's baselines.

**Twist: core everything, analyze by lottery.** Lab analysis is the cost, not coring. Pull cheap baseline cores on a dense grid before spreading, chain-of-custody them and archive them unanalyzed. After spreading, the lottery draws cells and the lab analyzes only those **paired** before/after cores. Pairing cancels most spatial variance, and the draw happens too late to game.

### Turn 3 — Visionary
Tailings drop to a second lane, and the dice ceremony stays. The twist goes further with an unseeded domain: **anti-doping**.

**Mutation: the B-sample archive.** WADA stores athletes' samples for ten years and retests them as assays improve, and London 2012 medals were stripped that way. Every paired core gets split. The A half goes to the lottery. The B half is sealed under chain of custody for ten years. The Builder's three problems change shape:
- *Late:* credits **vest in tranches**. Year one issues the lower bound of what's verified now. Later lottery rounds pull fresh post-cores against the archived baselines, so long-run weathering becomes a **top-up** and not a guess. Rising-only credits invert the buffer pool.
- *Biased:* run a **comparison audit**, the most efficient kind of RLA, instead of testing raw tonnes. The developer's model files a per-cell prediction before spreading, just as a voting machine files a cast-vote record. The lab checks paired cores against it. Model bias now shows up as discrepancy the test is built to catch. Stark's SHANGRLA framework handles any bounded quantity, so noisy reads become clamped per-cell discrepancies with the lab's duplicate error built in.
- *Partial:* river and ocean losses get a published catchment haircut, retestable later like everything else.

Nobody games a sample they know may be re-examined in 2036 with better methods.

**Second-order effect: certainty gets a price tag.** Because unanalyzed cores sit in the archive, the protocol can quote the marginal value of one more analysis: "$40 more lab work raises this pool's lower bound by about 2 t." A buyer who wants a tighter error bar pays for extra draws, and that money becomes farmer income. The archive becomes the densest chain-of-custody ERW field dataset anywhere; academic labs will pay to analyze cells the lottery skipped.

### Turn 4 — Builder
**Kills.**
- *"Rising-only credits."* At a 5% risk limit, about one pool in twenty is certified wrong. Later rounds claw back too, so keep a buffer sized to the risk limit.
- *Archive scope.* Drying shifts exchangeable cations, so B-samples support only total-element mass balance (Ti/Zr tracers), not porewater or exchangeable assays. Storage is cheap; Rothamsted has archived soil since 1843.
- *Academic side-analyses.* If anyone can fund analysis of chosen cells, developers will cherry-pick. Every analyzed core counts in the audit, and extra analyses are drawn by lottery too.

**Business model.** Developers pay a per-tonne audit fee, recovered from cheaper lab work. Buyers pay for extra draws. Farmers get free rock and an upfront spreading payment, as Mati and Terradot do, because tranches are too slow for cash flow.

**Stack.** Stark's open-source SHANGRLA, Rivest's consistent sampler seeded from the dice, an offline field app (GPS, photos, barcoded tamper-evident bags), a LIMS and an append-only custody log. No blockchain.

**Failure modes and abuse.**
- *Escalation.* Poor early models mean high discrepancies, and the audit drifts toward a full count. Cap the budget and issue at whatever lower bound was reached.
- *Salting.* Bre-X salted gold cores. Borrow mining's NI 43-101 QA/QC: samplers split at the field, insert blind certified reference materials, and send duplicates to a second lab.
- *Baseline fraud.* Third-party samplers only, GPS-locked.
- *Heavy metals.* Every pull screens nickel and chromium, so the archive is also a contamination record.

**90 days.**
- Weeks 1–4: re-run public ERW trial data through comparison audits against the default stratified grid.
- Weeks 5–8: submit a sampling module to Isometric, and sign one developer and one 60-farm co-op.
- Weeks 9–12: bank baselines before spreading.
- **Kill metric:** under 30% lab savings at equal uncertainty, or no registry engagement.

### Turn 5 — Visionary
All three kills accepted. The risk limit *is* the buffer size. That beats today's negotiated buffers. One fix for escalation: **stratify the draw by the model's own confidence**. Cells where the model claims tight predictions get sampled more, so an overconfident model gets caught early and cheaply, before the budget cap bites.

**Final form.** Recount is the sampling-design and audit layer for ERW developers, not a registry. Third-party samplers pull a dense grid of GPS-locked baseline cores before spreading. Each is split and archived unanalyzed. The developer files per-cell predictions in advance. After spreading, a public dice ceremony seeds Rivest's sampler, and the lab analyzes only paired before/after cells. SHANGRLA runs a comparison audit to the risk limit or the budget cap. Credits are issued at the lower bound in tranches, against a buffer sized to the risk limit. Blind reference materials, second-lab duplicates and nickel/chromium screens are borrowed from mining QA/QC. B-halves are kept ten years for total-element retests. Farmers get free rock and paid spreading up front.

**Moonshot (2036): carbon removal gets a recount.** A nonprofit **sampling authority**, a Colorado Secretary of State for carbon, audits every durable removal claim by lottery: ERW, biochar, soil carbon, ocean alkalinity. Behind it sits a continental B-sample library, a Rothamsted for the whole market. When the science shifts, the market is re-adjudicated from physical evidence, not from arguments about models. Developers compete on discrepancy rates the way auditors compete on clean opinions. A tonne's price tracks its audited confidence.

**Scores.** **Innovation 3.** Risk-limiting audits, WADA-style B-samples and NI 43-101 QA/QC are all proven. The new part is the transfer, squarely at the OACP anchor. **Feasibility 7.** Open-source statistics, cheap coring and a narrow wedge, and no new science needed. Registry acceptance and the 30% lab-savings kill metric are the real gates.

### Turn 6 — Builder
Stratifying by the model's own confidence is a sharp fix, as long as the unequal draw probabilities are weighted in the test (SHANGRLA's stratified audits handle that). My verdict is innovation 3, and I'm taking feasibility down to 6. Nothing proves itself until a full weathering season has passed, and registry acceptance isn't ours to control. The best part is that the election-audit transfer makes MRV cost scale with how wrong you are, not how big you are, and that's the economics small farms need.
