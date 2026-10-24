# Idea 23 — Nightside
> Seed: 23. [wild] **Sleep science (targeted memory reactivation) × dreams × collaborative storytelling.** A shared dream-incubation network.

### Turn 1 — Visionary
**Working name:** Nightside

**Hook:** A story that is always being dreamed somewhere. It moves west with the night from sleeper to sleeper, and its canon is whatever independent dreamers keep dreaming.

**Why these collide.** Sleep science now has both channels a network needs. *Write:* targeted memory reactivation replays a cue learned while awake during sleep and biases what gets replayed (Rudoy et al., *Science* 2009). MIT's Dormio steered sleep-onset dreams toward a spoken theme (Haar Horowitz et al., 2020). *Read:* Konkoly et al. (*Current Biology*, 2021) held two-way exchanges with lucid dreamers in REM through eye signals. Storytelling supplies the protocol. The isomorphism: the Haudenosaunee dream-guessing festival, where a community interpreted and enacted members' dreams, rebuilt as a gossip protocol whose only channel is sleep.

**Core loop. The verb is incubate.**
1. **Rehearse.** In the evening you hear the latest chapter, with each canonical element (a character, a door, a city) paired to its own sound.
2. **Cue.** A headband and earbuds detect sleep onset or REM and replay tonight's commissioned cues.
3. **Report.** Prompted wakings capture several voice reports a night; an LLM extracts motifs.
4. **Converge.** A motif becomes canon only if dreamers who got the same cue, blind to each other's reports, dream it independently and beat a shuffled-cue control. Everything else is apocrypha. The updated cues pass to the next time zone.
5. **Vote live.** A small corps of lucid dreamers gets Konkoly-style yes/no questions inside REM ("Does she open the door?") and answers with their eyes.

**Honesty clause.** No telepathy. Maimonides' 1960s dream-telepathy studies failed to replicate. Convergence comes from shared cues, and the network measures exactly how much.

**Why now.** The FDA's Jan 2026 wellness guidance gives closed-loop audio wearables room (brief §9). Apple's BCI HID makes neural signals a standard input (§15).

### Turn 2 — Builder
**What exists.** MIT's Dormio glove is open source. Elemind sells a closed-loop audio headband, and Muse S stages sleep at home. Paller's lab ran targeted lucidity reactivation in people's homes with sound cues and an app (Carr, Konkoly et al.). REMspace and Prophetic sell lucid-dreaming tech with bold claims. DreamBank holds about 20,000 scored reports. Nobody runs a story through many sleepers, and nobody measures transmission at scale.

**Hardest problem: the channel has almost no bandwidth, and the test is confounded.** REM cue incorporation is weak and inconsistent, and consumer headbands stage REM poorly. Several forced wakings a night wreck sleep; volunteers tolerate a week, not a season. The convergence test as written measures day residue. Everyone heard the chapter at dinner, so of course they dream the door.

**Twist: rehearse everything, cue half.** Every evening, everyone rehearses *all* candidate elements, each paired with its sound. Overnight, each sleeper is randomly cued with only half of them. Day residue cancels, leaving the *sleep effect* per element. Every night is a crowd-scale randomized trial, and canon is the set of elements with a measured incorporation lift. The plot becomes natural selection by sleeping brains: the story evolves toward whatever the brain preferentially replays. That's stranger than authorship, and it yields the largest TMR dataset ever. Move the main channel to **sleep onset** (Dormio-style, 20–30 minutes of hypnagogic loops at bedtime). It's reliable, needs no REM detection, costs no sleep and yields several reports per session. REM stays with the lucid corps.

**MVP.** With a sleep lab's IRB, recruit 60 dreamers across three time zones for four weeks, using Dormio-style kits or Muse S plus earbuds, randomized cues, voice reports, LLM motif tagging and blind human raters. Exclude PTSD and parasomnias.
**Kill metric:** no element shows a significant lift for cued over uncued.

### Turn 3 — Visionary
Twist taken, and it shrinks nothing: it makes the story *evolve*. Half-cueing turns every night into a trial, sleep onset buys reliability, and REM stays with the lucid corps. But selection is only one-third of evolution. It also needs variation and heredity.

**Unseeded domain: population genetics.** Story elements are alleles, and sleepers are hosts.
- **Fitness.** An element's incorporation lift, cued vs. uncued.
- **Mutation and heredity.** Dreams distort, so the door comes back as a trapdoor. A variant that recurs across independent sleepers is spun off as a new element with its own sound, rehearsed the next evening and entered into the randomization.
- **Recombination.** Cue two elements in one loop. If "door + river" fuses into a chimera that shows up more often than either alone, that's epistasis, and the chimera enters the canon.
- **Demes.** Each time zone is a population and the westward handoff is migration, so regional variants of the story diverge and reconnect.

