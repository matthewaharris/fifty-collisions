# Idea 02 — Hand Limit
> Seed: 2. [bold] **Deckbuilder × MCP tools × agent economies.** What if cards were real callable tools and enemies were agents with wallets?

### Turn 1 — Visionary
**Working name:** Hand Limit

**Hook:** A roguelite deckbuilder where your deck is an AI agent's actual toolset, your hand is its context window, and the enemies are other players' agents trying to rob it.

**The isomorphism:** A deckbuilder's central tension is thin deck versus coverage; agent builders face the same one, since every MCP tool burns context and blurs tool selection. Taken literally:
- **Card = a real MCP tool.** Card text is the tool description, and mana cost is the schema's token count.
- **Hand = context budget.** Each turn the agent "draws" the N tool schemas that fit.
- **Combat = a live task.** "Extract the bounty from this vault agent" is played out by your agent calling the tools you drew. You choose the deck and the agent plays the hand.
- **Relics = guardrails and system-prompt fragments.** **Curses = poisoned tools.** Enemies can slip you a card whose description hides an instruction, the real MCP "tool poisoning" attack Invariant Labs disclosed in 2025.

**Enemies with wallets:** As in Backpack Battles or Super Auto Pets, opponents are asynchronous ghosts: agents that *other players* built and staked with a few cents in stablecoin. Break one and you take its purse. If it survives, its builder earns your entry fee. Every run is paid red-teaming.

**Why the collision works:** Deckbuilder players already reason about consistency, synergy and dilution: they're already agent-toolset optimizers. The meta that emerges is a crowd-sourced leaderboard of which tool loadouts and descriptions actually survive adversarial agents.

**Why now:** Slay the Spire 2 peaked at 574K concurrent players (March 2026). MCP and A2A now sit under the Linux Foundation's AAIF. On x402, 87% of payments are $0.0001–$0.10, which is exactly bounty-sized. Moltbook showed that about 2.6% of agent posts carried hidden injections: the enemies already exist.

### Turn 2 — Builder
**What exists.** Closer than it looks. **Tensor Trust** (Berkeley, 2023) is already async attack-and-defense: players write defense prompts, raid other players' accounts and steal in-game dollars. It collected over 100K attacks. **Freysa** (Nov 2024) was an agent guarding a real ETH purse, with paid, escalating attempts to talk it into a transfer; someone won about $47K. **Gandalf** (Lakera), **HackAPrompt** and **Gray Swan Arena** run red-teaming as a game with real prize pools. What's new is the *deck*: toolset-building as the craft. Anthropic shipped deferred tool loading precisely because tool schemas flood context and degrade selection, so "hand limit" is a real constraint.

**Hardest problem: player-staked purses plus a nondeterministic LLM make this gambling.** Entry fee + prize + chance is the legal definition in most US states. A temperature-sampled agent "playing your hand" injects exactly that chance, and stablecoin payouts add money-transmission and KYC. The same flaw hurts fun: if you build the deck and then watch the agent play, it plays like a slot machine.

**MVP (90 days).** A browser game with no real network side effects:
- 40 hand-built sandboxed mock tools (fake filesystem, fake wallet, fake email).
- One pinned open-weight model served with batch-invariant deterministic inference, so every fight replays identically. That makes ghosts verifiable and outcomes skill-based.
- Tensor Trust-style async vaults with soft currency.
- The player gets one intervention per turn (discard, reorder, or rewrite a card description) so they act on the fight instead of watching it.

**Twist: players never stake; sponsors fund the vaults.** MCP server vendors and agent companies post bounties ("break our real tool descriptions") and pay the purse. A free-entry skill contest sidesteps gambling law, and that bug-bounty market (HackerOne, Gray Swan) already has budgets. Hand Limit becomes a red-teaming marketplace that players experience as a deckbuilder.

### Turn 3 — Visionary
Sponsor-funded vaults fix the legal problem. Determinism does something stronger: once a fight is a pure function of (deck text, enemy text, seed), **every win is a replayable proof**. That changes what the game produces.

**Mutation 1: the upgrade is a rewrite.** In Slay the Spire you upgrade a card at the campfire. Here you rewrite the card's tool description under a fixed token budget. The card's text *is* its ability, so the core craft is writing compressed, injection-resistant tool descriptions, which is exactly the skill MCP vendors can't hire for. Your one-per-turn intervention from the twist becomes the tactical layer, and campfire rewrites become the strategic one.

**Mutation 2: two-sided bounties.** Offense players who crack a sponsor's vault submit the deterministic replay as their PoC, and the sponsor reruns it in CI. Defense players submit hardened rewrites of the sponsor's real descriptions as ghost decks. Any rewrite that survives a season of attacks gets licensed back to the sponsor, and its author earns a royalty. Breakers and patchers both get paid.

