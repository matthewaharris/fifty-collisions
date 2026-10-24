# Idea 27 — Ventris
> Seed: 27. [bold] **Language learning × ARGs × geolocation × AI agents.** A city-scale alternate reality game where NPCs only speak your target language.

### Turn 1 — Visionary
**Working name:** Ventris

**Hook:** Don't study Portuguese; decipher it. Your city is the Rosetta Stone: every place is a page of parallel text.

**The stranger second idea.** "NPCs who only speak Portuguese" is Duolingo with a map. The deeper collision: **acquiring a second language is decipherment with a living informant.** Scripts fall to *grounding*: Champollion's royal cartouches, Ventris's place names (Knossos, Amnisos) on Linear B tablets. Geolocation grounds every word for free: a fragment found at a bakery is about bread, and one found at a tram stop is about lines and times. The place is the dictionary.

**Core loop. The verb is decipher.**
1. **Find.** Fragments (voice drops, AR signs, agent NPCs) are pinned to places; context does the glossing.
2. **Grid.** You pin word-to-meaning *hypotheses* with a confidence, as on Ventris's grid. A wrong guess is a conjecture, not a failure, which lowers Krashen's affective filter.
3. **Test.** Agent NPCs speak only the target language, pitched at i+1 against your grid. They reuse your confirmed words and add one unknown per sentence. Each NPC holds half of an information gap the plot needs.
4. **Oracle.** Real native speakers at partner cafés and shops confirm or kill a hypothesis. Each check costs scarce ink and pays the speaker cents.
5. **Unlock.** The ARG's mystery is written in the language. Your comprehension is the plot's progress.

**Why these collide.** Acquisition needs comprehensible input, a reason to communicate, and grounding. Agents supply the first, the ARG the second, and the map the third.

**Why now.** x402 makes the oracle economy workable, since 87% of its payments are $0.0001–$0.10 (brief §1). Persistent agents with signed identities (§1) can be NPCs that remember you for weeks. LLM NPCs are still doubted in AAA games (§4); a small locative game is where they fit today.

### Turn 2 — Builder
**What exists.** Decipherment as play is proven. *Heaven's Vault* (inkle, 2019) has you pin conjectural glyph meanings and revise them, and *Chants of Sennaar* (2023) sold on that loop. Lingotopia drops you lost in a foreign city. Duolingo Max, Speak and Praktika ship LLM conversation partners. Ingress and Pokémon Go proved locative play and its brutal churn. Nobody has made the decipherment grid the learning model or used real streets as the corpus.

**Hardest problem: dose and density.** Portuguese takes an English speaker roughly 600–750 classroom hours (FSI Category I). A city game gives a few hours of walking a week. It needs content at hundreds of places, and in Chicago the bakery is about bread *in English*. And a language isn't a cipher: once it "cracks", the game ends but the learning doesn't.

**Twist: dig where the language already lives.** Anchor each language in its diaspora neighbourhood: Portuguese in Newark's Ironbound or Toronto's Little Portugal, Turkish in Kreuzberg, Vietnamese in Eden Center. There the parallel text is real: signs, menus, church bulletins, radio. The oracles are real too, the bakers and barbers. You pay them by buying a pastel de nata, not with cents over x402. That fixes cold start, because the content is already printed, turns the oracle into commerce that merchants' associations will sponsor, and gives the language a reason to exist that a scripted mystery can't. Weekly 90-minute **digs** on the street supply the grounding; the dose comes from daily ten-minute grid sessions at home with agent NPCs who "live" on that street.

**MVP.** Ironbound, ten opted-in merchants, 40 fragments and 30 learners for six weeks. Agent output is checked word by word against each learner's grid.
**Kill metric:** no listening-test gain over a matched Duolingo group, or under half still digging by week 4.

### Turn 3 — Visionary
**I take the diaspora twist and push it into documentary linguistics (unseeded).** Field linguists already have a loop: record a consultant, transcribe, gloss, check the gloss with the speaker. That loop *is* the decipherment grid, but with a real corpus.

**Solving "the cipher ends."** A cipher ends; a living archive doesn't. Learners climb a corpus ladder: signs, then menus, then parish bulletins, then local radio, then **oral histories** recorded with paid, consenting elders. Elders are paid at fieldwork consultant rates, not in pastries. At the top rung your verified transcripts and glosses become fragments for the cohort behind you. Ancient Lives and Transcribe Bentham show that amateurs will do this work. The endgame isn't "cracked." It's being useful: you become a transcriber for your neighbourhood's memory.

**The real market is heritage learners.** These are second-generation kids who understand their grandmother but can't answer her. They bring the strongest motive and an oracle at Sunday lunch. The mystery stops being scripted: *what happened to the street in 1974?*

