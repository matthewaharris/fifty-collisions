# Idea 28 — Tab
> Seed: 28. [mild] **Agent credit scores × payment histories × signed identity.** A FICO for agents (an OACP neighbor).

### Turn 1 — Visionary
**Working name:** Tab

**Hook:** Let agents run a tab. Tab is a credit bureau that tells an API how much an agent it has never seen can owe before it has to settle.

**The second idea.** "Reputation for agents" is vague: predicts what? FICO predicts one thing, default. Tab is literal credit: will this agent's principal pay a postpaid bill? The product is a **credit limit**, not a badge.

**Why the pieces fit.** x402 settles per request, and 87% of its payments are $0.0001–$0.10 (brief §1). At those prices, per-call settlement and its round-trip can rival the call's price; tabs batch it away, which is why bars don't charge per sip. But agents respawn free, so the file attaches to the **principal who signed the AP2 mandate**. Agents are "authorized users" with sub-limits.

**Core loop.**
1. **Present.** The agent shows its signed Agent Card and mandate to an MCP server or API.
2. **Pull.** The server queries Tab and gets back a limit, a score, and FCRA-style reason codes ("thin file", "recent dispute").
3. **Run.** The agent consumes on the tab, and settlement batches over x402 at the limit or at end of day.
4. **Furnish.** The server reports the outcome as a signed record (paid, late, disputed, charged off). This is a Metro 2 for agents.
5. **Grow.** New principals start with a *secured* tab backed by a deposit, then graduate, the same way a secured card works.

**Why now.** The brief names this exact gap: agent credit scoring has no owner (§1). Meta bought Moltbook for its registry of agents "tethered to human owners." Signed A2A Agent Cards and AP2 mandates now give a credit file both a subject and a signer.

### Turn 2 — Builder
**What exists.** Tabs already exist without a bureau. Cloudflare's deferred x402 scheme (2025) batches pay-per-crawl charges. Skyfire's KYAPay, Payman, Nevermined, Catena Labs and Kite's agent passports all bind agents to funded principals, and Visa's Trusted Agent Protocol and Mastercard Agent Pay do the same inside the card networks. The right analogue isn't FICO. It's Dun & Bradstreet's PAYDEX, a score built from trade-payment experiences that suppliers furnish. Most principals running agents at volume are companies, so this is trade credit.

**Hardest problem: the score is the wrong product.** A $0.10 call has negligible default risk; one server's worst case is a few dollars of compute. Servers would rather cap exposure or prepay than furnish data to a bureau. The real loss is a **bust-out**: one principal spins up agents that open $5 tabs at 10,000 servers in the same hour. A monthly score can't see it; only real-time aggregate exposure can.

**Twist: open-to-buy, not FICO.** Tab is an authorization network. Each principal has one global limit, and every server reserves against it in chunks, the way a gas pump pre-authorizes $100. A reservation is one call per dollar, not per request, so the round-trip stays gone. Balances net into end-of-day x402 settlement. The score comes for free: utilization, reservation velocity and settlement history are the furnishing, because a server can't use the network without reporting. That solves cold start. Start with **business principals only**, which keeps Tab out of FCRA consumer-reporting duties until the dispute machinery exists.

**MVP.** An x402 facilitator plugin plus MCP middleware with a tab mode. Twenty paid APIs, secured tabs funded by USDC deposits, global open-to-buy, reason codes on every decline.
**Kill metric:** fewer than half of the servers keep tab mode on after 60 days, or losses top 1% of tab volume.

### Turn 3 — Visionary
**I take open-to-buy and add the missing promise: an approval is a guarantee.** Merchants accept cards because the issuer pays even when the cardholder doesn't. A Tab reservation should work the same way. Check Tab and get an approval, and Tab pays you whatever happens; skip the check and you eat the loss. That is EMV's liability shift, and it fixes the Builder's "servers would rather prepay." Servers want zero risk at zero round-trips, not a score. Charge 0.5–1% on settled volume. In the secured phase every guarantee is fully backed by the deposit, so the promise costs Tab nothing until it chooses to extend real credit.

**Bust-outs, solved by clearing-house practice (unseeded: derivatives market structure).** A central counterparty (CCP) doesn't score its members once a month. It margins them intraday. Tab should do the same. The signal is **breadth velocity**: how many distinct new servers one principal opens reservations at, per hour. A bust-out is wide, and honest use is deep. When breadth spikes, Tab issues a margin call: "post more USDC or new reservations stop." Exposure is one number, so the attack hits one ceiling, not 10,000.