That answers the bandwidth objection. One sleeper is a terrible channel, but a population is a good search engine even when every individual is noisy. Evolution never needed a high-bandwidth genome per organism.

**Second-order effects.**
- **The plot reveals the prior.** The canon climbs the fitness landscape of the sleeping brain's generative model, and that landscape is the scientific map. TMR helps weakly learned items most (Creery et al., 2015), so mid-salience elements may beat strong ones. Surprising winners are findings.
- **Threat drift.** Revonsuo's threat-simulation theory predicts selection for danger, so the story will darken by itself. Track valence as a trait rather than only censoring it. The darkening rate tests the theory and drives a brake: cap the frequency of high-threat alleles.

### Turn 4 — Builder
**Kills and fixes.**
- *Designed recombination.* Pairs grow combinatorially; 60 sleepers can't power an interaction. Chimeras enter only as spontaneous mutants; epistasis waits for thousands of sleepers.
- *The sound is the confound.* A bell cue makes people dream a bell. Counterbalance sound-to-element pairings across sleepers (a Latin square), and score only semantic content.
- *LLM motif extraction.* It will find doors everywhere. Use a motif dictionary pre-registered each night, embedding clusters for mutants, and blind human raters on a sample.

**Self-defence for free.** Dreamers will fake reports to push favourite elements. Nobody knows which half they got, so a fake inflates both arms and the lift holds. Exception: a cue heard awake, so ask each morning and drop those nights.

**Abuse.** The real abuse is advertising. Coors ran a "dream incubation" campaign in 2021, and dozens of sleep scientists signed a letter against it. The charter bans brands and paid elements, and every element passes public review.

**Ethics.** Threat drift risks nightmares. Exclude nightmare disorder, PTSD and minors; each sleeper sets a valence ceiling. Dream reports are intimate: scrub them, consent per use, never sell.

**Sustainability.** A research instrument plus a serial, not a startup: a consortium of sleep labs pays for the open dataset, with grants on top. Dreamers buy kits at cost. The canon ships as a podcast and book crediting every sleeper: the first novel written by sleep.

**Stack.** A Dormio fork or Muse S over BLE, earbuds, Whisper, server-side nightly randomization, and a hierarchical Bayesian lift model (random effects: sleeper, sound, night).

**90 days.** Weeks 1–3: IRB, the kit, 12 pre-registered elements. Weeks 4–8: 60 dreamers in two demes, four nights a week. Weeks 9–12: the first selection round, the first mutants and Chapter 1.
**Kill metrics:** no lift, or under half of dreamers still adhering by week 6.

### Turn 5 — Visionary
All kills accepted. The Latin square matters most: without it we'd be selecting bells, not stories. Randomization doubling as fraud defence is elegant. One refusal: "two demes" is the MVP, not the form. The westward relay and lucid corps are the wild core; they stay, staged behind the evidence.

**Final form.** Nightside is a story evolved by sleeping brains.
- **Rehearse.** Each evening, every candidate element is heard with its counterbalanced sound.
- **Cue.** Dormio-style sleep-onset loops play a random half of the elements each night.
- **Score.** A pre-registered motif dictionary and blind raters score voice reports; a hierarchical Bayesian model estimates each element's lift, its fitness.
- **Evolve.** Spontaneous mutants that recur across sleepers become new alleles. Each time zone is a deme, and the canon passes west with the night.
- **Vote.** A lucid corps answers plot forks with their eyes from inside REM.
- **Govern.** No brands, personal valence ceilings, a cap on threat frequency, and scrubbed, consented data.
- **Publish.** An open dataset, plus a serial crediting every sleeper.

**Moonshot (2036): a planetary dream observatory.** Tens of thousands of sleepers keep one story dreamed around the Earth. Divergence between demes gives the first measured map of which dream content is universal and which is cultural. Selected benign elements become therapy cues, extending TMR-boosted nightmare therapy (Schwartz et al., *Current Biology* 2022). With better REM detection, dozens of lucid dreamers get the same beat at once, and their eye signals form a real-time consensus: the first assembly convened inside sleep.

**Scores.** **Innovation 8.** It treats collaborative fiction as an evolutionary screen of the sleeping brain's generative prior, run as a relay around the planet. **Feasibility 4.** Every component has been published, but a crowd-scale lift from consumer kits, season-long adherence and a working lucid corps are all unproven.

### Turn 6 — Builder
Keeping the relay and the lucid corps staged behind evidence is right. They're the wild core, and the half-cue trial is the ladder up to them, not a replacement. Innovation 8 stands: a story whose plot is selected by measured incorporation lift across sleeping populations is a new instrument, a new art form and a new dataset at once. Feasibility 4 also stands. A 60-person, four-week trial is buildable on published kits with a lab's IRB, but nobody knows whether home sleep-onset cueing produces a lift the crowd can detect, or whether dreamers stay past week six.
