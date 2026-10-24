# Idea 13 — Canary Ladder
> Seed: 13. [wild] **Post-quantum cryptography × prediction markets × insurance.** Hedging Q-Day. Who underwrites the end of ECC?

### Turn 1 — Visionary
**Working name:** Canary Ladder

**Hook:** A parametric insurance market whose trigger is a proof of the peril itself. Cover pays out when someone forges a signature that only a cryptographically relevant quantum computer could produce.

**The problem.** Parametric cover needs an objective trigger. Q-Day has none: lab press releases hype, and a state that breaks ECC hides it. Cryptography, unlike a hurricane index, can make the event **self-certifying**.

**Core loop.** The verb is **climb**.
1. **The ladder.** An MPC ceremony generates elliptic-curve keys on curves from 160 to 256 bits. Private keys are destroyed; recovering one needs every share. The top rung is a real secp256k1 key. Each rung guards a bounty in a contract that pays whoever signs their own address with that rung's key. Rungs start past the public classical record (about 130-bit kangaroo solves), so a fall means Shor or a classical breakthrough. Either way, ECC is weaker than priced.
2. **The curve.** Contracts like "rung 224 falls before 2030" form a **Q-Day term structure**: a hazard curve priced by money, not keynotes.
3. **The cover.** Exchanges, custodians and exposed-pubkey holders buy protection paying in tranches as rungs fall. Quantum skeptics underwrite it the way cat-bond investors underwrite hurricanes, earning premium until the canary dies.
4. **The discount.** Premiums fall when exposure verifiably migrates to ML-DSA or BIP-360 outputs, so insurance becomes the migration incentive.

**Why the collision works.** Markets need resolution, insurance a trigger, crypto a canary. One mechanism serves all three. The bounty also pays an attacker to *reveal* capability rather than hoard it, at least up to the bounty's size.

