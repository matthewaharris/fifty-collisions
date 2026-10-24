# Trends Brief: what's real in late 2025 to 2026 (as of 2026-10-23)

This brief is meant to ground a brainstorm on new apps, games, and systems. Confidence markers:
- **[solid]**: several outlets agree, or the claim comes from a primary source.
- **[soft]**: a single source, a vendor claim, or sources that disagree.
- **[stale?]**: may be older than 2026, or may have changed since.

Many of the sources are trade blogs and aggregators, so check any number before it goes into a pitch.

---

## 1. AI agents: protocols, autonomy, economies

**Trends and facts**
- **The protocol stack is settling into layers.** MCP is the agent-to-tool layer, A2A is the agent-to-agent layer, and IBM's ACP reportedly merged into A2A in 2025. The Linux Foundation formed the **Agentic AI Foundation (AAIF)** in Dec 2025 to govern these, with Anthropic, Google, OpenAI, Microsoft, AWS and Block among the founders. [solid on the AAIF; soft on the details]
- **A2A v1.0** reportedly ships with **cryptographically signed Agent Cards**, and more than 150 orgs reportedly run it in production. One blog says MCP's biggest revision (July 28, 2026) moved the protocol core from stateful to stateless. [soft: I couldn't verify this against the spec]
- **Agent payments have a stack too.** Google's **AP2** (Sept 2025) handles signed "mandates", meaning authorization. Coinbase's **x402** (HTTP 402 plus stablecoins) handles settlement per request. The two are joined through a Google and Coinbase extension. The **x402 Foundation** (Linux Foundation, July 2026, about 40 members including Visa, Mastercard, Stripe, Google, Circle and Solana) was reported but is unverified. Separately, OpenAI and Stripe launched the **Agentic Commerce Protocol** and Instant Checkout, and Visa launched its **Trusted Agent Protocol** with Cloudflare. [solid that these exist]
- **x402 volume: many payments, little money.** Chainalysis counts about 138.5M cumulative x402 transactions on Base by Q3 2026, up from about 59K in Q3 2025, and **87% of payments are between $0.0001 and $0.10**. Brian Armstrong cites more than 160M agentic payments a year. An a16z figure puts real dollar volume near only about $1.6M a month. [solid on the counts; the dollar value is disputed]
- **The first agent social network.** **Moltbook**, a Reddit for OpenClaw (ex-Clawdbot) agents, launched in late January 2026 and claimed 1.4 to 1.5M agents within a week. **Meta acquired it on March 10, 2026** for its registry of agents "verified and tethered to human owners". Researchers found that about 2.6% of sampled posts carried hidden prompt injections. [solid]
- **Autonomy is growing.** On METR's time-horizon metric (v1.1, Jan 2026, 31 tasks of 8+ hours), frontier models reportedly reach **roughly 12 to 15 hours at 50% success**, with very wide confidence intervals. METR itself warns that estimates above about 16 hours are unreliable. The doubling time is quoted at 4 to 7 months. [soft on the exact numbers]
- **Agentic checkout has had a setback.** OpenAI reportedly **deprioritized native Instant Checkout by March 2026** after a 4% fee, moving toward in-app purchases. The lesson is that agent-led shopping is harder than agent micropayments. [soft: from a vendor analysis]

**Unexploited gap:** Agent identity, reputation and credit scoring across protocols. A FICO-style score for agents, built on signed Agent Cards and x402 payment history, has no clear owner yet. Meta bought Moltbook for exactly the registry piece.

---

## 2. Finance: DeFi, stablecoins, prediction markets, tokenization

**Trends and facts**
- **Stablecoins are about $315 to 323B** (mid-2026 all-time high). USDT is about $190B (about 59%) and USDC about $77.6B. The **GENIUS Act** was signed in July 2025, but the final regulations are still being written. Forecasts range from $500B by the end of 2026 (an OCC estimate cited by Motley Fool) to $1.2T by 2028 (a Coinbase model). [solid on the range]
- **Prediction markets are now mainstream.** Combined Kalshi and Polymarket volume reached **$44.8B in June 2026** (up 75% month over month, driven by the World Cup), with Kalshi at $31.5B. Kalshi's best week was **$15.27B (Sept 14 to 20, 2026)**, and parlays are surging. Volume methods (notional vs. taker) differ a lot between sources. [solid on the direction]
- **A legal war between the CFTC and the states.** The CFTC has sued about 9 states (IL, AZ, CT, NY, RI, WI, MN, NM, KY) arguing that federal law preempts state gaming laws. Wisconsin is suing Kalshi, Polymarket, **Robinhood, Coinbase and Crypto.com**. A Massachusetts court subjected Kalshi's sports contracts to state gaming law. A **Supreme Court case is widely expected around 2027.** [solid]
- **Tokenized real-world assets (RWA), excluding stablecoins, are about $20 to 39B.** Tokenized treasuries are about $9 to 13B, with BlackRock BUIDL, Ondo and others. Tokenized stocks are still small at about $0.5B (CoinGecko). [soft: definitions vary]
- **Futarchy is being tested in the wild.** MetaDAO on Solana uses pass/fail conditional markets settled by a time-weighted average price (TWAP). It has helped raise more than $100M in token sales, but revenue fell from about $1.8M in Q4 2025 to about $0.56M in Q1 2026. Its pitch has shifted from "governing DAOs" to "investor protection and capital discipline for token launches." [soft]
- **Post-quantum pressure on crypto.** BIP-360 (P2MR) was merged as a *draft* in early 2026 and has not been activated. A EUROCRYPT 2026 paper cut the estimate to break 256-bit elliptic-curve cryptography (ECC) to about **1,098 logical qubits**. Google has an internal 2029 deadline to migrate to post-quantum cryptography. [soft on the qubit number]

