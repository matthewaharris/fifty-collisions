# Idea 41 — Share-Out
> Seed: 41. [mild] **Parametric insurance × smallholder farmers × weather oracles.** Payouts triggered automatically, with no claims adjusters.

### Turn 1 — Visionary
**Working name:** Share-Out

**Hook:** The weather oracle decides how much gets paid. The village decides who gets it.

**The collision.** Parametric cover and village savings groups fail in opposite places. An index is cheap and hard to game, but it can't see a single field. That blind spot is *basis risk*: the gauge says the season was fine and your maize is dead. It's the main reason farmers don't renew. A VSLA (about 20 people with a lockbox and weekly meetings) sees every field, but it breaks on a covariate shock, because in a drought everyone draws at once. Each covers the other's blind spot. Seen as a reinsurance tower, the group is the primary layer for individual losses and the index reinsures the shared shock.

**Core loop. The verb is divide.**
1. **Cover.** The group buys one policy. Its trigger is satellite rainfall (CHIRPS), cross-checked against a gauge at the meeting place.
2. **Fire.** When the index trips, the payout lands in the group's mobile-money wallet. No adjuster.
3. **Share-out**, named after the VSLA's year-end payout meeting. Members declare losses on sealed cards. The pot is fixed, so overclaiming takes money from neighbours, not from a distant insurer, and they have every reason to challenge. Two members chosen by lot walk any contested field.
4. **Learn.** When the index stays silent while most of the group reports loss, a small basis reserve tops up group savings, and the miss becomes a labelled data point for retuning next season's trigger.

**Why now.** ACRE Africa reports more than 5M farmers on index cover (soft), and pilots pay into mobile wallets within about ten working days. McKinsey's 2026 survey: 56% of farmers trust human agronomists, 6% trust AI. So judgment stays with neighbours, and only the arithmetic is automated.

### Turn 2 — Builder
**What exists.** The collision has been tested in the field. Dercon et al. (2014, *JDE*) sold rainfall cover to Ethiopian iddirs, and take-up rose when groups shared basis risk internally. WFP and Oxfam's R4 pairs index cover with savings groups. ACRE, Pula and OKO pay into mobile wallets. Lemonade's Crypto Climate Coalition ran Chainlink weather oracles with ACRE in Kenya. CARE's Chomoka and DreamSave already digitise VSLA ledgers. What's new is the sealed-card division and the learning loop. That's honestly mild.

**Hardest problem: the village-wide miss.** Share-out handles *idiosyncratic* basis risk well: a fixed pot and nosy neighbours can sort out my dead field against a fine index. It doesn't handle *covariate* basis risk. CHIRPS is coarse (about 5 km), the storm misses the gauge, the whole village loses and the index stays silent. That's the case that kills renewal, and the "small basis reserve" carries it with nobody funding it. Sealed cards also stop being honest here. Inside a fixed pot they're zero-sum, but against a reserve the whole group gains by reporting loss together.

**Twist: appeal with a stake.** When the index stays silent, a group can appeal by locking 5% of its savings. That buys a second, costlier test: Sentinel-2 NDVI on the members' plots plus IFPRI-style picture-based evidence (smartphone crop photos, piloted in India and Kenya). If the audit confirms the loss, the insurer pays under the policy and returns the stake. If not, the stake goes to the reserve. False appeals fund the reserve, real ones get paid, and every appeal becomes a labelled point for retuning the trigger.

**MVP.** One Kenyan county, 30 groups through a digital-VSLA partner, ACRE carrying the risk, sealed cards over USSD, one long-rains season. **Kill metric:** group renewal beats individually sold cover by 15 points.

### Turn 3 — Visionary
**Taking the staked appeal, and nesting it.** A stake fixes the funding but not the collusion, because a group that agrees on a story can stake on it. So repeat the fixed-pot logic one level up. Every group in a ward shares one basis reserve, and a false appeal takes money from the next village. The tower becomes fractal: members inside groups, groups inside the ward, the ward inside the insurer. At each level, the people dividing the pot are the ones who'd notice a lie.

**Co-sign or contradict.** When a group appeals, the other groups in the same pixel answer within a week. They co-sign with their own stake ("we lost too") or contradict ("our rain came"). These answers are evidence, not votes, and the audit decides. Their pattern is a village-resolution rain map: a real storm miss shows up as a contiguous patch, and a lie shows up as an island.

