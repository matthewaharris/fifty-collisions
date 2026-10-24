# Idea 38 — Tally
> Seed: 38. [mild] **Mining safety × wearables × edge AI.** Underground, offline, life-critical.

### Turn 1 — Visionary
**Working name:** Tally

**Hook:** The brass tag board, carried on every cap lamp. Any one lamp that reaches the surface brings back the whole crew's last known positions.

**The problem, concretely.** On September 8, 2025, about 800,000 t of wet material rushed into Grasberg Block Cave. Freeport first said it knew where the seven trapped workers were. Three days later it said it hadn't been able to locate them. Recovery took until October 6. Tracking in modern mines relies on leaky feeders, Wi-Fi nodes and fixed readers, which are the first things a mud rush, fire or collapse destroys. The data disappears with them.

**Core loop. The verb is gossip.**
1. **Encounter.** Every cap lamp gets a BLE-plus-LoRa tag. When two miners pass, or a miner passes a cheap battery beacon, the lamps swap compact records: who, where, when, gas readings, and motion state.
2. **Carry.** Each lamp holds a rolling tally of the *entire* crew, a few kilobytes. This is delay-tolerant networking (unseeded: ZebraNet, which tracked zebras by gossiping between collars and had no infrastructure). Data moves with people, not wires.
3. **Infer.** On-device models handle the gaps: step-based dead reckoning between beacons, man-down and immobility detection, and a sudden-airflow or pressure-wave event flag.
4. **Reconcile.** At the portal, any lamp syncs the tally. After an event, the first lamp out, a refuge chamber's reader or a rescuer's handheld gets every last-seen record that any survivor carried.

**Daily value, so it's maintained.** At end of shift the tally reconciles headcount automatically, replacing the paper or brass check-in board. It also logs vehicle near-misses.

**Why now.** Copper is at record highs, Grasberg is offline until late 2027 (§12), and critical-minerals policy is pushing new and deeper mines into places without legacy infrastructure. Edge ML on BLE chips is commodity.

### Turn 2 — Builder
**What exists.** Crowded field, one gap. Newtrax (now part of Sandvik) runs battery-powered BLE mesh tracking. Strata's CommTrac is a node mesh, and its nodes carry backup batteries because the US MINER Act requires post-accident tracking. MST's PED system has done one-way through-the-earth paging for decades. Roobuck and NLT put tags in cap lamps. On the hobby side, **Meshtastic** already does LoRa mesh with store-and-forward on $30 boards. Nobody lets the *people* carry the data once infrastructure is gone.

**Hardest problem: you can't test it without a disaster, so it rots.** Disaster-only systems decay quietly: beacons knocked off by loaders, batteries dead, lamps left uncharged. There's also an honest limit. Lamps of trapped miners don't come out, so the tally knows where people were *before* the event, not after. Its value is the minutes and hours that sparse fixed readers miss.

**Twist: fake a collapse every shift.** Each night the server picks a random T0 in the shift that just ended. It throws away all infrastructure data after T0 and every lamp that wouldn't plausibly have escaped. Using only gossip the survivors carried out, where was each person? Compare with what actually happened. The output is a **dark map**: drives and stopes where the tally goes blind, with a recommended beacon drop for each. Coverage becomes a number the shift boss sees every morning, and maintenance follows the number. The system is drilled 300 times a year without anyone getting hurt.

**MVP.** One non-gassy hard-rock mine (no permissibility certification). Sixty belt pucks (nRF52840 + SX1262, Meshtastic fork), forty beacons and a portal gateway. In the first month it does end-of-shift headcount, then the nightly drill.
**Kill metric:** over 30 shifts, the drill places under 90% of the crew on the correct level at a random T0.

### Turn 3 — Visionary
**Taking the nightly drill, and closing the gap it exposes.** The tally goes stale at T0; two additions narrow that window.

**The T0 flush.** Lamps carry pressure sensors and accelerometers. When one detects an event signature, such as a pressure wave, a sudden change in airflow, or several lamps going immobile at once, it sends its record and everything it holds as a single high-power LoRa burst. Every lamp in range stores it first. The escapers then carry out a snapshot from *the event itself*, not from twenty minutes earlier. The nightly drill models this too: at a random T0, which lamps would have heard which flush? The dark map gains a second layer, **flush shadows**.