**Unexploited gap:** Prediction markets for *personal or small-group* decisions, such as household, team or DAO choices, with stablecoin settlement. Today the big platforms only list public events, so the legal grey zone is mostly about sports. Futarchy at small scale is largely unbuilt.

---

## 3. Game theory and mechanism design

**Trends and facts**
- **Collusion among AI agents is an active research area.** "Mapping Human Anti-Collusion Mechanisms to Multi-agent AI Systems" was accepted at ICML 2026 and NeurIPS 2026. It maps leniency and whistleblowing, auditing, and market design onto LLM agent swarms. Open problems include working out which agent caused emergent coordination, and agents learning to evade detection. [solid]
- **Algorithmic pricing collusion is now a policy issue**, building on the Calvano et al. Q-learning work and the German gasoline study by Assad et al. More real agents are now bidding and pricing on behalf of users through x402 and ACP. [solid as background]
- **Futarchy and conditional markets** are the most visible new mechanism actually running (see MetaDAO). Prediction-market parlays show how much users want *composable* bets. [solid]
- **Mechanism design underpins the agent protocols.** AP2 "mandates" are delegation contracts, and x402 is per-request pricing. Classic results suggest that **posted take-it-or-leave-it prices are among the few formats that resist collusion** (Goldberg and Hartline), which may help explain why x402 uses posted prices rather than auctions. [my interpretation]

**Unexploited gap:** A "mechanism sandbox" where people design auctions, voting rules or markets, then watch LLM agents exploit or collude against them. It would be a teaching tool, a red-teaming tool and a game at once. Nothing consumer-facing exists.

---

## 4. Gaming: roguelites, deckbuilders, AI-native games, chess

**Trends and facts**
- **Slay the Spire 2** (Mega Crit) entered Early Access on **March 5, 2026** and peaked at **574,638 concurrent players** on Steam. That is the largest roguelike deckbuilder ever, more than Resident Evil Requiem's 344K and more than Hades II. It sold about **6.5M copies by mid-May**, about $108M in March alone. [solid]
- **The roguelite boom continues.** **Mewgenics** (Feb 2026) peaked at about 115K concurrent players. Balatro and Monster Train 2 are the reference games for the genre. Commentators note that "the roguelite cull hasn't happened": the genre keeps absorbing other mechanics. [solid]
- **AI-native games exist but are still niche.** Meshy launched **Black Box: Infinite Arsenal** at GDC 2026, where players text-prompt weapons and combat logic is generated in real time. Meshy reports $30M ARR and more than 10M users, mostly for its 3D tooling. Its developer was skeptical about LLM NPCs in AAA before 2027. OpenAI's Sora app (Sept 2025, all-AI video feed, 3.8M US downloads in month one) and Meta's Vibes (moving to a standalone app in 2026) are "AI-native UGC" more than games. [solid on the launches; soft on engagement]
- **Chess is going esports and freestyle.** FIDE and Freestyle Chess settled their dispute, and the **first official FIDE Freestyle (Chess960) World Championship** was held Feb 13 to 15, 2026 in Weissenhaus ($300K purse). The **Esports World Cup chess** event expanded to 21 to 22 players with a $1.5M pool (Aug 2026; sources disagree on the venue). [solid]
- **AI chess coaching is now a product category.** Chess.com has Play Coach with four persona coaches and adjustable help. An independent test found LLMs still make "elementary tactical mistakes" as coaches, so engine-grounded explanation is the bottleneck. [solid]

**Unexploited gap:** A deckbuilder or roguelite where the *opponents are real LLM agents with persistent memory and their own economies*, rather than a scripted AI. Also, a Chess960-native trainer that explains *principles* rather than memorized lines, which suits freestyle chess because openings can't be memorized.

---

## 5. Social networks: decentralized, AI companions

**Trends and facts**
- **Bluesky: the protocol grows while the app shrinks.** It has about 44 to 46M registered users, but monthly actives (Similarweb) were about **10.4M in June 2026, down 27% year over year**, with daily actives around 3M. Jay Graber stepped down as CEO in March 2026 and Toni Schneider is interim CEO. It raised a $100M Series B. **More than 1,000 AT Protocol apps are used weekly** (Skylight, BlackSky, Eurosky). [solid]
- **AI companions are a regulation story.** Character.AI **banned open-ended chat for under-18s (Nov 2025)** after lawsuits. California passed companion-safety rules (SB 243, Sept 2025). EU AI Act Article 50 transparency duties take effect in Aug 2026. The UK reportedly proposes mandatory breaks for minors. [solid on Character.AI and California; soft on the UK]
- **MIT Technology Review named AI companions a "10 Breakthrough Technologies 2026" entry** (Jan 2026). [solid]
- **Agent-populated social networks are a new category.** Moltbook (see section 1) put humans in the role of spectators while agents post. [solid]
- **All-synthetic feeds:** Sora app and Meta Vibes (see section 4). [solid]

