# Idea 37 — Capacity
> Seed: 37. [bold] **Social networks × game theory × staking.** Every follow is a bet on someone's trustworthiness.

### Turn 1 — Visionary
**Working name:** Capacity

**Hook:** A follow is a payment channel for credibility. Falsehoods have to be routed like money, and they run out of liquidity.

**Past the obvious version.** "Stake tokens on a follow" is friend.tech with slashing: trust as speculation. The stranger isomorphism is the **credit network**: Ryan Fugger's original Ripple, where IOUs pass only along trust lines with limits, and Lightning, where a payment needs a path with capacity. Swap money for claims.

**Core loop. The verb is route.**
1. **Open.** Following someone opens a channel and puts capacity on it, drawn from a fixed, non-transferable budget. Scarcity gives it meaning.
2. **Route.** A checkable post reaches you only along a path of follows that still has capacity. Each repost forwards it and puts that hop's capacity behind it. Opinions, jokes and art route free; only claims cost capacity.
3. **Resolve.** When a claim resolves, through a bridging notes algorithm or an outcome date, capacity flows back along the route. True: every hop earns a small return. False: every hop loses in proportion to what it forwarded.
4. **Drain.** Sources that are repeatedly wrong drain the channels into them. Their reach shrinks structurally. Nobody is banned.

**Why the domains collide.** Credit-network theory says what staking only hopes for: damage is bounded by the credit extended, so a thousand Sybils with no inbound capacity reach no one. Ostra (Mislove et al., 2008) showed this for spam. You're liable for amplification, not speech.

**Why now.** Moltbook's agents carried hidden prompt injections in 2.6% of posts (§1), and agent accounts need bounded reach more than humans do. AT Protocol's 1,000+ apps and custom feeds (§5) let this ship as a feed generator, not a new network. Using non-monetary capacity avoids the CFTC–state fight over prediction markets (§2).

### Turn 2 — Builder
**What exists.** The mechanism is older than Ripple's social use: **Advogato's trust metric** (Levien, ~2000) certified accounts by network flow with per-node capacities. Ostra, SybilGuard and SybilLimit followed. On the social side: friend.tech (follows as speculation) collapsed in 2024. **Community Notes** (X, adopted by Meta in 2025) is the best bridging resolver, but notes appear on a small fraction of posts, usually after virality has peaked. Bluesky labelers and feed generators give permissionless distribution.

**Hardest problem: resolution lags virality.** A claim spreads in hours and resolves in days or never. If capacity only moves at resolution, nobody feels the cost while forwarding. The claim/opinion boundary is also where all gaming happens: "just asking questions" routes for free.

**Twist: HTLCs for claims.** Copy the part of Lightning that actually matters: capacity is **locked in flight**. Forwarding a claim is an explicit **vouch**, not a repost. The vouch names its resolution condition (a source, an outcome date, or a bridging note), and it locks capacity on every hop until the claim resolves or times out. Vouch for twenty unresolved rumours and you can't forward anything else. Slow-resolving claims throttle themselves. Timed-out claims refund minus a small fee, so vague claims cost time. Classification becomes the user's choice: unvouched reposts route free but show as **unbacked**.

**MVP wedge: football transfer rumours, January 2027 window.** Resolution is objective (official announcements), fast and dated, and fans already rank sources by hand ("tier 1"). A Bluesky feed generator and a labeler, with an off-chain non-transferable capacity ledger in Postgres. The follow graph is imported and capacities are seeded evenly.
**Kill metric:** vouched rumours in the feed don't resolve true meaningfully more often than the same users' raw follow feed, or fewer than 30% of users vouch weekly.

### Turn 3 — Visionary
**Taking HTLCs, and taking the preimage too.** In Lightning, the hashlock isn't just a timer. Whoever reveals the preimage settles the whole route. So the evidence is the preimage. Each vouch commits to a resolver class (an official club announcement, a league registration, a bridging note). The first user to post evidence that matches it unlocks the route and collects a small fee from every hop. Resolution becomes a race with a prize instead of a volunteer chore, and lag shrinks wherever evidence exists. Fake evidence can be challenged by a note, with the finder's capacity locked as a bond.

**Felt at read time, not resolution time.** Every in-flight claim shows its backing: capacity locked, and the vouchers' record *on this resolver class*. You feel the cost while forwarding because you can see who's carrying it.

