# Idea 46 — Mound
> Seed: 46. [wild] **Swarm robotics × termite architecture × construction.** Buildings that grow rather than get built.

### Turn 1 — Visionary
**Working name:** Mound

**Hook:** A building with no final drawing. A resident swarm keeps growing it out of local soil, and the people living inside act as the pheromone.

**The isomorphism.** Construction fixes the design at the moment of greatest ignorance, before anyone has lived there. A *Macrotermes* colony never does, because for termites building, living and maintenance are one process. No termite holds a plan. Each reacts to local cues such as CO₂, humidity and the shape of fresh mud. Turner's and Ocko's measurements show the mound works as a lung driven by daily temperature swings. It is homeostasis made solid. Desire paths show that humans already build by stigmergy.

**Core mechanic. The verb is *tend*.**
1. **A genome instead of a blueprint.** The architect writes rules, not shapes: deposit where strain is high and people linger, remove where nobody goes, open a channel where CO₂ pools.
2. **A resident swarm.** Hundreds of palm-sized climbing robots carry site soil wetted with a microbially induced calcite precipitation (MICP) solution, which bacteria harden into sandstone. Each senses only what it can reach.
3. **Occupants as pheromone.** Footfall, body heat, breath and one gesture (two taps on a wall means "more here") write the field that the swarm reads.
4. **Metabolism.** Material flows both ways. An unused room thins and feeds the rooms people do use. The building is never finished, only tended.

**Why wild.** It's a new institution. Cities would permit genomes, not drawings, and inspect metabolisms, not as-built plans. A dead swarm would mean a building that has started to age. Honest caveat: MICP sandstone is weak (single-digit MPa) and releases ammonium.

**Why now.** Bedrock Robotics (Feb 2026) and Figure's reported $25 robot-hour put a price on robot labour. Harvard's TERMES (2014) proved local building rules with a handful of robots.

### Turn 2 — Builder
**What exists.** TERMES (2014) built a fixed target from purpose-made bricks with three robots. IAAC's Minibuilders (2014) climbed and printed a shell. Mediated Matter grew the Silk Pavilion and Fiberbots. Biomason sells MICP bricks, factory-cured rather than wetted on site. ICON and Branch Technology print buildings, but from a fixed file. Eastgate Centre (Harare, 1996) borrowed the mound's ventilation, then froze it in concrete. No structure has kept changing shape after people moved in.

**Hardest problem: the verb *remove*.** Throughput isn't the bottleneck: 300 robots carrying 50 g every three minutes move about 3 t a day, and curing paces each layer more than hauling does. The bottleneck is that a swarm which thins unused walls is rewriting the load path every night, out of rock with the strength of weak sandstone. No engineer can sign that, and no insurer will cover it.

**Twist: a compression-only genome.** Masonry stands by shape, not strength. Heyman's safe theorem says an unreinforced structure is safe if any thrust line fits inside its section. Nubian vaults, Gaudí's chains and ETH Block Research Group's thin shells all stand on weak material for that reason. So the genome only admits funicular forms. Every deposit or removal has to keep a local thrust check inside the section. Each robot estimates it from embedded strain tags and nearby curvature. Weak MICP becomes acceptable, and "permit the genome" gets concrete. The city certifies an invariant, *every reachable shape stands*, the way you'd verify a program, and the inspector audits that invariant instead of a drawing.

**First step on Monday.** A simulator: run real footfall and CO₂ traces through candidate genomes, and check every intermediate shape with COMPAS thrust-network analysis. After that, a 3 m vault on an arid campus, grown by 50 robots over a single season.

### Turn 3 — Visionary
**Taking compression-only and giving it a skeleton.** I accept it: it keeps the strangeness and turns the permit into a proof. One correction: a thrust line is a global property, and a robot can't compute it from curvature. It doesn't need to. In masonry, an unsafe step shows up locally first, as a face going into tension: a hinge starting to open. So each robot's rule is "never open a hinge", and that can be read from the strain tags.

**Unseeded domain: bone.** Bone is the one structure that already thins unused material and thickens loaded paths using only local signals. In Zurich in 1866, the engineer Culmann saw the anatomist Meyer's femur sections and recognised his own crane's stress trajectories. Wolff's law (1892) grew out of that meeting. Mound borrows two of bone's mechanisms:
1. **The mechanostat (Frost).** Deposit above an upper strain set point, resorb below a lower one, and do nothing in the dead band between them. Remodelling simulations under rules like this converge towards load-path forms without any global solver.
2. **Coupled remodelling units.** Bone couples removal to refilling: osteoclasts and osteoblasts travel together as one tunnelling unit. In Mound, removal is a moving cavity of bounded size: bite, measure, then refill or advance. The permitted invariant becomes N−1: *every reachable shape stands with one cavity anywhere in it.* That can be checked offline, once, in the simulator.