**After T0: lamps become avalanche transceivers (unseeded: backcountry rescue).** Ski tourers carry 457 kHz beacons: low-frequency magnetic fields pass through snow, and rescuers follow the field lines to a buried partner. After a flush, a cap lamp drops into the same mode: a low-frequency magnetic ping carrying a few bytes (ID, and whether it's moving). A 15 Wh lamp battery keeps that going for days. To be honest about range: through wet fill it's tens of metres, not hundreds. It won't speed up digging through 800,000 t, but it tells rescuers *where* to dig, which Grasberg's rescuers couldn't say for weeks.

**Rot, handled the avalanche way.** Ski tourers run a beacon check at the trailhead every morning. The portal gateway does the same: a lamp whose flush radio or magnetic ping fails its check doesn't get the green light.

**Second-order: desire paths (urban planning).** Gossip records how people actually move, which rarely matches the designated escapeways. Overlaying the dark map shows where real routes and rescue plans diverge. Self-escape training is rebuilt around real routes, and time-to-refuge per crew becomes a weekly number.

### Turn 4 — Builder
**The flush stays, and blasting calibrates it.** Mines blast every shift, so feed the detector the firing schedule: each scheduled blast is a free daily test, and anything unscheduled is a candidate event. Simultaneous flushes use random backoff, and a 200-person crew fits in about 6 kB.

**Kill: the 457 kHz ping.** In saturated, conductive fill (σ ≈ 0.5 S/m) the skin depth at 457 kHz is about a metre. Through-the-earth systems use a few kHz for that reason, with antennas no cap lamp can hold. The ping moves to phase two as a low-kHz pinpointing aid for rescuers' handhelds and refuge chambers, good to perhaps ten metres.

**Failure modes.**
- Misdirected rescuers: the tally is labelled supplementary to statutory tracking, with drill-reported confidence.
- Pucks left in the crib: they show up on the dark map as a person who never moved.
- Rot: the portal check runs in the lamp-room charging rack, where the lamps already sit.

**Ethics: desire paths are also a surveillance feed.** Management will want tracks for productivity discipline; unions will rightly fight it. The data-use agreement limits it to safety, the drill and aggregate paths. The joint health and safety committee holds the key to individual tracks.

**Stack.** RAK WisBlock (nRF52840 + SX1262). Meshtastic is GPL-3, so the firmware stays open and the drill server is the product. CBOR records, TimescaleDB, and a TinyML event model.

**Business.** $10 per lamp per month plus hardware at cost. Firmware is licensed to lamp OEMs (Roobuck, NLT), which sidesteps Newtrax's bundling.

**90 days.** Days 1–30: boards, firmware and a site agreement signed by the H&S committee. Days 31–60: one level, headcount checked against the tag board, blast data logged. Days 61–90: drill and dark map go live, and the kill metric is measured.

### Turn 5 — Visionary
**Final form.** Tally is open firmware for cap-lamp pucks (nRF52840 + SX1262) plus a drill server, sold as a product. Lamps gossip crew records whenever they pass each other or a beacon, so every lamp carries a 6 kB tally of the whole shift. When a lamp detects an event signature, it sends a **T0 flush**. Scheduled blasts give the detector a free test every day, and anything unscheduled counts as a candidate event. Each night the server picks a random T0 in the shift that just ended, keeps only what escaping lamps would have carried, and scores placement against reality. The output is a **dark map** of blind drives and flush shadows, with a recommended beacon drop for each. Lamp-room racks run the morning check. The H&S committee holds the key to individual tracks. It's labelled supplementary to statutory tracking, with the drill's own confidence. The 457 kHz kill stands: a one-metre skin depth is physics. Pinpointing moves to low-kHz handhelds later. The unseeded idea it really runs on is **chaos engineering**: like Netflix's Chaos Monkey, but for a mine: fail the infrastructure on paper nightly so the real failure finds nothing new.

**Moonshot (2036).** Regulators require a post-accident tally score the way they now require a lost-time injury rate, published per mine. Autonomous loaders and drill rigs act as data mules, and they drop the beacons the dark map asks for on their next pass, so coverage repairs itself. The open standard spreads to tunnelling and cave rescue. Rescue teams carry one reader that works with any vendor's lamps.

**Scores.** Innovation **4**. Gossip meshes and tag tracking exist. A nightly synthetic-disaster drill that turns coverage into a maintained number is new to mining. Feasibility **7**. Off-the-shelf boards, one cooperative mine, a kill metric within 30 shifts.

### Turn 6 — Builder
**Closing remark.** I'm scoring this 3/6, not 4/7. The nightly drill is a strong idea, but it's chaos engineering moved underground and built on parts that already exist, which is exactly where the OACP anchor sits. Getting underground access, H&S committee sign-off and radio approvals into 90 days depends on finding one unusually cooperative mine. What I like is that it's honest about what it can't do: it tells rescuers where people were at T0, not where they are now, and it proves even that every morning instead of hoping it's true.
