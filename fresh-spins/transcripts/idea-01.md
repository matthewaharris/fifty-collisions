# Idea 01 — Ran It
> Seed: 01. **A household object people own and mildly resent** (physical product; Amazon and TikTok Shop lane). The kitchen timer, the reusable ice pack, the dish rack, the lunchbox, the key hook, the doorstop, the laundry basket. Pick one with a large installed base and a daily annoyance. Possible borrowings: #9 Caught (a single control that only registers noticing), #38 Tally (a tag board that proves everyone is out), #11 Washout (a tiny self-experiment built into a habit). Constraint: tooling and first run under about $50K, retail price $15–60, a demo that works in a 5-second video.

### Turn 1 — Visionary
**Working name: Ran It.** The old product is the clean/dirty dishwasher magnet, a $6–10 sign sold in dozens of identical Amazon listings. What's stale: it only knows what the last person remembered. Whoever ran the load at 11pm forgot to flip it. At 7am someone stacks dirty mugs on clean plates, or reruns a clean load "to be safe." The magnet records memory, not events.

**The one change:** a clip for the top rack that flips itself to CLEAN when a wash cycle gets hot and stays there until someone thumbs it back to DIRTY. Inside is an off-the-shelf manual-reset bimetal snap disc, set near 50°C, which pushes a green flag into a window. No battery, no radio, no app, so no FCC bill.

**First buyer:** couples and roommates who already own the magnet, on the morning they open the door and can't tell whether the load ran. Second buyer: office managers with a shared kitchen dishwasher that nobody owns.

**Why now:** the brief's first rule is a silent 5-second demo, and this has one: door opens, clip reads green, thumb clicks, it turns red. It's one moulded part plus a disc (about $0.50, unverified), close to the cheap-tooling profile of BrüMate's $25–30k start. Household-argument videos suit TikTok Shop (about $15.1B US GMV in 2025). US5839458 (1998) patented a bimetal clean/dirty pointer. It looks expired, and I found nothing like it on sale. The incumbents print magnets; they don't make mechanisms.

**Borrows:** from #38 Tally, state is set by the event rather than by anyone's memory, and every hot cycle proves the device still works. From #9 Caught, a single control you press only when you notice.

**Price:** $19.99 for a two-pack (one to gift). The open problem is resetting on unload.

### Turn 2 — Builder
**Correction.** US5839458 (Delcarson, 1998, expired) isn't bimetal. It uses a release arm that a dish holds down. The bimetal-spring-plus-latch design is US6196239 ("Dishwasher status indicator"). Prior art goes back to 1971 (US3608514, US4129954, US4653423), and US10662571 ("Clean status indicator", 2020) may still be live (freedom to operate unverified). So it's free to copy but has little claim to patent, which means APEX won't stop knockoffs. Fifty years of patents, no hit: a warning. No Amazon listing found (unverified).

