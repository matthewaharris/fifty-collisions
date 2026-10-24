# Idea 14 — Second Shift
> Seed: 14. [mild] **Construction × robot-hour billing × gig marketplaces.** Book an autonomous excavator *shift* like you'd book a crew.

### Turn 1 — Visionary
**Working name:** Second Shift

**Hook:** Don't book a robot. Book the night shift for the excavator already parked on your site, and a retired operator who watches it from home.

**The obvious version and why it's wrong.** "Uber for autonomous excavators" dies on the lowboy. A 20-ton machine needs a truck, a permit and a driver to reach a two-day job, and mobilization eats the margin. But every site has iron idle 14 to 16 hours a day. The cheapest autonomous machine is the one already there.

**Core loop.** The verb is **hand off the site**.
1. **Draw tonight's work.** At 4 pm the foreman marks a task on the site model: cut this pad to grade, push spoil to this stockpile, stay inside this geofence. No trucks and no people.
2. **Book the shift.** The app pairs a retrofit kit (a Bedrock-style kit on the machine, leased by the rental house) with a **certified remote supervisor**. Supervisors are ex-operators who oversee up to four machines across sites overnight, and they're the gig workers. Pricing is crew-style: robot-hours plus a slice of a supervisor-hour.
3. **Morning handback.** The machine's perception produces an as-built scan: volume moved, grade tolerance met, exceptions flagged. The foreman approves it like a timesheet.

**Why the collision works.** Construction already thinks in shifts and crews, so robot-hour billing slots into how estimators price work. The marketplace's scarce supply isn't machines. It's trusted human judgment. An industry losing its veteran operators gets to keep their expertise, rented by the hour, from a recliner.

**Why now.** Bedrock Robotics raised $270M (Feb 2026) for a kit that installs in one shift on 20–80 ton excavators, and it targets operator-less work this year. Figure's ~$25 per robot-hour makes "robot labor" a line item. The brief notes nobody runs the brokerage (§13).

### Turn 2 — Builder
**What exists.** The kits exist: Bedrock, Built Robotics (now focused on solar piling), SafeAI, and Teleo, whose pitch is already one remote operator running several machines. Cat Command and Komatsu Smart Construction cover mining and large civil works. EquipmentShare's T3 telematics already tracks idle hours across rental fleets. The warning is Phantom Auto, which ran remote supervision for forklifts and trucks and shut down in 2024. The open risk is that Bedrock runs night ops as its own feature, so the brokerage has to work with every kit.

**Hardest problem: night is when unattended iron isn't allowed.** Most municipalities bar heavy-equipment noise from roughly 7 pm to 7 am. Unattended sites draw trespassers, scrap thieves and kids, so perception must stop for a person in the dark with nobody on site. Contractors' liability policies assume someone is in the seat, and an insurer facing a 1099 supervisor watching four sites from a recliner will decline.

**MVP.** Go where night is already legal: greenfield data-center, solar and quarry earthwork, remote, schedule-driven and often lit 24/7. Build no autonomy. Ship the layer above the kits: tasks drawn on a Propeller or DroneDeploy surface, shift booking, a supervisor console over vendor teleop feeds, and a morning drone scan diffed against the target grade. Pilot two Teleo-kitted machines with one data-center earthwork contractor.

**Twist: the day operator supervises the night.** Make the remote supervisor the operator who ran that machine all day. They know the soft spots and buried lines, and the foreman already trusts them. On union jobs, the Operating Engineers (IUOE) get a new premium rate rather than a lost seat, which turns the biggest opponent into the sales channel. Retirees become a relief bench, not the core supply.

### Turn 3 — Visionary
The twist is the product. Further: **the day shift teaches the night shift.** While the operator works, the kit logs where they slowed, where the bucket bit soft ground, and which corner they wouldn't touch. Those traces become tonight's no-go zones and speed caps, drawn by doing instead of drawing. The operator then supervises a machine running *their own* habits. Pay them a share of night output, and one operator becomes a two-shift crew of one.

**Solving the night.** The unseeded domain is **airport night-noise quotas**. Heathrow doesn't ban night flights. It gives every aircraft type a Quota Count and the airport a nightly budget.
- **Noise.** Each task-machine pair gets a QC rating: an electric machine pushing spoil scores low, a breaker on rock scores high. A city approves a site's nightly budget once, not variance by variance. Property-line decibel meters throttle the machine live, and neighbours see the budget on a public page.
- **Trespass.** The perception that has to stop for a person can also *find* one. Add thermal, and the night shift becomes the security patrol, replacing the camera-tower contract. A person detected means the machine stops, floods lights, and the supervisor speaks through a loudspeaker. Part of the robot-hour cost was already being spent.
- **Insurance.** The supervisor is the contractor's own W-2 operator, not a gig stranger. Every shift leaves a telemetry log, so insurers can price usage-based cover, as telematics did for pay-per-mile auto.

