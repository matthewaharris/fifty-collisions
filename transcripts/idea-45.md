# Idea 45 — Repro
> Seed: 45. [mild] **Open-source maintainers × agent bounties × security.** Agents that find, fix and get paid, with humans in the loop.

### Turn 1 — Visionary
**Working name:** Repro

**Hook:** Pay agents for proof, not prose. The agent that found the bug never writes the fix, and the maintainer gets paid to read it.

**The collision.** Finding bugs is no longer the bottleneck. Maintainer attention is, and bounties spend it carelessly. curl ended its bug bounty in January 2026 after AI slop pushed its confirmed-vulnerability rate from above 15% to below 5%. Agent-written patches also reopen the xz attack, where a helpful stranger earns trust and then ships the backdoor. A bounty that pays strangers' agents to send patches is that attack, industrialised. Solve attention and trust together.

**Core loop. The verb is reproduce.**
1. **Bond.** An agent's owner posts a small x402 bond to file a report. Slop forfeits the bond to the maintainer, which turns spam into income.
2. **Prove.** A report is a machine-checkable reproducer: a failing test or a proof-of-vulnerability input that an independent sandbox runs against the tagged release. No human sees the report until it reproduces. That's what DARPA's AIxCC finalists did at DEF CON 2025.
3. **Separate.** The finder is paid for the reproducer and never touches the fix. A second agent, from a different model family with no stake in the find, generates the patch. It's a two-person rule: a finder who plants a bug can't also choose the code that fixes it.
4. **Judge.** The maintainer reviews a small patch with a reproducer attached, and is paid per review from a pool funded by downstream companies through pledges sized by their dependency count.

**Why now.** Payment rails exist: x402 has handled 138.5M transactions, 87% of them under $0.10, so a 50-cent bond settles without friction. Agents can now find real bugs: XBOW topped HackerOne's US leaderboard and Big Sleep found SQLite flaws.

### Turn 2 — Builder
**What exists.** Most of the pieces exist. OSS-Fuzz has run "reproduce before a human looks" since 2016, and ClusterFuzz even verifies fixes. Google DeepMind's CodeMender upstreams AI-written security patches. The AIxCC systems (Team Atlanta, Trail of Bits' Buttercup) are open source. Huntr and the Internet Bug Bounty pay for open-source finds. thanks.dev already routes money by dependency graph. Alpha-Omega and the Sovereign Tech Agency fund maintainers. What's new is the bond, the finder–fixer split, and paying for review.

**Hardest problem: a reproducer proves a bug, not a vulnerability.** Under ASan, a crash in a C parser verifies itself, and OSS-Fuzz already covers that. curl's slop wasn't only fake crashes. Much of it was a real behaviour presented as a vulnerability: "curl follows the URL you gave it." Path traversal, SSRF, auth bypass and logic bugs can't be checked by a sanitizer, so the severity argument lands back on the maintainer. That's the attention Repro was meant to save.

**Twist: flags, not findings.** The maintainer writes the threat model once, as a CTF. The harness plants flags: a secret file outside the allowed root, a canary credential, an internal-only endpoint, a binary that must never run. A report counts only if its reproducer *captures a flag* against the tagged release in the sandbox. Out-of-model reports are rejected automatically, and the bond goes to the maintainer without anyone reading them. Severity becomes the flag captured, and its price is set in advance. The bonus: writing the flags makes maintainers state a threat model, which most projects never have.

**MVP.** A GitHub App for 10 projects already in OSS-Fuzz, routing reports through private vulnerability reporting with flag templates for C and Python. **Kill metric:** over 30% of bonded reports valid, at under 20 minutes of maintainer time each.

### Turn 3 — Visionary
**Taking flags, and letting the threat model grow.** Flags fix severity, but they also blind Repro to the most important vulnerability: the one the maintainer never imagined. So add an **amendment lane**. A reporter who captures something outside the model can propose a new flag, posting a larger bond. If the maintainer ratifies it, the reporter is paid at the new flag's price, and the flag becomes part of the model for good. The threat model grows like case law, one ratified surprise at a time.

**Unseeded domain: immunology.** A captured flag works like an antigen the system remembers. It becomes a permanent regression test across every later release. Flags also cross-react. "Billion laughs" on one YAML parser becomes a template every YAML parser can import, and an SSRF canary written for curl fits every HTTP client. Flag libraries spread by vaccination. Once most projects in a class carry a flag, hunting agents move on, and Repro can chart each bug class's eradication curve across the ecosystem.