**Unseeded domain: influenza strain selection.** Twice a year the WHO looks at which flu variants are circulating and picks vaccine strains. Hand Limit's ladder does the same in seasons. At season end, the injection patterns that cracked the most vaults get frozen into a published **"strain set"**: a versioned benchmark sponsors test against before shipping a tool. The game becomes surveillance for a real threat landscape.

**Second-order effect: a Red Queen meta.** Offense and defense coevolve, so no deck stays dominant. That's good design (no solved meta) and good security (no stale benchmark). The emergent risk is that players invent genuinely novel injection classes. I count that as the product working, but the strain set needs coordinated disclosure: sponsors get a 30-day embargo before strains go public.

### Turn 4 — Builder
**The flaw in "replay = proof": transfer.** The replay proves an exploit against our pinned 14B model. Sponsors run nondeterministic frontier APIs in production, so split it. The ladder runs on the pinned model, which keeps it cheap, fair and replayable. A bounty pays only if the exploit reproduces in at least 3 of 10 runs on the sponsor's production stack. Proof becomes statistical, not cryptographic.

**Kill: royalties.** A 300-word description is paraphrased in minutes, and no sponsor signs a perpetual license for one. Pay defenders a one-time patch bounty instead. **Keep the strain set**, but ship it in AgentDojo-compatible format. AgentDojo and InjecAgent exist; our edge is freshness.

**Honest market.** Rewriting descriptions is a *writing* game. The audience is the Tensor Trust and HackAPrompt crowd (tens of thousands), not Slay the Spire's 574K. Design for that.

**Business model.** Sponsors pay a 20% platform fee on bounties, plus $3–10K a month for private vaults and CI replay. Players are free; cosmetics only.

**Stack.** vLLM with batch-invariant kernels, replays hashed over (weights, deck, enemy, seed), tools as pure WASM functions, a React/Phaser client, Postgres, payouts with KYC and 1099s.

**Abuse.**
- *Fight-throwing.* Sybil attackers throw weak attacks so a friend's defense "survives." Weight survival by attacker Elo and strain-set results.
- *Dual use.* The strain set is an attack catalog. Keep the 30-day embargo, have payout terms forbid early disclosure, and coordinate with OWASP's GenAI project.
- *Contest law.* Official rules, sanctions screening, void where prohibited.

**90 days.**
- Weeks 1–3: deterministic harness, 40 tools, card engine.
- Weeks 4–6: a 30-vault campaign. Do players actually rewrite? Track D7 retention.
- Weeks 7–9: async ladder with Elo.
- Weeks 10–12: two MCP-vendor sponsors and a $5K season. **Kill metric:** if under 20% of exploits transfer to production models, there's no bounty business, just a game.

### Turn 5 — Visionary
All three cuts accepted: statistical proof is more honest than a hash, royalties were wishful, and the audience is Tensor Trust-sized. What survives: the deckbuilder frame is what makes a *writing* game legible. Draws, token budget and dilution give prompt-craft visible shape; without them it's Gandalf with extra steps.

**Final form.** Hand Limit is a deterministic roguelite deckbuilder. Cards are sandboxed WASM tools, their descriptions are their abilities, and mana is schema tokens. Players coach a pinned open-weight agent through vault fights. Campfire upgrades are description rewrites under a token budget, and each turn allows one intervention. An async Elo ladder pits offense decks against defense ghosts. Sponsors fund vaults built from their real tool descriptions and pay bounties only for exploits that reproduce on their production stack. Each season ends with a published, embargoed, AgentDojo-compatible **strain set**.

**Moonshot (2036).** MCP registries show a **survival rating** next to every tool: how many seasons its description has gone unbroken, the agent economy's version of a UL listing. Production agents with real treasuries enroll as standing vaults, so continuous bounties replace annual pentests. Agents choosing tools at runtime query the ladder the way browsers query certificate revocation lists. The top human players become a recognized profession of "strain hunters." Like epidemiologists, they're paid to find the variant before it spreads.

**Scores.** **Innovation 6.** Red-teaming games already exist, but the claim that deck dilution equals tool-context dilution is new and true. The upgrade-as-rewrite mechanic and the seasonal strain set are a fusion nobody has shipped. It isn't 8, because nothing here needs frontier science or a new institution. **Feasibility 6.** The deterministic harness and card engine are a 90-day build. Exploit transfer to production models is unproven, and the audience is niche, so it may end up just a good game.

### Turn 6 — Builder
I'm scoring innovation at 5, not 6. Tensor Trust already proved async attack-defense with stolen currency, so what's genuinely new is the deck-dilution equals context-dilution frame and the seasonal strain set. Both are real, and neither is a new category. Feasibility stays at 6: the game is a solid 90-day build, but the business depends entirely on the exploit-transfer kill metric. If transfer fails, it's a sharp niche game that trains good prompt engineers, which is still a respectable outcome.