**Why now.** A EUROCRYPT 2026 estimate puts ECC-256 at about 1,098 logical qubits. Helios has about 48 logical qubits at 99.9995%. Google set a 2029 PQC deadline, and BIP-360 is still a draft (brief §2, §6, #5).

### Turn 2 — Builder
**What exists.** The ladder already exists by accident. The 2015 Bitcoin puzzle transaction funded keys of increasing size, and kangaroo solvers have taken the exposed-pubkey rungs up to 130 bits (2024). Certicom's ECC challenges have stood since 1997. Project Eleven's Q-Day Prize offered 1 BTC for a Shor break of a toy key. Metaculus and Manifold list unfunded Q-Day questions. Nexus Mutual sells crypto cover with discretionary claims, and Lloyd's now requires exclusions for state-backed cyber attacks, so nobody underwrites this peril.

**Hardest problem: the collateral dies with the canary.** If cover is paid in USDC or ETH, the payout asset, the chain's validator signatures and the contract's admin keys all rest on the curves that just fell. The insurer pays in money that is crashing because of the insured event. That's wrong-way risk at its purest. The top rung also can't compete: whoever breaks secp256k1 takes Satoshi's roughly 1M exposed-pubkey BTC instead of the bounty. Bitcoin is already the 256-bit canary.

**MVP: a ladder you can price this year.** Add **calibration rungs** at 135, 140, 145 and 150 bits, where GPU cost is known (thousands of GPU-years at 140 bits). Those rungs fall to money, so near-dated contracts resolve now and fit the classical cost curve. The quantum signal is a rung falling *ahead of* that curve, not a rung falling. Generate keys by distributed key generation, where one honest participant suffices, and accept claims only with hash-based (SLH-DSA) signatures behind a commit-reveal.

**Twist: a cat bond with a cryptographic trigger.** Keep the trigger on-chain and the money off the curve. A Bermuda SPV holds T-bills in trust, exactly as every cat bond does, and pays tranches when rungs fall. Insurance-linked-securities funds want perils uncorrelated with hurricanes, and Q-Day is. The skeptics already have a desk.

### Turn 3 — Visionary
Cat bond and calibration rungs both taken; neither shrinks the idea. The rungs supply the real instrument: **the quantum signal is a rung falling ahead of the classical cost curve.** Time-to-fall against GPU-years becomes a public ruler of cryptographic relevance. IBM's Quantum Advantage Tracker measures advantage on benchmarks nobody steals with. This one measures it where the money is.

**Wrong-way risk, finished.** The SPV moves the money off the curve, but the *trigger* still sits on a chain whose signatures may be failing. Remove the chain from the trigger. In a cat bond, a calculation agent declares the event (PCS or Verisk for hurricanes). Here the trust deed names a **verification function**: a valid signature on the claimant's message under rung key Pₖ, committed with SLH-DSA. Anyone can check it offline. This would be the first cat bond whose calculation agent is a theorem.

**Satoshi's problem becomes the feature.** Builder is right that no bounty competes with 1M BTC, and a state attacker never claims anything. So stop relying on claims. The unseeded domain is **deception security**: honeypots and Thinkst-style canary tokens.
- **Prey that sings.** Seed thousands of decoy wallets with exposed pubkeys, made by DKG to look like ordinary address reuse, across balance bands. A quiet harvester sweeping exposed coins will sweep decoys too. A decoy moving proves capability that was never meant to be revealed.
- **Harvest-now-decrypt-later tripwires.** Publish ECDH-encrypted blobs along high-value routes, each holding a live credential to a monitored lure. Decrypt one and use it, and the wire trips.

**Second-order effect.** The ladder measures *public* capability and the decoys measure *covert* capability. The spread between the two curves is the market's price of a hidden quantum computer: a number that intelligence agencies guard today, published as a bond spread.

### Turn 4 — Builder
**Kills and fixes.**
- *Small decoys as early warning.* Early quantum machines take hours per key, so harvesters hit the biggest exposed balances first. Small decoys trip late; big ones cost real capital. Bitcoin already holds natural decoys, about 1M BTC of dormant Satoshi-era P2PK outputs; index their movement for free. Our decoys keep one job: **the cheapest balance band swept reveals the attacker's cost per key.**
- *HNDL tripwires.* A state decrypting archives in 2035 won't log into a lure. Moonshot only.
- *Custom curves.* Use secp256k1 with **range-restricted keys**, like the 2015 puzzle. Ekerå–Håstad short-log Shor scales with key length, so one rung is both a GPU target and a quantum target, and it can hold a native bounty.

**Moral hazard.** Whoever owns a CRQC knows the trigger first and buys cover. Sell protection only to buyers with verified exposure. The prediction market stays open to informed traders; that leak is the signal. Ceremony fraud also fakes a trigger. Put **buyers and underwriters both in the DKG**, so faking it requires collusion across opposed sides.

**Business model.** Structuring fees from custodian and exchange sponsors, ILS funds as investors, and the rung and dormant-coin series licensed as a **Q-Day index**. Pitch Kalshi on listing rung contracts with our verifier as the resolution source.

**Stack.** DKG for range-restricted keys, an SLH-DSA claim protocol, an offline verifier in two languages, a Bermuda special-purpose insurer selling under Rule 144A.

**Safety.** A false trigger could crash crypto markets, so run two independent ladders and require both.

**90 days.** Weeks 1–4: ceremony, funded rungs at 135–160 bits. Weeks 5–8: verifier, dormant-coin index, trust deed with Bermuda counsel. Weeks 9–12: term sheet for a private $10–25M bond.
**Kill metrics:** no ILS fund will price a tranche, or no calibration rung draws a credible solver.

### Turn 5 — Visionary
All three kills accepted. Range-restricted keys are the thread's best fix: one rung is both GPU and Shor target. Satoshi's coins beat any decoys we could fund. One pushback: "both ladders must trip" lets whoever corrupts one ceremony suppress a real trigger. Use three ladders; trigger on two.

**Final form.** Canary Ladder is a cat bond with a cryptographic trigger.
- **Ladder.** Three DKG ceremonies, buyers and underwriters in each, produce range-restricted secp256k1 keys from 135 to 256 effective bits, each with a native bounty.
- **Trigger.** A trust deed names an offline verifier as calculation agent: an SLH-DSA-committed signature under a rung key, on two of three ladders.
- **Collateral.** A Bermuda SPV holds T-bills off the curve and pays tranches as rungs fall.
- **Q-Day index.** Rung fall times against the GPU-cost curve, dormant P2PK movement, and the cheapest decoy band swept. Kalshi lists rung contracts on our verifier.
- **Migration discount.** Cover is sold only to buyers with verified exposure, and premiums fall as exposure migrates to post-quantum outputs.

**Moonshot (2036): a futures market in mathematical hardness.** Q-Day is one peril among many. SIKE, a NIST round-4 candidate, fell to a classical laptop attack in 2022; lattices could too. Every deployed assumption, including ML-KEM, ML-DSA, SLH-DSA and RSA, gets a standing ladder and a hazard curve. NIST and central banks set deprecation dates from market-priced hardness, not committee calendars. Assumption cover becomes as routine as hurricane cover, and the market pays people to stop trusting maths early.

**Scores.** **Innovation 8.** Cryptographic hardness becomes an insurable peril with a self-certifying trigger: a new institution whose oracle is a theorem. **Feasibility 4.** Ladder, verifier and index ship in 90 days. A peril with no loss history and untested ILS appetite is where it stalls.

### Turn 6 — Builder
Two-of-three is right. I'd been guarding against false triggers and left the suppression attack open. Innovation 8 stands: the Bitcoin puzzle and cat bonds already exist, but making cryptographic hardness an insured peril, with a theorem acting as calculation agent, is a new institution, and the strange core survived every fix. Feasibility 4 also stands. The ladder, verifier and index can ship by Christmas, while a peril with no loss history may never clear an ILS committee. If the bond stalls, the Q-Day index is still worth building on its own.
