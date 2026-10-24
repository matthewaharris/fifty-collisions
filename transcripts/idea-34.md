# Idea 34 — Sonotype
> Seed: 34. [bold] **Bioacoustics × collecting games × biodiversity bounties.** Catch 'em all, but it's real insect and bird calls feeding science.

### Turn 1 — Visionary
**Working name:** Sonotype

**Hook:** A Pokédex with two columns, *Heard* and *Named*. The rarest card is a call that no scientist has matched to a species.

**The isomorphism.** Tropical bioacoustics already has the Pokédex's seen/caught split. A **sonotype** is a distinct call whose singer is unknown. Krause's acoustic niche hypothesis (partly supported, contested) says species divide up the frequency × hour-of-night plane. That turns each habitat's binder into a Mendeleev table: the empty cells are predictions.

**Core loop. The verb is pair.**
1. **Record.** Leave a phone or AudioMoth out overnight. Perch embeddings cluster the calls into sonotypes. Known calls become *Named* cards. Unknown ones become silhouettes in *Heard*, pinned to their cell in the niche grid.
2. **Pair.** You catch a silhouette by tying it to a voucher: synced video of the animal calling, a specimen, or a playback response. Experts confirm.
3. **Bounties.** Museums, agencies and nature-credit buyers post bounties on silhouettes and on **silences**. A silence is a grid cell that the habitat predicts should be occupied but that stays empty across ten nights. That is the repeat-visit non-detection data occupancy models need and nobody collects. Here absence is a card.
4. **Provenance.** The first person to hear a call and the person who pairs it split the bounty, and the named pair goes to Xeno-canto and GBIF.

**Why the domains collide.** Collecting games are completionism engines, and bioacoustics' bottleneck is labelled insect and frog calls. Completionism aimed at blanks hits that bottleneck.

**Why now.** Perch 2.0 (Aug 2025) covers about 14,600 species across many taxa, insects included, so clustering unknown calls is cheap. The brief (§14, §16.7) shows carbon markets moving only once measurement could be trusted, and the same applies to biodiversity. Repeated nights of verified silence are a building block for that measurement.

### Turn 2 — Builder
**What exists.** Every piece except the binder. BirdNET and Merlin's Sound ID turned millions of phones into recorders. iNaturalist takes sound uploads, and Seek hands out badges. Rainforest Connection's Arbimon already clusters unknown calls into "soundscape" patterns for researchers, and Zooniverse's Bat Detective crowdsourced ultrasonic labels. XPRIZE Rainforest (2024) proved acoustic surveys at speed. Nature-credit frameworks (Verra Nature, Plan Vivo PV Nature, Wallacea Trust) want verified measurements and have no supply of them. Nobody has made the unknown call the prize.

**Hardest problem: pairing, and paid silence.** Matching an unknown nocturnal katydid to its singer takes a specialist, a torch and luck. Phones also cut off near 20 kHz, and many katydids sing above it. Then there's silence. A bounty for non-detection pays best for a dead microphone, a rain-soaked night or a recorder in a sock drawer. Absence data only means something if detection probability is known.

**Twist: the call is the Poké Ball.** Crickets, katydids and frogs do **phonotaxis**: they answer or walk toward their own species' song. So the catch mechanic is playback. A kit (AudioMoth plus a cheap IR camera and speaker) plays the silhouette's call at intervals and records whatever answers or approaches. The responder is the candidate singer, on video, timestamped and geolocated. Experts confirm a clip instead of running a field trip. Playback also fixes silence: each night the kit plays a calibration tone and a common local caller's song. A night counts as silent only if that positive control was detected. Playback is capped and limited to invertebrates and frogs, never threatened birds.

**MVP.** One tropical field station, 150 AudioMoth kits for a local naturalist club, Perch 2.0 clustering, a silhouette binder, and one museum orthopterist posting the first ten bounties.
**Kill metric:** no silhouette paired by playback and confirmed by an expert within 90 days.

### Turn 3 — Visionary
**Taking playback, and making it a challenge-response (unseeded: authentication).** A responder walking toward a call isn't proof yet. Female crickets and katydids, though, have narrow **preference functions** tuned to their own species' pulse rate. So the kit sends a challenge set: the true silhouette call, plus two decoys shifted ±30% in pulse rate, in random order. A real match answers the true call and ignores the decoys, like a login that rejects wrong passwords. Three nights of correct answers make a pairing with a measured false-positive rate. Pairing stops being luck and becomes a protocol.

**Silence becomes a relay.** A kit that hears a silhouette passes the call to the next kit in the mesh, which plays it the following night. If the second kit's positive control passes and nothing answers across repeated challenges, that is a calibrated absence, with detection probability measured rather than assumed. The call moves across the landscape, building a range map for a species that has no name yet. The edge of that map is a silence card, and an edge that moves over the years is a climate signal.