**Occupants vote with weight.** All human input becomes load. Want a bench? Sit there often. Want a thicker wall? Lean on it. There is one signal, strain, and you can't fake it from a phone.

**Second-order effect: disuse atrophy.** Like an astronaut's bones, an empty Mound loses mass. A vacant building returns to soil, and its material migrates towards occupied neighbours. Land banking becomes physically impossible, and urban mass flows to where people actually are.

### Turn 4 — Builder
**Kill: "vote with weight."** Bone works because muscle forces dwarf its own weight. A vault is the opposite: under self-weight, stress doesn't depend on thickness, so a strain mechanostat has no fixed point, and a sitting adult is noise. Strain stays the *safety* rule. Growth reads presence (CO₂, heat, footfall) sensed on-robot, with no cameras.

**Kill for now: cross-lot migration.** Moving material across title lines is a property-law fight for after 2035. Atrophy stays inside the lot.

**Failure modes.** *Earthquakes:* the safe theorem covers gravity only, and unreinforced masonry is the deadliest building type. The N−1 invariant must also hold under 0.2 g lateral load, and early sites are low-seismic. *Rain:* the genome reserves a sacrificial skin the swarm re-crusts after storms. *Ammonium:* every rinse is captured and nitrified on site, and indoor NH₃ is monitored. *Egress:* doors and their clear widths are axioms enforced in firmware, beneath the genome.

**Abuse.** A hacked swarm is a drill inside your house. Genomes are signed, and the hinge and egress interlocks run on a separate microcontroller the genome can't override.

**Model.** Start where earth vaults are already legal and cheap. The Sahel's Nubian Vault Association has trained masons for thousands of vaults, and livestock shelters and grain stores want passive cooling. Revenue: tending subscriptions per square metre, plus genome certification, which works like a UL listing.

**Stack.** COMPAS thrust networks, Frost-style remodelling FE, *S. pasteurii* MICP on site soil, passive RFID strain tags read by robots walking the crown.

**90 days.** Days 1–30: the simulator proves the N−1 invariant under presence traces. Days 31–60: site-soil MICP strength, NH₄⁺ per kg, and rain erosion. Days 61–90: a 1:1 arch with 10 robots. **Gate:** 100 bite-refill cycles without a hinge, and the tags catch induced hinges in 10 of 10 sacrificial tests.

### Turn 5 — Visionary
**Accepting both kills.** A thicker vault isn't a less-stressed one, so strain becomes the conscience and presence the appetite. The sacrificial skin is pure termite: *Macrotermes* re-crust their mound after every rain.

**Final form.** Mound is a compression-only earth building with a resident swarm and no final drawing. The architect writes a signed **genome**. Growth follows presence (CO₂, heat, footfall). Every change is a bounded bite-and-refill cavity, borrowed from bone's coupled remodelling. Tension at a face, the first sign of a hinge, vetoes any move. The city never approves a shape. It certifies one invariant: *every reachable shape stands with one cavity anywhere in it, under gravity and 0.2 g lateral.* Egress and hinge interlocks run on firmware the genome can't touch. The tending subscription replaces the handover. When the swarm stops, the building starts to age. First sites are grain stores and livestock shelters in the Sahel, built where Nubian vaulting is already legal, trusted and taught.

**Why the tier holds.** It moves permitting from drawings to invariants and turns construction into an open-ended metabolism.

**Moonshot (2036).** Building codes ship as libraries of verified invariants, and any genome that satisfies them is legal anywhere the soil allows. Neighbourhoods opt into a community land trust whose mass is held in common, so atrophy can cross lot lines by consent and empty streets fade back into the ground. Archaeologists read a town's history from its wall sections, as anthropologists read a life from bone.

**Scores.** Innovation **8**. The parts exist, but nobody certifies a process instead of a product or lets a building remodel after move-in. Feasibility **3**. The simulator and the 1:1 arch are Monday work. Field swarms that run for years, in-situ MICP and an invariant-based permit are each unproven.

### Turn 6 — Builder
**Closing remark.** Mound holds 8/3. The strange core survived: a building with no final drawing, permitted as an invariant rather than a shape, which keeps remodelling after people move in. The Heyman-plus-bone combination is what makes it more than a provocation. Shape carries the safety, so weak earth is acceptable, and bounded bite-and-refill cavities turn "every reachable shape stands" into something a simulator can check before any robot moves. The honest risk is the field. A swarm that survives years of dust, rain and dead batteries is unproven, and so is in-situ MICP at building scale. I'd fund the simulator and the arch gate, then a single Sahel grain store, before anyone sleeps inside one.