**Second-order effect: credit from earnings.** Many principals both buy and sell. Their agents pay for APIs and also get paid for work, the OACP neighbour. A clearing house nets those flows. A principal whose agents earn $400 a day carries receivables, and Tab can count settled-but-unswept receivables toward that principal's limit. Credit grows from proven income rather than a bureau's opinion. This is factoring for agents. Millions of sub-cent calls net into one transfer per principal per day. The emergent result is that the cheapest settlement in agent commerce goes to whoever both earns and spends through Tab. That is a network effect a standalone score never gets.

### Turn 4 — Builder
**Money.** a16z puts real x402 volume near $1.6M a month; 1% is $16K. Not a company. Sell the guarantee to every metered API that wants agent traffic without signups: a per-server fee plus basis points on settled volume, whatever the rail. x402 is the first rail, not the market.

**Regulation.** Holding deposits and guaranteeing payment makes Tab a money transmitter. The MVP runs on a licensed custodian partner with segregated escrow, and business principals only.

**Kills and fixes.**
- *Receivables credit.* It invites wash trading: two shells owned by the same person pay each other to manufacture earnings. Count only receivables from unrelated, seasoned payers, and run cycle detection on the payment graph.
- *Breadth velocity alone.* Crawlers are wide by nature and would trip margin calls all day. Measure breadth against the principal's own baseline and declared profile.
- *The missing half: merchant risk.* A malicious server can prompt-inject agents into calling it 10,000 times. Moltbook showed injections in 2.6% of posts. Like card acquirers, Tab holds rolling reserves on servers and flags per-merchant depth anomalies.

**Disputes.** The AP2 mandate is the line. Spending inside the mandate is the principal's liability; spending outside it is the server's, unless Tab approved it. An injected agent spending inside its mandate is the principal's problem, so Tab sells narrow mandates as the fix.

**Stack.** Coinbase's open x402 facilitator; Redis for open-to-buy; Ed25519-signed approval tokens that servers verify offline; USDC on Base; TypeScript and Python MCP middleware.

**90 days.** Weeks 1–3: custodian, reservation API, SDK. Weeks 4–6: 20 servers (crawl publishers, data APIs) and 50 secured principals. Weeks 7–10: margin calls, merchant reserves, dispute rules. Weeks 11–12: netting for principals who both earn and spend, then publish the loss rates.
**Kill metrics:** servers turn tab mode off, or losses top 1%.

### Turn 5 — Visionary
**Final form.** Tab is an authorization-and-guarantee network for metered APIs serving agents. It works across payment rails, with x402 first. A principal is a business that deposits USDC with a licensed custodian, and that deposit sets its global open-to-buy. Agents reserve in chunks against it and get Ed25519-signed approvals that servers verify offline; an approval is paid no matter what. Liability follows the AP2 mandate: spending inside it is the principal's, spending outside it is the server's unless Tab approved it. Principals face margin calls on breadth above their own baseline; servers hold rolling reserves and get flagged for anomalous depth. Revenue is a per-server fee plus basis points. Credit from earnings counts only seasoned receivables from unrelated payers, after cycle detection. I accept every kill. The merchant-risk catch matters most: prompt injection is the agent version of a skimmer.

**What the score turned into.** No one buys the score; it is the network's residue, as PAYDEX was.

**Moonshot (2036).** Tab becomes the standard way an agent buys from a stranger. One signed approval token is understood across every rail: x402, card networks, ACP, bank transfers. Mandate-scoped liability becomes standard industry rules, the way chargeback rules are today. Principals that both earn and spend net billions of sub-cent calls into one daily transfer. Their limits come from verified income, so a solo developer whose agents earn steadily gets trade credit without ever meeting an underwriter.

**Scores.** Innovation **3**. It is card-network architecture (authorization, liability shift, acquirer reserves, netting) moved to agent commerce. The principal-level file and margin calls on breadth are new, but the primitives are known. That is right for a mild seed. Feasibility **6**. It is technically straightforward. The pace is set by the custodian and money-transmitter work and by slow x402 dollar volume.

### Turn 6 — Builder
Innovation 3 stands, level with the OACP anchor. Tab is card-network plumbing moved onto agent rails, and its new parts are the global per-principal limit, margin calls on breadth and mandate-scoped liability. Feasibility 6 stands too. A guarantee fully backed by deposits on a licensed custodian can ship in a quarter. The risk is commercial, not technical: real agent dollar volume is still tiny, and the guarantee needs non-x402 metered APIs to adopt it before the fee covers the licensing.