**Emergent: the Poké Ball also catches the hunter.** Parasitoid flies such as *Ormia ochracea* and gleaning bats hunt by eavesdropping on these same songs, so playback draws them in on camera too. The binder grows **edges** as well as species: who eavesdrops on whom. Interactions are far scarcer data than species lists, so they pay bigger bounties.

**Second-order: splits.** When the decoy test cleanly separates two clusters inside one Named species, that may be a cryptic species. North American *Gryllus* crickets and *Chrysoperla* lacewings were found this way, by song first. Players get a Split card, and the bounty is a type description in which the song is the primary evidence.

### Turn 4 — Builder
**Kills and fixes.**
- *Decoys at ±30%.* Pulse rate in ectotherms tracks temperature closely, and female preference shifts with it. A fixed decoy can be the true song on a cold night. Kits log temperature and offset decoys from the corrected call. Spacing is calibrated per taxon on Named species first, which is the protocol's own positive control.
- *A mesh.* AudioMoths have no radio. A cloud relay does the job: a Raspberry Pi kit with LTE, about $250. Ultrasonic speakers cost hundreds more, so v1 takes only sonotypes under 20 kHz: crickets, frogs, some katydids.
- *Moving calls across the landscape.* That broadcasts foreign songs into local populations. Relay only within one watershed, under the research permits playback already needs in protected areas.
- *Split bounties paid on a type description.* That takes years and permitted specimens (Nagoya rules abroad). Pay when a partner taxonomist confirms a split candidate by DNA barcode. Players never collect.

**Abuse.** Faked pairings use internet recordings, so the server issues a fresh challenge order each night as a nonce, and kits sign their audio with device keys, so a pre-recorded "response" can't match. A covered mic fails its positive control. The worst abuse is **poaching**: rare frogs sell in the pet trade. Sensitive taxa get 10 km grid cells, as GBIF already does.

**Money.** Play is free. Revenue comes from monitoring-as-a-service for nature-credit projects and for TNFD corporate disclosure, priced per calibrated site-night. Bounties carry a 15% fee; kits rent to clubs. GBIF and NSF grants fund the open data.

**Stack.** Perch 2.0 embeddings, HDBSCAN sonotypes, Pi Zero 2 kits with IR cameras and signed uploads, and occupancy models for detection probability.

**90 days.** Weeks 1–4: kits and challenge protocol, calibrated on five Named crickets. Weeks 5–9: 150 kits at one station. Weeks 10–12: first decoy-verified pairings and silence cards.
**Kill metric:** decoy discrimination fails on Named species.

### Turn 5 — Visionary
**Final form.** Sonotype is a two-column binder for the night chorus. Signed Pi kits with an IR camera and a speaker record overnight. Perch 2.0 and HDBSCAN split each night into *Named* cards and silhouettes pinned on a frequency × hour niche grid. To **pair** a silhouette, the kit runs a challenge-response: the true call plus temperature-corrected decoys, in a fresh order each night set by a server nonce. A responder that picks the true call three nights running goes to an expert as a candidate. A **silence** card is a challenged cell inside one watershed where the positive control passed and nothing answered. Eavesdropping flies and bats on camera become **edge** cards. A **split** candidate pays when a partner taxonomist confirms it by DNA barcode. Revenue comes from calibrated site-nights sold to nature-credit and TNFD buyers. I accept every kill. The watershed limit even helps: range maps are built from local silences, not from broadcast songs.

**Moonshot (2036).** The protocol becomes the unit of account for nature. "Playback-verified presence" and "calibrated silence" turn into standard measurement units, as tonnes of CO₂ did for carbon. Biodiversity credits are then priced on detection-corrected occupancy instead of habitat-area proxies. The unnamed-insect backlog gets a song-first pipeline, with silhouettes ranked by the bounties waiting on them. Taxonomy accepts the **acoustic holotype**, a signed, challenge-verified recording beside the barcode. Edge cards add up to a continental eavesdropper food web, and moving silences become an early climate range-shift alarm.

**Scores.** Innovation **6**. Arbimon, BirdNET and iNaturalist exist. What's new is the unknown call as the prize, phonotaxis turned into a cryptographic challenge-response, and silence made into a verified, bountied asset. Feasibility **5**. The hardware is cheap and the science of preference functions is established. Permits, enough taxonomists to review pairings, and nature-credit demand set the pace.

### Turn 6 — Builder
Innovation 6 and feasibility 5 stand. The single move that earns the score is using the animal's own preference function as the authenticator. It turns a citizen-science lucky shot into a protocol with a measured false-positive rate, and it makes silence something you can pay for without paying for broken microphones. The real constraint is the market, not the hardware. Nature-credit and TNFD demand for calibrated site-nights is still forming, so grants and a single museum partner have to carry the first two seasons.