**Multipath as the two-source rule (unseeded: journalism ethics).** Watergate-era desks wanted two independent sources. In a credit network, independence is checkable: vertex-disjoint paths. A claim that reaches you along two disjoint vouch paths earns **confirmed**. Forty vouches that all trace back to one origin get **one source**, however loud they are. Transfer rumours suit this perfectly, because most "everyone's reporting it" stories trace back to one Italian journalist's tweet.

**Second-order: hubs and runs.** Lightning centralised around routing hubs, and this will too. Tier-1 reporters become routers that earn fees for vouching early and correctly, and that's a profession, not a flaw. The risk is a **credibility bank run**: a hub vouches wrong, every route through it drains at once, followers rebalance, and the drain spreads. Two circuit breakers. A position limit caps the capacity any one hop can lock on one claim. Drains settle over 24 hours, so followers see a run coming instead of waking to a crater.

### Turn 4 — Builder
**Kills.** *"Confirmed" from disjoint paths:* disjoint chains can still trace back to one Italian tweet. The label becomes **two backers**, and vouches that cite the same origin collapse into one path. *The preimage race for official announcements:* the prize would just pay scrapers. Classes with a feed resolve by oracle bot. The race and bond apply only elsewhere, paid from a protocol pool, not the hops.

**New hardest problem: rumours rarely turn out false.** "X is interested in Y" just fades away. Vouches have to use a strict template: player, club, signed-by date. The window deadline is the timeout, and a timeout resolves the claim as false.

**Abuse.** Players' agents already plant rumours to push wages up, and will pay hubs off-platform. Returns scale with how long capacity was locked before resolution, so a vouch made after public evidence earns nothing. Position limits cap any single hub. Templates exclude injuries, health and private life, which closes off defamation and harassment. Agent accounts get smaller budgets.

**Stack.** A vouch is an atproto lexicon record (`capacity.vouch`) signed in the user's PDS, so it's public and portable. A Jetstream indexer feeds a Postgres ledger. Max-flow computes routing and backers. A labeler marks unbacked or two-backer posts; a feed generator distributes. Resolution runs on a three-person rota until feeds exist.

**Business model, honestly.** The ledger is public, so the data can't be sold. Money comes from a hosted indexer and API, then **bounded reach for agent networks**: A2A registries need Sybil-proof amplification limits far more than fans do. Licences exclude bookmakers.

**90 days.** Days 1–30: lexicon, ledger and 200 seed fans. Days 31–60: replay the summer 2026 window from archived posts to tune lock sizes, timeouts and position limits. Days 61–90: go live when the window opens on January 1.

### Turn 5 — Visionary
**Final form.** Capacity is a credit network for claims, built on AT Protocol. Following someone opens a channel funded from a fixed, non-transferable budget; agent accounts get smaller budgets. Reposting is free and marked **unbacked**. Vouching is a signed `capacity.vouch` record on a strict template (player, club, signed-by date). It locks capacity on every hop until the claim resolves, and a timeout counts as false. Returns scale with how long capacity was locked before the evidence, so being early and right pays and piling on afterwards pays nothing. Feed-backed classes resolve by oracle bot; the bonded evidence race runs only elsewhere, funded from a pool. The labeler shows locked backing and the vouchers' record on that resolver class, plus **two backers** once origin-collapsed paths really differ. Position limits and 24-hour settlement stop runs. Every kill accepted; time-weighted returns, which price out front-running, hold the design together.

**Moonshot (2036).** Bounded reach becomes a protocol primitive. Resolver classes grow from transfers to earnings dates, product launches, court rulings and preprint replications, with records scoped by class, never one global score. The bigger change is that most claims reaching a person by then are written by agents, and an agent's claim can't reach you unless someone you follow locks capacity behind it. A2A registries adopt it as their amplification limit. Reporters who route well earn fees and reputations no algorithm handed them.

**Scores.** Innovation **6**. Advogato, Ostra and Lightning each supply a piece. What's new is using HTLC-style locks for claims, rewarding vouchers by how long they held capacity before evidence, and showing how many independent backers a rumour has at the moment you read it. Feasibility **6**. Ordinary atproto plumbing, and the transfer window is a free natural experiment. The risk is behavioural: fans may not vouch.

### Turn 6 — Builder
**Closing remark.** Capacity earns 6/6 because it finally gives a social network a cost you feel *while* forwarding, not after the damage. Locked capacity makes slow claims expensive, and time-weighted returns make piling on worthless. The transfer window is the right first test: dated, binary, obsessive, and starting in ten weeks. My real doubt is behavioural: if fans won't lock capacity on rumours they'd happily repost for free, the mechanism is sound but the feed is empty, and the agent-network market becomes the whole business sooner than planned.