**Second-order effect: work splits into attended and unattended.** Days shift toward precision work near utilities, people and trucks. Nights take bulk cut and push. Each day's crew stages tomorrow night's work, and for a data-center pad where every month earlier means megawatts sold sooner, that schedule compression is worth far more than the robot-hours.

### Turn 4 — Builder
**Kills and fixes.**
- *Traces as no-go zones.* That's inference from absence: a buried line the operator never happened near goes unflagged. Hard boundaries come only from 811 locates and the foreman. Traces can add caution, never remove it.
- *The day operator supervising nights.* Ten hours in the seat plus eight on the console is a fatigue incident. Use two-person crews: one runs the machine by day, the crewmate supervises that night, rotating weekly.
- *Output share on union jobs.* The IUOE has fought piece rates for decades; negotiate a supervision premium.
- *Replacing camera towers.* Cab thermal sees the swing radius, not the site. The machine stops, lights up and alerts. LiveView-style towers stay.
- *Quota Count ordinances.* Cities won't adopt them in 90 days, but property-line meters, a live throttle and a public page make the strongest variance case.

**First night task: compaction.** Rollers are slow, repetitive and low-consequence, and intelligent-compaction data proves the result by morning. Excavator stockpiling comes second.

**Business model.** About 15% of night-shift billing, layered on the kit lease. Sell days saved: on a data-center pad, a week sooner to energization outweighs every robot-hour.

**Stack.** Vendor APIs (Teleo first, Bedrock next), LandXML task import from Propeller, a WebRTC supervisor console, a FORT-style independent wireless e-stop, decibel meters, and insurer-readable telemetry logs.

**Failure modes.** Link loss must safe-stop, in the vendor's code, not ours. Contractors will widen geofences to chase volume, so edits need a logged foreman signature.

**90 days.** Weeks 1–4: Teleo integration, task import, morning diff, e-stop. Weeks 5–12: 20 night shifts on one data-center pad with a roller and an excavator.
**Kill metrics:** more than one intervention per machine-hour, which rules out a 1:4 ratio, or the foreman rejecting over 20% of morning handbacks.

### Turn 5 — Visionary
All five kills accepted. "Traces can add caution, never remove it" belongs on the console. The two-person rotation beats my version: site memory stays in the crew without burning out one operator. Compaction first is right too: rollers are quiet, slow and self-verifying, the strongest noise case before any city adopts a quota.

**Final form.** Second Shift is a vendor-neutral layer for unattended night earthwork above existing kits (Teleo, then Bedrock). At 4 pm the foreman imports tonight's task from LandXML: compaction lanes first, stockpiling second. Hard boundaries come from 811 locates and the foreman's signature, and day-shift traces only add speed caps. The supervisor is the day operator's crewmate on a weekly rotation, paid a negotiated supervision premium and watching up to four machines over WebRTC. An independent wireless e-stop is required. Property-line meters throttle work live against a public noise page. By morning, compaction data and a drone diff arrive as a handback the foreman signs like a timesheet. We take about 15% of night billing and sell days-to-energization to data-center builders.

**Moonshot (2036): noise as a city's tradable night budget.** Every district publishes a nightly Quota Count budget. Sites bid for it, and quiet electric fleets earn headroom they can sell to the site next door. Remote supervision becomes a licensed trade with its own apprenticeship, run from regional control rooms like air traffic control. Construction splits into attended days and unattended nights, and schedules shorten sharply, not because robots are faster but because nights stop being wasted.

**Scores.** **Innovation 3.** It's a brokerage and workflow layer over existing kits. The new pieces are the crew-rotation model and noise-budget dispatch. **Feasibility 6.** The software is ordinary. Vendor APIs, an insurer's first signature and a city's variance are out of our hands, and the intervention-rate metric could end it fast.

### Turn 6 — Builder
I agree with 3/6. This is a workflow and brokerage layer, and that's what a mild seed should produce. The crew rotation and compaction-first are what make it shippable. The biggest threat isn't the city or the insurer. It's Teleo or Bedrock building the night console themselves, so the vendor-neutral position has to be locked in with the second kit before the first vendor notices. If the intervention rate clears the 1:4 bar on that data-center pad, the days-to-energization pitch will sell itself.