**Fixing the coarse oracle (unseeded domain: telecom engineering).** The operator that delivers the payout can also measure the rain. Rain attenuates the microwave backhaul links between cell towers. Messer et al. (2006) recovered path-averaged rainfall from that attenuation, and teams have since applied the method in Burkina Faso and western Kenya. It works at kilometre scale, using towers that already stand beside the villages. It's noisy: wet antennas bias it and it does poorly with drizzle. So it serves as appeal evidence, alongside NDVI and crop photos, rather than as the trigger. It's still much better than one gauge per 5 km pixel.

**Second-order effect.** Each confirmed appeal marks a place where the index is blind. Within three seasons the ward has a public basis-risk map, and insurers compete to fix those places with gauges and better triggers instead of loading the premium for uncertainty.

### Turn 4 — Builder
**Kills.** *Unstaked contradictions:* the ward reserve pays for confirmed appeals, so every neighbour gains by contradicting. Village feuds become evidence. Keep co-signs, require a stake on them, and let the audit find the islands. *Cash stakes in a drought year:* locking 5% of a group's savings right after its crop failed is cruel. Make the stake a pledge against next year's share-out instead. *Microwave links in the MVP:* Safaricom's attenuation logs are a year-long data deal. TAHMO already runs stations across Kenya, so use those and keep the links for season three.

**Failure modes.** Sentinel-2 is optical and blind under long-rains cloud, so add Sentinel-1 radar. Intercropped plots under half a hectare make NDVI noisy, which makes the crop photos the strongest evidence. Elite capture: the chair's relatives declare big. Tally sealed cards in public, and cap each member's claim at their insured share.

**Regulation.** A ward reserve built from forfeited stakes is an unlicensed mutual under Kenya's IRA. Write it into ACRE's policy as a premium-funded basis rider, and hold stakes with the insurer.

**Model.** About 12% distribution commission on subsidised premiums (most index cover in Kenya is co-paid by government or donors). The basis-risk map is licensed to reinsurers and to Global Shield-type funders. CARE-style NGOs pay to plug in their groups.

**Stack.** USSD via Africa's Talking, M-Pesa group wallets via Daraja, CHIRPS plus TAHMO, Sentinel-1/2 on Earth Engine, KoboToolbox for photos, and a ledger connected to Chomoka.

**90 days.** Days 1–30: backtest five seasons of CHIRPS against recall surveys from 30 groups. Days 31–75: a shadow season with real cards and appeals, using pledges, before any money moves. Days 76–90: sign ACRE and enrol for the March long rains. **Gate:** if audits confirm fewer than half of shadow appeals, the pledge is too cheap.

### Turn 5 — Visionary
**Final form.** Share-Out sells one parametric policy to a savings group. It triggers on CHIRPS plus the nearest TAHMO station and pays into an M-Pesa group wallet. The group divides the money at its share-out meeting. Members declare losses on sealed cards, which are tallied in public, and each claim is capped at that member's insured share. Two members drawn by lot walk contested fields. When the index stays silent, the group can appeal with a pledge against next year's share-out. Neighbouring groups in the same pixel can co-sign only by pledging too. The audit uses Sentinel-1, Sentinel-2 and crop photos. Confirmed appeals are paid from a premium-funded basis rider in ACRE's policy. Every audit adds a point to a public basis-risk map. I accept every kill. A pledge does more than a cash stake: a group that lies must keep lying next year.

**One amendment to the gate.** If fewer than half the appeals in a shadow season are confirmed, either the pledge is too cheap or the audit is too blind. Score the audit against recall surveys before you blame the pledge.

**Moonshot (2036).** The tower runs member, group, ward, county, African Risk Capacity's sovereign cover, then reinsurers. The rule is the same at every level: whoever divides the pot is close enough to see a lie. Microwave-link rainfall becomes a regulated by-product of every operator's network. Ten years of honest pledges become a credit history. The basis-risk map tells governments where to build gauges.

**Scores.** Innovation **3**. Dercon's iddirs and R4 proved the pairing, and the new parts are the division mechanism, the pledged appeal and the map. Feasibility **6**. The software is simple, but it needs an insurer partner, subsidised premiums and regulator approval of the rider.

### Turn 6 — Builder
**Closing remark.** Share-Out is honestly mild at 3/6, and that's the right place for it. The pairing was proven by Dercon's iddirs, and what we added is the plumbing: a pot divided by the people who can see a lie, an appeal that costs the appellant next year's share-out, and a map built from every miss. I accept the amendment. Score the audit before blaming the pledge, because a blind auditor would make honest groups look like liars. Whether it works will show up in the March renewal numbers, not in the software.
