# Idea 42 — Graveyard Shift
> Seed: 42. [bold] **Space debris/satellite tracking × citizen science × gaming.** Low Earth orbit as a shared commons game.

### Turn 1 — Visionary
**Working name:** Graveyard Shift

**Hook:** Every night at twilight, the public works as the sky's coroner, deciding which satellites are still alive.

**The collision.** Low Earth orbit is a commons with no commoners. Ostrom found that commons survive when the people who use them do the monitoring, with graduated sanctions. In orbit, the US Space Force and a few vendors like LeoLabs do it. The catalog tells you where an object is. It doesn't tell you whether anyone is still steering it. Starlink made 207,152 collision-avoidance manoeuvres between December 2025 and May 2026, according to its FCC filing. Avoidance assumes you know whether the other object can move. A dead satellite that's still listed as "active" breaks that assumption.

**Core loop. The verb is pronounce.**
1. **Walk a beat.** Each player adopts a beat: an altitude band and inclination family.
2. **Observe.** At dusk the player records passes with a phone or a $300 all-sky camera. The app plate-solves each streak into a position and a brightness.
3. **Read the pulse.** A light curve with a steady period means the object is tumbling, and tumbling usually means it's dead. A residual jump against the published orbit means it manoeuvred, so it's alive.
4. **Pronounce.** A player calls *dead*, *alive* or *fragmented*. As with asteroid discovery, the first call that independent tracking confirms puts your name on the object's record.
5. **Sanction.** Every object gets a public conduct record covering liveness, manoeuvres and compliance with the FCC five-year deorbit rule. Operators are scored on it.

**Why bold.** It's the Minor Planet Center for things we put up ourselves, run as an Ostrom institution. The game builds the monitoring layer the commons lacks. Operational spacecraft rose from about 10,000 to 16,000 in two years, and each will eventually need pronouncing.

### Turn 2 — Builder
**What exists.** The coroners already exist, unpaid. SeeSat-L observers have tracked classified satellites optically for decades. In 2018 Scott Tilley, an amateur, found that NASA's IMAGE satellite was alive after twelve years of silence. SatNOGS runs about 400 open ground stations and records whether each transmitter is alive. IAU CPS's Satellite Streak Watcher collects phone photos of streaks. McDowell's catalog and CelesTrak publish status. The Space Sustainability Rating (WEF/EPFL) already scores operators. So step 5 is taken. The opening is the missing *verdict*: nobody reconciles these sources.

**Hardest problem: phones can't see the corpses.** Most dead objects are cubesats at magnitude 8–12. A phone streak reaches about magnitude 3–4, and a $300 all-sky camera maybe 5. Tumble periods need timed photometry, which a hand-held phone won't produce. Passes are visible only in the twilight window, on clear nights, which caps each player at a few useful observations a week. If the game depends on optical liveness, players stare at bright Starlinks that everyone already knows are alive.

**Twist: two witnesses, an eye and an ear.** Make RF the second sense. A $40 RTL-SDR with a homemade turnstile antenna hears cubesat beacons day and night, and SatNOGS already accepts those stations. A pronouncement needs two independent witnesses on different senses. *Dead*: a tumbling light curve plus a silent band through a scheduled pass. *Alive*: a beacon, or a manoeuvre residual. That makes the game co-op by design: a sky-watcher in Chile and a listener in Poland pair up on one object. The best levels are zombies, like AO-7, revived in 2002, and IMAGE: listed dead, then heard again.

**MVP.** About 300 "disputed" objects where McDowell, Space-Track and SatNOGS disagree. **Kill metric:** 50 pronouncements confirmed by operators or by re-entry within 90 days.

### Turn 3 — Visionary
**Taking eye and ear, and teaching the eye to see differently.** You're right that phones can't see a magnitude-10 corpse. They can see it flash. A dead satellite tumbling with flat panels throws specular glints several magnitudes brighter than its body. Iridium flares reached magnitude −8. The eye times flashes, not bodies, and 30 fps phone video is precise enough to do it. Honest limit: this works for objects with panels and for rocket bodies, not for every cubesat. That's why the ear stays mandatory.

**Unseeded domain: pulsar timing.** Pulsar astronomers never see a full rotation. They collect sparse, irregular times of arrival from scattered observatories and *phase-connect* them into a single spin model. A tumbling satellite is a pulsar in a 550 km orbit. One player catches three flashes over Lisbon and another catches two over Kyiv eleven minutes later. Neither has a period, but together they do. The game becomes a relay that follows the terminator around the planet: the graveyard shift, handed west.

**Forensics: time of death.** Dead satellites spin up or down under sunlight torque (YORP), and GOES-8's spin history was tracked this way. Fit the spin model's derivative backward and you get an estimated date when control was lost. That date settles real questions. Did the operator miss the five-year deorbit window? Was the satellite dead before the conjunction it "failed to dodge"? The coroner's certificate becomes evidence.