**What exists:** clean/dirty magnets at $6–10 (Visionary's figure), and the dishwasher's own free "Clean" light, which on many machines goes out the moment someone opens the door to peek.

**Physics, the hardest problem:** eco programmes wash at roughly 40–50°C for a long time (unverified per model), and the disc lags inside a plastic shell. US hot tap water is about 49°C (120°F). A plain 50°C trip point will miss eco loads and fire on a hot-tap rinse. One wrong reading and they're back to the magnet. Detergent (alkaline, enzymes), rinse aid and roughly 70°C drying are fine for food-contact PP and silicone, but bare bimetal corrodes, so it needs a fully welded seal.

**Twist:** don't design a disc and flag. Buy an off-the-shelf manual-reset thermal cutout (KSD301 class, sold at 40–150°C, about $0.30–0.60, unverified). Its reset button already pops out when tripped. Pot it under a green-capped silicone dome in a single welded PP shell. The pop-up button is the flag, and pressing it is the reset: one mould and one dome, tooling about $10–15k.

**MVP:** 40 hand-built units with 45/50/55°C parts, logged across 20 home dishwashers on eco, quick and normal cycles, plus hot-tap tests.

**Economics:** $19.99 two-pack, landed about $4. After the 15% referral fee ($3.00), FBA ($3.18) and 15% ads ($3.00), that leaves about $6.80 (34%) before returns.

### Turn 3 — Visionary
**Accepted:** the KSD301 pop-up as flag and reset, and the patent correction. No moat from patents.

**Wash vs. hot rinse: trip on time, not temperature.** Borrow from cold-chain time-temperature indicators: integrate heat over time. Pot a 42°C KSD301 inside a thermal slug, a small steel washer in thick silicone, tuned to a time constant of about 10 minutes. A 60-second 49°C hot-tap rinse lifts the core only a few degrees. A 30-minute eco wash at 45°C, or any normal cycle, trips it. The Builder's 40-unit MVP becomes a tuning run: vary slug mass, log core temperature against eco, quick and hot-tap. If quick cycles miss, we print "not for rinse-only or quick wash" (unverified until logged).

**Copycats:** a knockoff with a bare 50°C disc will misfire on eco, so make the slug the named claim. "Eco-proof: tested on 20 dishwashers," with a side-by-side video of a bare disc lying. Beyond that, honestly, this is a gift and colourway product, not a consumable. Repeat comes from colour drops with lines ("RAN IT" / "YOUR MOVE"), housewarming two-packs and a 6-pack office kit at $39, a buyer Amazon knockoffs don't court. File the $130 provisional on the slug as cheap insurance, not a moat.

**Five seconds:** it ships tripped. The first use is pressing the green dome and hearing it click to red. The photo is a green dome among wine glasses with the caption "it ran."

**Demand test (3 weeks, about $1.5k):** 15 hand-potted units filmed in real dishwashers. Run three silent 5-second Reels and TikToks with $600 of spend to a Shopify pre-order page ($19.99 two-pack, ships in 90 days, refundable). Run a parallel LinkedIn and r/OfficeManagers post for the $39 office kit. **Go** at 2%+ visit-to-preorder and paid CAC of $15 or less. Stop below 0.7%.

### Turn 4 — Builder
**Slug physics hold up, mostly.** Eco runs long at 45–50°C: Bosch eco washes about 2 hours before rinsing (one German study); Fisher & Paykel lists a 45°C wash, 55–60°C final rinse. A 60-second 49°C tap rinse lifts the core about 3°C. Heated dry only follows a wash, so tripping there is correct. The real gap is part tolerance: a cheap KSD301 drifts about ±5°C (unverified), so a "42°C" part might trip at 47°C and miss eco. Fix: test every unit in two baths (no trip at 38°C, trip at 44°C), about a $2k rig.

**Cuts.** Colour drops: they sell things carried in public, not things inside a dishwasher. Office 6-pack: one kitchen, one unit. $15 CAC gate: DTC contribution is about $10 after shipping, so the bar is $8.

**Market fit, plainly:** no consumable, no visible colourway, and copies at $9.99 within months; the only defence is speed, reviews and the bath-tested eco claim. It's a profitable impulse SKU selling thousands of units, not returning customers in 12 months. Around 5. The one repeat buyer worth testing: realtor closing gifts and short-term-rental hosts buying branded packs in 25s (unverified).

**Liability:** food-contact PP and silicone, nickel-bearing bimetal sealed in (check Prop 65), no battery, never sold as a kids' product.

**First 100:** about 60 test pre-orders, 30 TikTok Shop creators at 20% commission, 30 Vine units ($200).

**90 days:** weeks 1–3, bin-test 200 parts from two suppliers and pot 15 units. Weeks 3–6, logger runs in 20 dishwashers. Weeks 6–10, aluminium tool and silicone dome (about $12k), 500-hour 70°C detergent soak and 1,000 thermal-shock cycles. Weeks 10–13, 1,500 two-packs. About $25k all in.

**Kill:** under 1.5% pre-order or CAC over $8; eco misses over 2% in logging; or 60 days after launch, under 8 orders a day or returns over 8%.

### Turn 5 — Visionary
**Accepted:** the two-bath test on every unit, the $8 CAC gate, and the kill metrics. Colour drops and the office 6-pack are cut.

**Final form.** Ran It is a welded PP clip with a silicone dome. Inside, a bath-tested 42°C manual-reset cutout is buried in a steel-slug thermal mass. After a real wash the green dome pops up. A hot-tap rinse doesn't move it. Press it and it goes back to red. Hero SKU: $19.99 two-pack on TikTok Shop and Amazon.

**Boring on the surface,** it's a dishwasher magnet. **But** it's set by the wash, not by whoever remembered.

**The repeat buyer: short-term-rental turnover.** Checkout lists say "run the dishwasher"; the cleaner can't tell whether the guest did. The clip settles that with no app, and the cleaner resets it at turnover. Hosts with several listings buy one per unit and rebuy as they add units or replace lost clips. Channel: host Facebook groups, BiggerPockets, cleaning companies. The offer: a 10-pack at $69, with a printed turnover-checklist card. Realtor closing-gift packs (25 laser-etched at $175) are a second test, unverified. Same 3-week test: 20 DMs to host co-ops, target 3 paid 10-packs.

**3-year version:** a small line of battery-free "event-set" proofs for STR turnover, all on one cutout-and-slug platform. The dishwasher clip comes first, then a linen tag that shows the sheets went through a hot wash. No hygiene claims. Per-property kits for cleaning companies, $2–4M a year; the two-pack stays the funnel.

**Scores.** Innovation **3**: fifty years of prior art; what's new is the time-integrating slug and actually shipping it. Feasibility **8**: one tool, off-the-shelf part, no radio, ~$25k. Market fit **5**: a proven buyer of the stale magnet, but no consumable and $9.99 copies coming. I'd move to 6 only if host 10-packs actually reorder. Not yet earned.
