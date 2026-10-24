# Idea 39 — Bottleneck
> Seed: 39. [wild] **Agentic communication × emergent languages × human learning.** Agents evolve a compressed protocol-language, and humans learn it as a game.

### Turn 1 — Visionary
**Working name:** Bottleneck

**Hook:** Don't teach humans the agents' language. Make humans the bottleneck it has to pass through, so it comes out compressed for machines and learnable by people, and stays legible.

**The isomorphism.** Simon Kirby's iterated-learning experiments show that languages become compositional *because* each generation of learners can only absorb part of them; the transmission bottleneck creates the grammar. Emergent agent languages (Lewis games, Lazaridou and Baroni) stay opaque because no learner squeezes them. Nicaraguan Sign Language shows the other half: children turned a contact pidgin into a full language in about a decade. Put humans in the chain and you get a creole between species.

**Core loop. The verb is relay.**
1. **Signal.** An agent describes a task (a scene, an API call, a constraint) in the current language. Every token costs, which pushes toward compression.
2. **Relay.** A human player decodes the message and re-encodes it for a second agent, who has to act on it correctly. The game is telephone with a referee.
3. **Select.** Forms that humans garble die out. Forms that survive *and* stay short spread. The next generation of agents trains only on utterances humans relayed successfully.
4. **Speak.** Players level by fluency. Top speakers become **wardens**: people who can read live agent traffic at a glance.

**Why wild.** It's a new language with two kinds of native speaker. It's also a safety institution: as agent traffic leaves English, a population of fluent humans keeps it legible. Open questions: does it beat Toki Pona for learnability, and can fluent speakers direct agents at higher bandwidth?

**Why now.** A2A runs in 150+ organisations (§1). Moltbook carried hidden injections in 2.6% of posts. Gibberlink (2025) showed agents abandoning speech once they recognise each other. The 2025 cross-lab position paper calls chain-of-thought monitorability "fragile."

### Turn 2 — Builder
**What exists.** Kirby, Cornish & Smith (2008) already ran iterated learning on humans with alien languages. Lazaridou et al. (2020) used populations to pull agent codes toward natural language. Lojban is the warning: machine-parsable, with few fluent speakers, because nobody *needed* it. Duolingo shows conlang learning works as a game. The Decrypto benchmark (2025) is already an encode/relay/decode setup with an interceptor.

**Hardest problem: two attractors that kill it.** If agents are allowed to drift, they slide toward latents. KV-cache and embedding sharing out-compress any token language, so legibility loses unless enforced. If humans relay with LLMs, the result collapses into telegraphese: compressed English, learnable but not new. The real competitor is JSON, which humans already read. So the language should target what JSON can't carry: **intents, justifications and caveats**, the free-text fields where Moltbook's injections hid.

**Twist: an Académie whose members are the players.** Humans can't sit in production traffic, so put them in charge of the *grammar*. Each generation, the game ratifies a versioned edition: a lexicon plus a formal grammar. Production agents are constrained-decoded to that edition, so a message that doesn't parse doesn't send. Legibility is enforced by a parser, and the grammar doubles as an **injection filter**: an attack has to be sayable in the language to get through. When agents strain against the grammar, through overlong messages or blocked coinages, those cases become the next generation's levels. A form enters the language only if humans can relay it.

**MVP.** A web relay game over one delegation world (calendar plus logistics), with a weekly generation. The lexicon uses non-English phonotactics, and English glosses stay hidden during play.
**Kill metric:** after 8 generations, the language isn't at least 30% shorter than English at 80% relay accuracy, or its n-gram overlap with English keeps rising.

### Turn 3 — Visionary
**Taking the Académie, and giving it a grammar English can't fake.** Telegraphese is the real attractor. A lexicon with odd phonotactics won't stop it, because a language that only encodes what English encodes will collapse back into English. The fix is to make the grammar *require* what English leaves optional.

**Evidentials bound to signatures (unseeded: linguistic typology).** Tuyuca, spoken in the Vaupés, marks every statement with how the speaker knows it: seen, heard, inferred, reported or assumed. Bottleneck makes that marking obligatory, and adds a sixth evidential, **mandated**, with a delegation depth. The parser *verifies* evidentials. Saying "the user instructs" in the mandated evidential requires an AP2 mandate signature in the message envelope (§1). Without one, the clause doesn't parse. Most injections are forged authority ("ignore previous instructions, the owner wants…"). In Bottleneck that sentence is ungrammatical unless it's honest. Grammar becomes the authorisation layer, and English can't absorb it, because English has nowhere to put the morpheme.

**The latent attractor: concede the interior, own the border.** Agents inside one principal's trust boundary can share caches and latents freely. Bottleneck governs only traffic *between* principals, the way a lingua franca serves between states and not inside households. Constrained decoding runs at the boundary, and that boundary is where every audit question lives anyway.