**Unexploited gap:** Small, *friend-graph-sized* social apps on AT Protocol where each person's AI companion is a portable, inspectable account they own, rather than a walled-garden character. Companion data that can move between services, plus age safety built into the protocol, is largely unbuilt.

---

## 6. Quantum computing: real milestones and access

**Trends and facts**
- **Quantinuum Helios** (launched Nov 2025, 98 qubits, about 48 logical qubits) reported **99.9995% logical fidelity on Oct 19, 2026**. Quantinuum calls this a world record, 10x better than its own record from September 2026. Sandia reportedly called Quantinuum's fault-tolerant **Apollo (target 2029)** "feasible". The next system, Sol, is planned for 2027. [solid: press release from 4 days ago]
- **IBM Nighthawk** (120 qubits, 218 tunable couplers) and the **Loon** fault-tolerance test chip were delivered in Nov 2025. IBM targets **verified quantum advantage by the end of 2026** and runs an open **Quantum Advantage Tracker** with Algorithmiq, Flatiron and BlueQubit. IBM claimed verifiable advantage in Feb 2026 with UChicago, RIKEN and Qedma; independent consensus is still pending. [solid on the claim; contested on whether it holds]
- **Google Willow** (105 qubits) showed below-threshold surface-code error correction in Dec 2024. Google opened **Willow early-access proposals in 2026** and started a **neutral-atom team in Boulder (March 2026)**. [solid]
- **Neutral atoms are scaling.** QuEra and Pasqal reportedly have arrays of more than 10,000 atoms. Microsoft and Atom Computing's "Magne" (about 50 logical from about 1,200 physical qubits) is targeted for early 2027. [soft]
- **Accessible APIs:** IBM Quantum Platform (Qiskit, small free monthly allotment), AWS Braket, Azure Quantum, and Quantinuum Nexus. **[stale?]** Check the current free-tier terms.

**Unexploited gap:** Consumer and educational products that use *real* quantum hardware as a source of verifiable randomness or as a visible game mechanic, with provenance users can check. Also, tools that help small crypto projects and businesses plan post-quantum migration.

---

## 7. Consciousness science and simulation-theory discourse

**Trends and facts**
- The **COGITATE adversarial collaboration** pitted IIT against Global Neuronal Workspace theory. It was published in *Nature* (2025) with n=256 participants and fMRI, MEG and intracranial EEG. **Both theories took serious hits and neither won.** A follow-up collaboration, **INTREPID** (IIT vs. active inference vs. neurorepresentationalism), is pending. [solid]
- **AI welfare is now institutional.** Anthropic's model welfare program started in April 2025, with Claude models able to end abusive chats. An FT report (June 2026, seen via a secondary source) says DeepMind, Anthropic and Meta are all formally studying AI consciousness. Eleos warns that model self-reports are highly sensitive to what users seem to expect. [solid on Anthropic; soft on the FT details]
- **Simulation hypothesis in 2026:** A *Frontiers in Physics* commentary (2026) argues that astrophysical constraints make a simulation near-impossible. UBC Okanagan work (late 2025) uses Gödel-style arguments to claim physics is non-algorithmic. Melvin Vopson keeps pushing his "information as physical" and "infodynamics" ideas. Rizwan Virk reportedly cites about a 70% probability, tied to AI world models. [soft: mostly secondary sources]
- **The "world models" connection.** Generative world simulators (Genie-style models, Sora) have made "simulated worlds with agents inside" a concrete engineering object, so the discourse has moved from philosophy toward product. [my interpretation]

**Unexploited gap:** A game or experience where players run *preregistered experiments* on simulated agents, for example to test consciousness theories against each other, in the style of COGITATE but playable. It would be citizen science combined with philosophy. Nothing exists.

---

## 8. Meditation and contemplative tech

**Trends and facts**
- **Transcranial focused ultrasound (tFUS) for meditation.** The University of Arizona's SEMA Lab (Jay Sanguinetti) showed that modulating the default mode network (DMN) with tFUS improved mindfulness. It crowdfunded the **first tFUS meditation retreat**, and its hypothesis is that suppressing the basal ganglia deepens meditative states. This is research, not a product. [solid]
- **Jhourney** sells jhana retreats ("blissful meditation for everyone") with EEG feedback. Its 2026 status is unverified. [soft]
- **Incumbents are moving into AI mental health.** Headspace has its **Ebb AI companion** plus therapists. Calm focuses on broad content. Waking Up is the "philosophy of consciousness" niche. I found no big 2026 AI launches from any of them. [stale?]
- **Neurofeedback headbands** (Muse and similar) plus AI personalization are being discussed, as in the FitMind podcast of March 2026, but there is no breakout product. [soft]

**Unexploited gap:** *Objective, verifiable* progress tracking for contemplative states. Wearable EEG or HRV, plus an LLM teacher, plus jhana-style protocols could act as a "Strava for meditation" where streaks reflect measured state rather than minutes logged. Incumbents are still built around content libraries.

---

## 9. Health, biohacking, wearables, longevity