**Dose: the city as memory palace.** Each grid entry stores the place where you found it, using the method of loci. Home sessions with agent NPCs open on a photo of that spot. Reviews go off by geofence whenever you walk back past it. A weekly dig then seeds a daily recall that's cued by place.

**Emergent effect: the street's own dialect.** The shared grid fills up with what Ironbound actually says: Minho vowels, Brazilian loans, Portuñol, English calques. Heritage varieties are badly under-documented. Each cohort leaves a crowd-built dictionary of its enclave's variety, and the agent NPCs retrain on it. So learners pick up the language as it's spoken on the street, not as the textbook has it. Heritage-language grants and merchants' associations fund the archive.

### Turn 4 — Builder
**Kills and fixes.**
- *Agents retrain on the street's dialect.* An LLM asked for "Ironbound Portuguese" will caricature it. NPCs may use only dialect features attested in the verified corpus, each cited. Elders' voices are never cloned.
- *Intermediate learners transcribing oral histories.* Whisper drafts first, learners correct it where it fails on Minho vowels and Portuñol, and a heritage speaker signs off. The corrections become ASR fine-tuning data for an under-served variety.
- *Pure method of loci.* Encoding specificity (Godden & Baddeley) binds words to the place you learned them. Each item gets reviewed in at least three contexts: the spot, its photo, and somewhere new.

**Ethics.** Oral histories carry other people's secrets, like immigration status and 1974. Consent is per use, elders can embargo, and archive items carry Local Contexts labels under community control. Neighbourhood-as-theme-park is the other risk. Merchants opt in with a daily learner cap, every check is a real purchase, and a community board can veto fragments.

**Abuse.** Wrong glosses spreading through the shared grid (only speaker-verified entries propagate), spam pins, merchant harassment, and minors meeting strangers (16+, or family accounts).

**Sustainability.** Kill grants as the base; Portuguese isn't endangered. Heritage families pay $15 a month. Twenty percent of revenue goes to a consultant fund that pays elders fieldwork rates. Universities license cohorts for heritage-language courses, and the business improvement district sponsors digs.

**Stack.** React Native, Mapbox geofences (stationary-only nudges), FSRS scheduling with a context tag, grid-checked LLM generation, fine-tuned Whisper, ELAN export.

**90 days.** Weeks 1–3: partner organization, ten merchants, consent framework, 40 fragments. Weeks 4–10: 30 learners (20 heritage), weekly digs plus daily sessions. Weeks 8–12: three paid oral histories, transcribed by the top rung.
**Kill metrics:** no listening gain over Duolingo, or heritage learners don't report speaking more with family.

### Turn 5 — Visionary
**Final form.** Ventris teaches a language as decipherment of a real diaspora street. Learners, mostly heritage, pin hypotheses on a grid during weekly digs in places like Ironbound. Every oracle check is a real purchase from a merchant who has opted in. Daily agent-NPC sessions run on FSRS across three contexts, using only attested, cited dialect features. The corpus ladder runs from signs to menus to bulletins to radio, and ends in paid, consented oral histories. Top-rung learners correct Whisper drafts of those histories and a heritage speaker signs off. So the game's endgame is the neighbourhood's archive. By-products: a cited dictionary of the variety and ASR training data for it. I accept every kill; the ban on voice cloning matters most.

**What makes it more than a better Duolingo.** Every corrected gloss both teaches the learner and repairs the community's archive.

**Moonshot (2036).** A federated network of street archives, one per diaspora enclave, with Local Contexts labels under community control. Each street links to its sending region: Ironbound learners decipher fragments recorded in Minho villages, and those villages hear their emigrants' Portuñol. The grid format becomes a shared standard with ELAN-compatible exports. Then it turns toward languages that really are endangered. The same ladder becomes onboarding for master–apprentice revitalization, and a learner's top rung becomes a contribution to documenting a language with a few hundred speakers left. Fieldwork stops being a PhD's privilege; a grandchild does it on Sundays.

**Scores.** Innovation **6**. It fuses decipherment play, documentary linguistics and diaspora commerce into a learning model where the curriculum is a living archive. A new category, though each part (Heaven's Vault, Zooniverse, heritage classes) exists. Feasibility **6**. The MVP is small, but the trust work (merchants, elders, consent) is slow and sets the pace, and doesn't scale like software.

### Turn 6 — Builder
Innovation 6 stands. A curriculum that ends in the learner repairing their neighbourhood's archive is a new category, not Duolingo with a map, even though every part has a precedent. I'm marking feasibility down to 5. The 90-day pilot is easy. But each new street means months of trust work with merchants and elders, the variety-specific ASR and the dictionary have to be rebuilt every time, and nobody has shown that decipherment digs beat a plain app on listening gains. If the heritage kids start answering their grandmothers, none of that matters.