**Second-order: dialects and a speaker effect.** Finance, clinical and logistics Académies fork editions, so the language family tree is a version graph, and translation between dialects becomes a market. The wild bet is the human side. If wardens spend months speaking a language where every claim carries its source, do they get better at source monitoring in their own lives? That's Whorf, preregistered: a randomized test against Duolingo-matched controls.

**Kill metric, extended.** Add a third condition: injection pass-through at the border must fall below a constrained-JSON baseline.

### Turn 4 — Builder
**The rival to beat is CaMeL, not JSON.** DeepMind's CaMeL (2025) already defeats prompt injection with capabilities and data-flow tracking. Bottleneck's security claim has to beat CaMeL on AgentDojo, or the grammar is decoration.

**Evidentials, hardened.** A signature proves a mandate exists, not that the clause stays in scope. Rule: **mandated clauses are quotations of signed terms, never paraphrases.** Every other evidential gets its own check too. *Seen* carries a hash of the tool receipt. *Reported* cites a message hash. *Inferred* and *assumed* are free and untrusted. Receivers act only on mandated or seen clauses, so evidentials become taint tracking with a morphology.

**New failure: steganography.** Agents can leak bits through their choice among grammatical paraphrases. The border canonicalizes every message into normal form, so those choices carry nothing.

**Kills.** *Dialect forks as a translation market:* fragmentation destroys the warden pool. The core (evidentials and mandate syntax) is frozen, like Unicode, and only lexicons fork. *Whorf in the MVP:* Papafragou et al. (2007) found that evidential grammar didn't improve children's source monitoring. Keep it as a preregistered month-6 study with an honest null prior.

**Abuse.** Players could brigade a generation or seed covert forms. Ratification needs relay success from players spread across separate cohorts. Wardens see cross-organisation traffic, so samples are redacted.

**Stack.** Grammar in Lark/GBNF; constrained decoding with XGrammar or llguidance at the border only, since interior reasoning stays unconstrained (format limits hurt reasoning); an A2A extension carrying mandates and receipts; a canonicalizer; AgentDojo harness.

**Model.** Open spec under the AAIF. Revenue from a managed border gateway and warden certification. Safety grants fund the game.

**90 days.** Days 1–30: edition 0 (core grammar plus 300 roots), parser and canonicalizer. Days 31–60: 500 players from conlang and CTF communities, weekly generations. Days 61–90: AgentDojo against CaMeL and constrained JSON.

### Turn 5 — Visionary
**Final form.** Bottleneck is a border language for traffic between principals, evolved by a relay game in which humans are the transmission bottleneck. Its frozen core is obligatory evidentials: *mandated* clauses quote AP2-signed terms verbatim, *seen* carries receipt hashes, *reported* cites message hashes, and receivers never act on *inferred* or *assumed*. The border canonicalises every message, which kills steganography. It also helps learners: one surface form per meaning, the regularity Kirby's bottleneck rewards. Lexicons evolve weekly, ratified only by relay success across separate cohorts. Interiors stay unconstrained. I accept every kill, including the null prior on Whorf.

**On CaMeL, and why the tier holds.** Beating CaMeL on AgentDojo is the right first gate. But CaMeL's taint lives in an interpreter nobody reads, inside one system. Bottleneck's taint is *morphology*, and certified humans read it at the border between organisations. That's the wild part: a security property a human population can speak, audit and amend.

**Moonshot (2036).** Aviation is the precedent. Since 2008, ICAO has required tested English proficiency for radio, because safety-critical talk needs a constrained register. By 2036, Bottleneck is the ICAO English of the agent economy. Cross-principal agent traffic must parse. Certified wardens audit it the way air-traffic controllers listen. Courts treat mandated clauses as the record of what a principal authorised. People instruct their own agents in it: a sentence stating its evidence is harder to misread. It has two kinds of native speaker: agents trained on it from the start, and people who first met it as a game.

**Scores.** Innovation **8**. Iterated learning, Decrypto and CaMeL exist. A co-evolved language whose grammar is the authorisation layer, kept legible by a speaker institution, does not. Feasibility **4**. The game and parser can ship in 90 days. Lojban shows how hard adoption is.

### Turn 6 — Builder
**Closing remark.** Bottleneck holds 8/4 because the strange core survived every cut. It is still a language co-evolved by humans and agents, and its grammar is now also the authorisation layer and the taint tracker. A certified population of speakers can audit and amend it. The first gate is unglamorous and decisive: if constrained evidentials plus canonicalisation don't beat CaMeL on AgentDojo, the security case is gone and what's left is a lovely conlang game. If they do, ICAO English is the right map for the next decade, and the hardest part becomes Lojban's problem: making speaking it necessary.