**Trends and facts**
- **Wearable regulation loosened.** The FDA issued updated general-wellness and clinical decision support guidance on **Jan 6, 2026**. The FDA then **closed out its warning to Whoop** over Blood Pressure Insights (June 2026). Whoop raised **$575M at a $10B valuation**. The Apple Watch **hypertension notification** feature was FDA-cleared in Sept 2025 for Series 9 and later. [solid]
- **GLP-1s in pill form.** **Lilly's orforglipron (Foundayo)** was FDA-approved on **April 1, 2026**. It is once daily with no fasting needed, costs **$149 to 349 a month in cash**, and starts at $25 a month with insurance. [solid]
- **Longevity reprogramming moves into the clinic.** **NewLimit raised $435M at $3.1B (June 2026)** for mRNA delivered by lipid nanoparticles (LNP) carrying fewer than 10 transcription factors to the liver, with human trials planned for 2027. **Life Biosciences** has Phase 1 approval for partial reprogramming in optic neuropathy (Jan 2026). Retro Biosciences (backed by Sam Altman) reportedly began its first trial in Dec 2025. [solid on NewLimit; soft on Retro]
- **Continuous glucose monitors (CGMs)** are an approximately $15B market. Over-the-counter devices (Dexcom Stelo, Abbott Lingo) launched in 2024 and 2025. [stale? for 2026 specifics]

**Unexploited gap:** Under the looser wellness guidance, a *personal n-of-1 trial platform*. You randomize your own interventions (GLP-1 dose timing, sleep, supplements) using CGM, HRV and blood-pressure wearables, with proper statistics. Most apps only show dashboards and never run experiments.

---

## 10. Healthcare AI

**Trends and facts**
- **Autonomous AI prescribing.** In Utah's regulatory sandbox (launched Jan 2026), **Doctronic's AI renews about 190 chronic-condition drugs** without a physician sign-off. Physicians review the first 250 patients, and controlled substances are excluded. The FDA says it has authorized no AI chatbots, and legal scholars debate the misbranding question. [solid]
- **Ambient scribes are everywhere and unregulated.** Abridge is UPMC's primary scribe tool, and roughly 70% of UCSF physicians use one daily. The AMA's 2026 survey reports that **81% of physicians use AI**. Scribes are classified as administrative tools and sit outside FDA oversight. They are now creeping into drafting orders and real-time ICD-10 and HCC coding. [solid on the trend; soft on the figures]
- **Prior authorization agents.** PrescriberPoint launched an end-to-end autonomous prior-auth agent on April 21, 2026. 93% of physicians report care delays from prior authorization. [solid]
- **FDA AI device count:** about 1,350 to 1,430 AI-enabled devices. It is disputed whether any *generative* AI device has been cleared. [soft]

**Unexploited gap:** A *patient-side* agent that fights the payer's agent: appeals, prior-auth tracking, bill negotiation. Most investment goes to the provider and payer side, and agent-vs-agent negotiation is coming whether or not anyone designs for it.

---

## 11. EVs, batteries, V2G

**Trends and facts**
- **Sodium-ion goes industrial.** CATL **Naxtra** is about 175 Wh/kg, with first sodium-ion storage systems delivered in **Sept 2026** and a **60 GWh deal with HyperStrong**, the largest so far. BYD is targeting about 50 GWh a year. Sodium-ion material costs have reportedly halved. [solid]
- **Megawatt charging.** CATL's Qilin third generation is about 280 Wh/kg at 10C, around 3 MW peak. CATL's Shenxing third generation charges from 10 to 98% in about 6.5 minutes. [solid]
- **V2G is going mainstream in the US.** In **April 2026**, PG&E approved the **Cybertruck plus Powershare Gateway** for its residential vehicle-to-everything (V2X) pilot, the first AC-based V2G system in California. It offers up to $4,500 in incentives, with 1,000 spots. Ford and GM use DC setups. Tesla Powershare Grid Support launched first in Texas (CenterPoint, Oncor). [solid]
- **Europe:** Octopus Power Pack (a UK V2G tariff with free charging under conditions) and Mobilize/The Mobility House V2G with the Renault 5 in France, Germany and the UK. [stale?: launched 2024 to 2025]
- **Lithium has rebounded** to about $24 to 25K per tonne (May 2026), driven by demand for grid storage. [soft]

**Unexploited gap:** V2G participation designed as a *game or co-op*. Neighborhoods could pool cars into a virtual power plant with transparent payouts, possibly in stablecoins, with leaderboards. Today it is utility-run pilots with no consumer layer.

---

## 12. Mining: crypto and physical/critical minerals