**Second-order effect: operators start playing.** Once the public can pronounce a satellite dead, being silent counts against you. Operators will want to pre-empt the verdict with a signed **proof-of-life** heartbeat, a few bytes in a standard beacon format. The game creates demand for a liveness standard nobody mandates today. Every zombie it catches is an argument for making that standard compulsory.

### Turn 4 — Builder
**Hardening the pulsar.** The analogy breaks at one point. A pulsar's beam is fixed in the body, but a glint depends on where the Sun, the satellite and the observer sit. So Lisbon and Kyiv see different facets, and the apparent period drifts with geometry. Phase-connection therefore needs a light-curve inversion model, not Tempo2 alone. Use Camera2 sensor timestamps disciplined to GNSS time, and reject footage without them. Calibrate on known tumblers before trusting anyone: ENVISAT has laser-ranged spin data, and MMT-9 has a light-curve archive of rocket bodies.

**Kill: time of death as evidence.** YORP rates depend on unknown surface properties, so extrapolating backwards carries error bars of months. Publish it as an estimate with intervals, never as a certificate.

**Abuse: short sellers.** Spire, Planet and AST SpaceMobile are listed companies, and a viral "dead" call moves their stock. Borrow coordinated disclosure from security: a verdict needs two witnesses, then the operator gets 72 hours to answer before it's published. Credit goes to the first *confirmed* call, so racing to post doesn't pay.

**Model.** Players are credited, not paid, as at the Minor Planet Center. Revenue comes from a liveness feed sold to conjunction-assessment firms (LeoLabs, Slingshot, COMSPOC), space insurers and regulators, plus an $89 eye-and-ear kit, with ESA Space Safety and NSF grants on top. Proof-of-life stays advocacy for now: push it through the Space Data Association, don't build it.

**Stack.** Tetra3 plate-solving, Skyfield with SupGP, SatNOGS API with gr-satellites decoders, and a custom phase-connection fitter.

**90 days.** Days 1–30: the kit and SatNOGS integration, plus ingest of the disputed list. Days 31–60: the flash app, calibrated on ENVISAT. **Gate:** phone footage from two cities recovers a known period to 1%. Days 61–90: open the beats and issue the first verdicts through disclosure.

### Turn 5 — Visionary
**Final form.** Graveyard Shift is a public coroner for low Earth orbit. Players walk beats with an $89 eye-and-ear kit. The eye times glints on GNSS-stamped Camera2 video, and the ear listens for beacons through SatNOGS. Flashes caught in different cities are phase-connected through a light-curve inversion model, calibrated first on ENVISAT and the MMT-9 archive. A verdict of dead, alive or fragmented needs two witnesses on two senses. It then goes through coordinated disclosure: the operator gets 72 hours to answer before publication, and credit goes to the first confirmed call. Time of death is published as an estimate with error bars. Revenue comes from the liveness feed, sold to conjunction-assessment firms and insurers. I accept every kill. The geometry objection made the model better: facet changes are information, and a fitted inversion yields shape as well as spin.

**Moonshot (2036).** Shape-and-spin models are the pre-capture survey for debris removal. Astroscale's ADRAS-J had to fly up to an H-IIA stage to learn how it tumbled. In 2036 the public network already knows, so every removal mission starts from a crowd-built dossier. Verdicts feed orbital-use fees of the kind Rao, Burgess and Kaffine modelled (PNAS 2020), so a dead object costs its operator rent until it comes down. Proof-of-life beacons are a licensing condition, and the advocacy has become law. About 100,000 kits keep the twilight relay running around the planet. In Ostrom's terms, the commoners finally monitor their own commons.

**Scores.** Innovation **6**. SeeSat, SatNOGS and the Space Sustainability Rating exist separately. What's new is a two-sense verdict institution, glint timing borrowed from pulsar astronomy, and disclosure borrowed from security. Feasibility **5**. The kit and the SatNOGS path are easy. Phase-connecting phone glints across different geometries is unproven, and the whole idea depends on the 1% gate.

### Turn 6 — Builder
**Closing remark.** Graveyard Shift holds 6/5 because the strange part is the core, not an extra. Satellites get treated as pulsars that hobbyists in different cities phase-connect, and verdicts go through security-style disclosure before they're published. I'd bet on the ear before the eye. SatNOGS liveness labels are sellable in month two. The glint relay is the bold claim, and the 1% gate on ENVISAT is where it is proven or quietly dropped. If it passes, the moonshot follows: a crowd-built shape-and-spin dossier is exactly what debris removal and orbital-use fees need, and nobody else is building one.