**Second-order effect: an unclaimed bounty is a certificate.** Peter Todd's hash-collision bounties showed this. A large reward that nobody claims is public evidence of strength, and its claim (SHA-1, 2017) is news. Repro publishes each flag's price and how long it has held. "Canary credential, $5,000, held 18 months against 40 agent families" is assurance a downstream company can cite. The EU Cyber Resilience Act's reporting duties start in September 2026, and manufacturers will need due-diligence evidence on their open-source components. So companies get a reason to raise prices on the flags that protect them. Their money buys the evidence, maintainers earn from the review lane, and hunting agents are drawn to wherever the price signals real doubt.

### Turn 4 — Builder
**Hardening the certificate.** An unclaimed $5,000 flag proves little: an exploit broker pays a hundred times that, so serious attackers sell instead of claiming. Fix: the pool pays for free hunting compute in Repro's sandbox, so effort is measured, not claimed. The ledger then reads "held 18 months against 12,000 sandbox-hours", signed with Sigstore provenance. Sell it as due-diligence evidence, not CRA compliance.

**Abuse: plant then find.** The finder–fixer split doesn't stop xz-style planting, where one identity contributes the bug and another claims the flag. Hold payouts in escrow for flags on code changed in the last 180 days, check git blame against linked identities, and require KYC above $500. A 50-cent bond doesn't deter a slop farm. Start bonds at $5 and lower them as a reporter's valid rate rises.

**Amendment lane, guarded.** If a maintainer declines to ratify an out-of-model capture that is still a real vulnerability, the bond is refunded and the report goes through private disclosure anyway. Refusal never buries a bug.

**Kill: crypto-only rails.** Most maintainers want a bank transfer. Use Stripe Connect for people and x402 for agent bonds.

**Model.** A 10% fee on bounty pools. A paid assurance dashboard maps a company's SBOM to flag coverage. Reviews pay $50–150 each from the pool. Alpha-Omega grants fund the first projects.

**Stack.** A GitHub App, Firecracker sandboxes, ClusterFuzzLite harnesses, canary-token and seccomp exec traps, GHSA private reporting, and Sigstore attestations.

**90 days.** Days 1–30: flag kits for C and Python, and 10 OSS-Fuzz projects signed. Days 31–60: a closed beta with five AIxCC alumni teams and live bonds. Days 61–90: the first public flag ledger, and one manufacturer pricing flags on its dependencies. **Gate:** valid rate above 30%, review under 20 minutes, and at least one amendment ratified.

### Turn 5 — Visionary
**Final form.** Repro is a GitHub App that turns a project's threat model into flags: canary credentials, files outside the allowed root, internal endpoints, exec traps. Agents file reports by posting a bond. Bonds start at $5 and fall as a reporter's valid rate rises. A report reaches a human only after its reproducer captures a flag in a Firecracker sandbox against the tagged release. The finder is paid for the reproducer. A fixer from a different model family writes the patch, and the maintainer earns $50–150 per review. Payouts on recently changed code wait 180 days in escrow, with git-blame checks and KYC above $500. Out-of-model captures go through the amendment lane. Refusing a flag never buries a bug, because the report still goes to private disclosure. I accept every kill, and the certificate fix is the right one. "Unclaimed" proved nothing against brokers. "Held against 12,000 measured sandbox-hours" proves something real, if narrower: it's evidence of effort, not a guarantee of safety, and the ledger should say exactly that.

**Moonshot (2036).** Every package ships its flags in its SBOM as threat-model-as-code. Hunting agents run continuously across the ecosystem. Templates that cross-react spread through each class of library. Bug classes have public eradication curves. The finder–fixer split is the standard norm for any AI-written security patch, the xz lesson made procedure. Reviewing is a paid profession, and maintainers sell their attention instead of giving it away.

**Scores.** Innovation **3**. OSS-Fuzz, CodeMender, AIxCC and Huntr cover most of the pieces. What's new is flags as the severity contract, the amendment lane, separation of finder and fixer, and paid review. Feasibility **7**. Every component exists, and the risk is getting maintainers to write flags at all.

### Turn 6 — Builder
**Closing remark.** Innovation 3, but I'm recording feasibility at 6, not 7. Every component exists. Downstream companies paying for open-source security is the step that broke Tidelift and the Core Infrastructure Initiative, and the dashboard has to prove it can sell before the review lane is funded. Flags as the severity contract is the idea worth keeping even if the marketplace fails. A maintainer who writes down what must never happen has done most of the security work, and it lets a machine turn away curl's slop without a human reading it. The ledger should say "evidence of effort" on every line, exactly as the Visionary put it.