**Trends and facts**
- **Bitcoin miners have become AI landlords.** Hashprice fell to about **$28 to 30 per PH per day** (Q1 2026), and the average cash cost to mine about **$80K per BTC** was above the price in Q4 2025. Public miners hold **more than $70B in AI and HPC contracts**: Core Scientific with CoreWeave for $10.2B over 12 years, TeraWulf with about $12.8B. CoinShares projects up to 70% of miner revenue from AI by the end of 2026. [solid]
- **Rare earth geopolitics.** China put **MP Materials and USA Rare Earth on its export-control list (June 22, 2026)**, 14 EU firms on July 24, and a second wave takes effect **Nov 10, 2026**. China holds about 91% of global rare-earth refining. The G7 targets less than 60% dependence on any single supplier by 2030. The Pentagon holds an equity stake in MP Materials. [solid]
- **Deep-sea mining.** The Metals Company (TMC) USA filed the first consolidated NOAA application (65,000 km² in the Clarion-Clipperton Zone, Jan 2026). It was published in the Federal Register on Aug 19, 2026, and TMC expects a commercial permit by Q1 2027. [solid on the process; the timeline is the company's]
- **Copper at record highs**, about $13K to 14.5K per tonne on the LME (2026), with AI data centers a cited driver. JPMorgan cites about 475Kt a year of data-center demand and about a 330Kt deficit. The Grasberg mine won't return to full output until the end of 2027. [soft on the exact levels]

**Unexploited gap:** *Provenance and traceability games or markets* for minerals, such as battery passports (the EU battery passport becomes mandatory in 2027) combined with tokenized offtakes. Also, tools that let miners sell flexible load to both bitcoin mining and AI inference, switching in real time on price.

---

## 13. Construction tech and robotics

**Trends and facts**
- **Autonomous heavy equipment through retrofit.** **Bedrock Robotics** (ex-Waymo team) raised a **$270M Series B at $1.75B (Feb 2026)**. Its kit installs in one shift on 20 to 80 ton excavators. It ran supervised autonomy on a 130-acre site (Nov 2025) and targets **operator-less work in 2026**. [solid]
- **Bricklaying robots.** FBR's Hadrian X (about 1,000 bricks an hour) built demo homes with CRH. Construction Robotics' SAM. [stale?]
- **Humanoids are in factories, not on job sites.** **Figure 03** has reportedly been deployed at BMW Spartanburg at about **$25 per robot-hour** (Jan 2026). **1X NEO** is available for home pre-order at $20K or $499 a month, with US deliveries promised in 2026. Tesla Optimus production claims conflict. I found no real construction-site humanoid deployments. [soft]
- **Site intelligence:** Boston Dynamics Spot is used for patrols and scanning (Skanska, Hensel Phelps). [solid]

**Unexploited gap:** Billing autonomous equipment by the hour, as with Figure's $25 per hour, makes "robot labor marketplaces" possible. A small contractor could rent an autonomous excavator *shift* through an app the way they would book a crew. Nobody runs that brokerage yet.

---

## 14. Agriculture and farm tech

**Trends and facts**
- **Retrofit autonomy plus lasers.** Carbon Robotics has the **LaserWeeder G2** and the **Carbon ATK** autonomy kit for John Deere 6R, 8R and 8RX tractors, and announced a "Carbon Autonomy Ready" program at the Farm Progress Show 2026 with Great Plains as the first partner. It also **cut staff to 275** citing the economy. [solid]
- **Deere launched "JD", a generative AI assistant, in Operations Center (Sept 1, 2026).** McKinsey's 2026 farmer survey: **17% of farmers use generative AI, but only 6% trust it, vs. 56% who trust human agronomists.** [solid]
- **Drone shock.** The FCC added foreign drones and components to its Covered List (Dec 2025). DJI held about **80% of US spray drones**, and US sales fell from 8,950 units in 2024 to 3,711 in 2025. [solid]
- **The vertical-farming collapse continues.** **80 Acres Farms shut down (Aug 2026)** and **Infinite Acres went bankrupt (Sept 8, 2026)**. 14 controlled-environment agriculture bankruptcies in 2025, and fewer than 10 of the 23 signatories of the vertical-farming manifesto survive. The causes were energy costs and financing. [solid]
- **Enhanced rock weathering (ERW)** is real carbon removal on farmland. Climeworks and Lithos Carbon have an offtake deal (Mar 2026), Frontier approved Puro.earth's ERW methodology (June 22, 2026), and credits sell for roughly €200 to 500 per tonne. [solid, except the price, which is soft]

**Unexploited gap:** *Trust infrastructure* for farm AI, for example AI advice co-signed by a human agronomist with a track record and outcomes the farmer can verify. Also, ERW credits as a revenue line for small farmers, with automated measurement, reporting and verification (MRV).

---

## 15. Other surprising areas

- **Space:** Starship has **still not demonstrated ship-to-ship propellant transfer** after more than 13 flights. SpaceX bought **EchoStar spectrum for $17B**. Next-generation Starlink direct-to-cell satellites launch on Starship from mid-2027, with first phone tests in late 2026. [solid on the spectrum deal; flight numbering in the sources is unreliable]
- **Neurotech:** **Neuralink demoed speech restoration (March 2026)**: an ALS patient's speech decoded from phonemes into his own voice, with about 12 to 21 trial participants. **Apple's BCI Human Interface Device (HID) protocol** makes brain signals a native input, and Synchron was the first to use it on iPhone, iPad and Vision Pro. [solid on the speech demo; soft on the counts]
- **Fusion:** Helion's Polaris reached **150 million °C (Feb 2026)** with deuterium-tritium (D-T) fuel. Its Orion plant (50 MW) is under construction in Malaga, Washington, targeting Microsoft by 2028. Net electricity has not been demonstrated. [solid]
- **AI scientists:** **Periodic Labs** ($300M seed, a16z) builds closed-loop AI and robotics labs for materials and superconductors. *Nature* says self-driving labs are "still in infancy, mostly incremental." [solid]
- **Geoengineering:** **Stardust** has raised $75M and published self-written ethics rules (April 2026), and claims it can cool the planet for $10B a year (CNN, Sept 2026). Make Sunsets runs unauthorized SO₂ balloon releases, and Mexico banned solar geoengineering after one. [solid]
- **Home robots:** 1X NEO at $20K or $499 a month. Samsung's Ballie slipped again. [soft]

---

## 16. Cross-domain connections

1. **Bitcoin miners, AI compute and V2G are all the same business: flexible load.** Miners sell interruptible power to AI. EVs sell interruptible storage to the grid. The missing piece is a *single flexible-load marketplace* where data centers, mining rigs and car batteries all bid. x402-style micropayments could settle it per kWh-minute.
2. **Futarchy plus healthcare AI.** Utah's sandbox approves AI prescribing case by case. Conditional prediction markets ("if the sandbox expands, will adverse events rise?") could inform regulators. Prediction markets on policy *outcomes*, not elections.
3. **Slay the Spire 2's scale plus the agent economy.** A deckbuilder in which every card is an *MCP tool* and every enemy is an A2A agent with a wallet. Roguelite runs would double as stress tests of agent protocols, with players red-teaming agents for fun.
4. **COGITATE-style adversarial collaboration plus Moltbook.** Agent social networks are natural testbeds for theories of machine consciousness and welfare. Preregister predictions, then watch 1M agents. AI welfare research needs exactly this kind of data.
5. **Post-quantum migration plus Quantinuum's logical-fidelity records.** The estimate for breaking ECC dropped to about 1,098 logical qubits while Helios has about 48 logical qubits at 99.9995% fidelity. A "Q-Day countdown" prediction market tied to verified tracker milestones would be a real hedging tool for crypto holders.
6. **Rare earth export controls plus construction and farm retrofit autonomy.** The Bedrock and Carbon kits need motors and magnets that China can now restrict (the Nov 10 wave). Choosing magnet-light actuators could be a strategic product decision, not just an engineering one.
7. **ERW plus mining tailings plus farms.** Crushed silicate from mine waste spread on fields earns €200 to 500 per tonne in credits. Physical mining, agriculture and carbon finance meet here, and small farms lack an MRV app.
8. **Meditation tFUS plus BCI HID.** Apple's BCI HID turns brain signals into a standard input. Once consumer EEG and tFUS headsets speak HID, "meditation state" becomes an input for apps, for example games that unlock when the default mode network quiets down.
9. **The GLP-1 pill plus CGMs plus the relaxed wellness rules.** A $149-a-month oral GLP-1 combined with an over-the-counter CGM makes metabolic n-of-1 experiments mass-market. Dose-timing optimization apps can operate in the loosened wellness space.
10. **The Freestyle Chess960 boom plus the weakness of LLM coaches.** Freestyle chess removes opening memorization, and engine-grounded *principle explanation* is exactly what LLM coaches are missing. A coach that explains *why* in random starting positions fits both trends.
11. **The prediction-market sports war plus game design.** Kalshi parlays are surging, so users want composable bets. Roguelite "build" logic (stacking modifiers) is the same mental model. A game where your deck is a portfolio of real event contracts sits right at the edge of the CFTC-vs-states fight.
12. **Vertical-farming failure plus AI-data-center waste heat.** Vertical farms died of energy costs, while data centers and converted mining sites have surplus heat and power. Co-locating greenhouses with AI and mining sites, with heat as the fuel, is an old idea made newly feasible.
13. **Bluesky's protocol-not-app strategy plus AI companion regulation.** Companion apps face age and safety rules. AT Protocol's composable moderation and labelers could deliver "regulated companion as a protocol feature" across many apps, so the compliance layer becomes the product.
14. **Figure's $25 per robot-hour plus stablecoin micropayments.** Robot labor billed per hour, or per task, is a natural fit for x402 and AP2 mandates: a robot paid by an agent, which was authorized by a human mandate. Physical-world agent commerce.
15. **Simulation-hypothesis discourse plus world models plus quantum randomness.** "Is this world simulated?" becomes a playable question in games built on generative world models, where provably non-pseudorandom quantum entropy is the in-game "glitch" or proof of being outside the simulation.

---

## Sources

**AI agents and protocols**
- https://kodekloud.com/blog/a2a-vs-mcp-agent-communication-protocols-explained-for-devops/
- https://zuplo.com/blog/agent-protocol-stack-mcp-a2a-acp-2026
- https://blindpay.com/resources/more/agent-payment-protocols-compared.md
- https://www.cobo.com/post/ap2-protocol-complete-guide-to-agent-payments-for-web3-developers-2026
- https://cointelegraph.com/news/agentic-payment-activity-surpasses-100m-transactions-on-base
- https://thedefiant.io/brian-armstrong-says-coinbase-processes-usd1t-in-stablecoin-payments-annually
- https://regional-front.cointelegraph.com/news/ai-agent-payment-volume-closer-to-1-6m-says-a16z
- https://www.digitalcommerce360.com/2026/02/16/openai-expands-agentic-commerce-push/
- https://nuvei.com/posts/openai-tried-to-checkout-it-couldnt
- https://www.techradar.com/pro/everything-you-need-to-know-about-moltbook
- https://www.digitalocean.com/resources/articles/what-is-moltbook
- https://en.wikipedia.org/wiki/METR
- https://www.datalearner.com/en/benchmarks/metr-time-horizons-v1-1

**Finance**
- https://www.theblock.co/post/406983/kalshi-polymarket-volume-45-billion
- https://defirate.com/?p=21173
- https://frontofficesports.com/article/trump-administration-sues-three-states-prediction-markets/
- https://www.crowdfundinsider.com/2026/06/285497-cftc-files-lawsuit-against-new-mexico-to-defend-federal-oversight-of-prediction-markets/
- https://www.chicagofed.org/-/media/others/people/documents/decarlo-stablecoins-under-genius-act.pdf
- https://www.fool.com/research/banks-payment-processors-stablecoins/
- https://www.ziromarket.com/blog/stablecoin-market-cap-323-billion-2026-genius-act
- https://data.coingecko.com/learn/what-are-real-world-assets-exploring-rwa-protocols
- https://blockworks.co/news/metadao-solana-governance-platform
- https://defillama.com/protocol/token-rights/metadao
- https://cointelegraph.com/features/bitcoin-s-quantum-upgrade-path-what-bip-360-changes-and-what-it-does-not

**Mechanism design**
- https://icml.cc/virtual/2026/78586
- https://arxiv.org/pdf/2511.12456
- https://www.microsoft.com/en-us/research/?p=144948

**Gaming and chess**
- https://respawn.outlookindia.com/gaming/gaming-news/slay-the-spire-2-hits-574k-players-crushing-aaa-games-on-steam
- https://www.gamingonlinux.com/2026/03/slay-the-spire-2-becomes-the-biggest-roguelike-deck-builder-on-steam-ever/page=1/
- https://recognizingpatterns.substack.com/p/the-roguelite-cull-hasnt-happened
- https://finder.techleap.nl/news/feed/meshy-launches-ai-native-game-and-hits-30m-arr-at-gdc-2026
- https://www.pcgames.de/Hardware-Thema-130320/Specials/KI-als-Chance-fuers-Gaming-diese-Spiele-zeigen-was-alles-moeglich-ist-1467192/5
- https://en.chessbase.com/post/the-chess-world-in-2026
- https://esportsworldcup.com/en/news/ewc26-chess-survival-guide
- https://www.chess.com/news/view/announcing-play-coach
- https://www.chess.com/blog/damafe/i-tested-chatgpt-as-a-chess-coach-does-it-actually-work

**Social**
- https://tech.slashdot.org/story/26/08/16/0326249/
- https://www.technologyreview.com/2026/01/12/1130018/
- https://decrypt.co/346770/character-ai-halts-teen-chats-after-tragedies-its-the-right-thing-to-do
- https://techcrunch.com/2025/09/30/openai-is-launching-the-sora-app-its-own-tiktok-competitor-alongside-the-sora-2-model
- https://www.pymnts.com/meta/2026/meta-challenges-sora-with-standalone-vibes-app/

**Quantum**
- https://www.quantinuum.com/press-releases/quantinuum-breaks-error-correction-world-record-by-10x-validating-its-leading-path-to-fault-tolerant-quantum-computing-by-2029
- https://www.hpcwire.com/2026/10/19/quantinuum-says-helios-results-strengthen-its-path-to-apollo/
- https://www.techtimes.com/articles/329227/20261020/quantinuum-achieves-factor-12-quantum-error-suppression-helios-sandia-calls-apollo-feasible.htm
- https://andesqubit.com/en/blog/quantum-hardware-roadmaps-2026-2030/
- https://www.nextplatform.com/2025/08/05/ibm-outlines-steps-to-verify-claims-of-quantum-advantage/
- https://spectrum.ieee.org/neutral-atom-quantum-computing
- https://quantumcomputingreport.com/news-archive-2026/

**Consciousness and simulation**
- https://pubmed.ncbi.nlm.nih.gov/40307561/
- https://nautil.us/inside-the-big-bet-on-consciousness-1207750/
- https://www.anthropic.com/research/exploring-model-welfare
- https://www.lesswrong.com/posts/KGtLswHGhsdJtyi8K/digital-minds-in-2025-a-year-in-review
- https://techjacksolutions.com/ai-brief/deepmind-anthropic-and-meta-are-formally-studying-whether-ai/
- https://public-pages-files-2025.frontiersin.org/journals/physics/articles/10.3389/fphy.2026.1808725/text
- https://metavert.io/compare/simulation-hypothesis-vs-the-matrix

**Meditation**
- https://news.arizona.edu/news/researchers-explore-use-ultrasound-achieve-mindfulness
- https://crowdfund.arizona.edu/project/42862
- https://gwern.net/doc/www/www.jhourney.io/73f27194be0295770bc7dc4163e03e2c89cbd5ef.html
- https://asomobile.net/en/blog/best-meditation-mental-health-apps-in-2026-a-practical-review-of-the-top-options/

**Health and longevity**
- https://www.mddionline.com/wearable-medical-devices/fda-backs-down-on-whoop-blood-pressure-monitoring-dispute
- https://www.statnews.com/2026/05/21/whoop-has-not-resolved-fda-dispute-over-blood-pressure-feature/
- https://www.mddionline.com/wearable-medical-devices/whoop-secures-575m-series-g-funding-at-10b-valuation
- https://www.mobihealthnews.com/news/apple-receives-fda-nod-hypertension-notification-feature
- https://www.smdailyjournal.com/news/national/fda-grants-speedy-approval-to-eli-lillys-weight-loss-pill-for-obesity/article_e01b2b0a-a89e-5f12-b2dd-a12ca974cd65.html
- https://athletechnews.com/newlimit-raises-435-million-age-reversing-medicine-human-trials/
- https://research.contrary.com/report/retro-biosciences

**Healthcare AI**
- https://law.stanford.edu/2026/03/19/utahs-experiment-with-ai-driven-prescription-renewals
- https://www.mobihealthnews.com/news/utah-allows-ai-renew-prescription-drugs-autonomously
- https://www.2minutemedicine.com/2mm-ai-roundup-prescriberpoint-launches-autonomous-agent-for-prior-authorization-utah-begins-first-in-nation-autonomous-ai-prescribing-pilot-and-jama-study-confirms-ambient-ai-scribes-ret/
- https://ai2.work/blog/ambient-ai-scribes-escape-fda-oversight-as-clinical-roles-grow
- https://enterprises.upmc.com/resources/insights/health-care-innovation-in-2026-from-ai-buzz-to-measurable-impact/

**EVs, batteries, V2G**
- https://carnewschina.com/2026/06/05/catl-to-deliver-first-sodium-ion-storage-systems-in-september-as-material-costs-halve/
- https://www.batterytechonline.com/design-manufacturing/how-catl-moved-sodium-ion-batteries-from-lab-chemistry-to-gwh-scale-reality
- https://chemxplore.com/news/catl-unveils-six-battery-innovations
- https://electrek.co/2026/04/20/pge-tesla-cybertruck-first-ac-vehicle-to-grid-california/
- https://www.electrive.com/2026/04/21/pge-and-tesla-integrate-cybertruck-into-v2g-programme/
- https://greenfleet.net/node/13711
- https://spglobal.com/market-intelligence/en/news-insights/research/2026/05/sqm-seen-posting-58percent-revenue-growth-in-2026-as-lithium-prices-rebound

**Mining**
- https://www.theblock.co/amp/post/395259/bitcoin-miners-face-breakeven-pressure-as-ai-pivot-accelerates-coinshares-says
- https://cointelegraph.com/news/bitcoin-mining-outlook-2026-ai-profitability-consolidation
- https://www.miningweekly.com/article/china-places-two-us-rare-earths-producers-on-export-control-list-2026-06-22
- https://www.spglobal.com/energy/en/news-research/latest-news/metals/072426-china-bans-rare-earth-exports-to-14-european-companies
- https://source.benchmarkminerals.com/article/china-s-rare-earth-export-controls-show-little-sign-of-easing-after-trump-xi-summit
- https://www.nasdaq.com/press-release/tmc-usa-files-first-consolidated-deep-seabed-mining-application-increasing-expected
- https://www.edisongroup.com/equity/the-metals-company/
- https://finance.yahoo.com/news/copper-prices-hit-record-high-082959021.html

**Construction and robotics**
- https://www.therobotreport.com/bedrock-robotics-270m-series-b-paves-way-operator-less-excavators/
- https://sacra.com/research/bedrock-robotics
- https://www.therobotreport.com/rbr50-company-2025/hadrian-x-bricklaying-robot-builds-first-homes-in-demo-program/
- https://the-gadgeteer.com/2026/05/20/home-robots-available-2026/
- https://lumichats.com/blog/humanoid-robots-2026-tesla-optimus-figure-ai-unitree-complete-guide

**Agriculture and climate**
- https://geekwire.com/2026/seattle-ag-tech-startup-carbon-robotics-reduces-workforce-citing-economic-conditions/
- https://forklog.com/en/mckinsey-reports-record-adoption-of-ai-in-agriculture-john-deere-launches-ai-agronomist/
- https://www.rfdtv.com/farmers-adopt-ai-while-still-trusting-agronomists-most
- https://www.farmprogress.com/technology/farmers-brace-for-shortages-after-fcc-drone-ban
- https://www.fertilizerdaily.com/20260610-dji-users-urge-fcc-to-rethink-foreign-drone-ban-as-ag-operators-warn-on-alternatives/
- https://www.hortidaily.com/article/9872227/infinite-acres-declared-bankrupt-in-the-netherlands/
- https://irei.com/publications/article/vertical-farms-promised-a-food-revolution-but-a-decade-later-many-investors-have-starved/
- https://decarbonfuse.com/posts/climeworks-and-lithos-carbon-partner-to-scale-verified-erw-credits
- https://mondovisione.com/media-and-resources/news/frontier-approves-puroearths-enhanced-rock-weathering-methodology-follows-fr-2026622/
- https://keyt.com/news/national-world/cnn-other/2026/09/08/this-company-says-it-can-help-cool-the-planet-for-10-billion-a-year-scientists-are-raising-red-flags/
- https://www.cleanthinking.de/stardust-solutions-geoengineering-system/

**Space, neuro, fusion, AI science**
- https://driveteslacanada.ca/news/spacex-reveals-starlink-spectrum-plans-following-17-billion-deal-first-phone-trials-in-late-2026/
- https://www.jamaicaobserver.com/2026/03/02/starlink-launch-direct-cell-satellites-2027/
- https://buttondown.com/patricknovak1/archive/spacex-ep070-starship-still-lacks-an-orbital-refuelin/
- https://3zebras.com/science/brain-computer-interface-2026-neuralink-synchron-update/15397/
- https://www.medicaleconomics.com/view/synchron-first-to-integrate-brain-computer-interface-with-apple-devices
- https://fortune.com/2026/02/13/sam-altman-fusion-helion-energy-milestone-doubters-grid-power-2028/
- https://www.maginative.com/article/periodic-labs-launches-with-300m-to-build-an-ai-scientist/
- https://media.nature.com/original/magazine-assets/d41586-026-00974-2/d41586-026-00974-2.pdf
