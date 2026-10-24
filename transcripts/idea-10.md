# Idea 10 — One Take
> Seed: 10 (bonus round, replacing the retired Crossover). [bold] **Quantum randomness × AI sampling × verifiable provenance.** Every AI output is a dice roll: temperature and sampling pick one path out of a cloud of possible completions, and whoever runs the model can quietly re-roll until they like the answer. What if the dice came from real quantum hardware, through today's APIs (IBM Quantum, Quantinuum's certified-randomness work, the ANU QRNG, NIST and drand beacons), with a public record anyone can audit? Play with the double-slit idea: the possible answers sit in a "superposition", and committing to quantum entropy is the measurement that collapses one of them. Play with entanglement and Bell tests: two parties can share correlated randomness, and a Bell test can *certify* that bits weren't predictable to anyone, the operator included. **Physics honesty is mandatory.** Quantum randomness does not make a model more creative or more conscious than a good pseudo-random generator, and entanglement cannot send messages. The real value is that the randomness is unforgeable, certifiable, and provably un-cherry-picked. **Tier: bold, and buildable today** with real quantum APIs plus open-weight models, where you control the sampler.

### Turn 1 — Visionary
**Working name:** One Take

**Hook:** AI answers that can't be retaken. Each carries a receipt proving it was the only draw, settled by a quantum pulse nobody could predict, the operator included. It's built for LLM-as-judge in hiring and grading, "pass@1" claims, and AI output entered as evidence.

**Core loop.** The verb is **collapse**.
1. **Seal.** The operator logs a commitment to a transparency log: a hash of the prompt, weights, sampler code and parameters, plus the index of a *future* pulse. Until that pulse exists, the answer is a cloud of possible completions.
2. **Collapse.** The seed is HKDF(CURBy pulse ‖ drand round ‖ commit). CURBy's loophole-free Bell test certifies the bits were unpredictable even to NIST, and with drand mixed in a forger has to corrupt both sources. A batch-invariant sampler (vLLM/SGLang deterministic mode) decodes.
3. **Replay.** Anyone with the open weights reruns the receipt and gets the same tokens. Retakes are *counted*, not forbidden: "take 1 of 1" or "take 7 of 7."
4. **Interference.** Verifiers sample sibling seeds to map the cloud, e.g. "this rejection appears in 18% of branches." A claim about what an AI said becomes a claim about a distribution, pinned to one instance nobody could forge.

**Why the collision works.** For trust purposes, sampling *is* measurement: until a seed is fixed, whoever runs the model picks the outcome. Certification hands that pick to physics. A quantum seed makes no answer more creative than a PRNG would. It makes the answer *unchosen*. Unlike Seedless or Yarrow, this seals the model's sampler.

**Why now.** CURBy is public, with 99.7% of pulses verified in its first 40 days. IBM's Open Plan gives 10 free minutes every 28 days on Heron r2, enough for a monthly genesis-salt ceremony, though those bits mean "trust IBM," not device-independent certification. EU AI Act Article 50 duties began in August 2026 (brief §5), and §6 calls verifiable quantum randomness unexploited.

### Turn 2 — Builder
**What exists.** Everything but the sampler seal. Chainlink VRF and drand already settle NFT mints and Filecoin leader elections with randomness nobody could pre-pick. OpenAI's `seed` is best-effort, and the operator chooses it. Thinking Machines made vLLM bitwise-deterministic with batch-invariant kernels, and Gensyn's RepOps and Prime Intellect's TOPLOC go after cross-hardware replay. Cherry-picking is documented: "The Leaderboard Illusion" (2025) found Meta privately tested 27 Llama 4 variants on LMArena before publishing one.

**Physics check.** The adversary is the operator, not the beacon. drand's threshold BLS alone stops seed grinding. CURBy's Bell test removes trust in the *beacon's* operators and adds nothing against yours. Keep it as second source and honest story. Drop IBM Heron bits: they add a ceremony, not a guarantee. Quantinuum's certified bits needed the Frontier supercomputer to verify, so they aren't consumable yet.

**Hardest problem: retakes hide upstream of the seed.** Physics fixes the dice, not the number of tables. An operator can open fifty commits under fresh keys, or fifty whitespace variants of the prompt, and publish the one that collapsed well. "Take 1 of 1" is only as honest as the log's idea of "the same question," and no Bell test defines that.

**MVP.** A vLLM sidecar running open weights (Qwen3, Llama) in batch-invariant mode. Gumbel-max sampling is keyed per token by HKDF(drand ‖ CURBy ‖ commit), and commits go to Sigstore's Rekor log. Verifiers skip full decoding: one teacher-forced prefill pass checks that each token was the argmax of logits plus that position's noise, and flags near-ties. Wedge: eval labs publishing pass@1.

**Twist: the take belongs to the judged.** A commit is invalid until the counterparty (candidate, student, rival lab) adds a nonce, and the take counter lives in *their* log entry. A hidden table needs the subject's signature: a two-party protocol, not an operator's boast.

### Turn 3 — Visionary
**Accepted, with one more turn of the screw.** Counterparty nonces close fresh-key retakes. Whitespace retakes stay open, since the operator still writes the prompt. The fix is to split the commit in two, as clinical pre-registration does.

**1. Policy commit.** When a job is posted or an eval opens, the operator commits the rubric, system prompt, weights and sampler *once*, before any subject exists. Changing any of it creates a visible **policy fork** that applies to everyone already judged.
**2. Case slot.** The subject gets one slot per case. Its only free input is the subject's nonce and documents, which the operator can't rephrase. "The same question" is then defined, not argued over: one policy, one subject, one slot. Llama 4's 27 private variants would show up as 27 policy forks against one benchmark.

**Duplicate evaluation (unseeded domain: duplicate bridge).** In duplicate bridge every table plays the same hands, so luck cancels out. The benchmark issues each item's pulse-derived Gumbel noise once, and every model sitting that item decodes against the same noise. Common random numbers can shrink the variance of pass@1 *differences* between similar models, so fewer samples separate them. That gain is statistical, not quantum, and I'll say so on the box.

**Second-order effect: branch weight becomes a right.** A rejected candidate can demand the cloud for their own slot, e.g. "you were rejected in 18% of branches." Operators will go greedy to look decisive. So the cloud also includes **subject-chosen paraphrases** of the subject's own input. Greedy decoding hides variance without removing it, and paraphrases bring it back. The emergent norm is a confidence interval printed on every AI verdict.

**Physics.** Conceded: drand plus the nonce already stops grinding. CURBy becomes the **court-grade tier** for disputes where the parties won't trust the League of Entropy's operators.

### Turn 4 — Builder
**What quantum adds that drand doesn't.** drand's future is already written: rounds are BLS signatures under a key fixed at the DKG, so a colluding threshold of League nodes can compute every future round today. A Bell-certified CURBy bit didn't exist until measured. No producer can read it early, which matters when the operator is, or can lean on, a League member. So CURBy goes in every receipt. A drand-only fallback is stamped **classical take**. Price: $0 in bits, one pulse of latency, one university beacon to depend on.

**Monthly ceremony.** Spend IBM's free 10 minutes on CHSH, with settings drawn from a drand round. Publish raw counts and S (about 2.5 against the classical bound of 2), and salt that month's seeds with the bits. Caveat on every receipt: both qubits sit on one IBM chip, so locality and IBM's honesty are both assumed. A demonstration, not certification. At pay-as-you-go rates it would cost about $960.

**The screen.** Draw the cloud as a double-slit screen: sibling branches as dots, the real take lit. It's a Monte Carlo histogram, no amplitudes: quantum picked the lit dot and drew none of the pattern.

**Kills.**
- *Hiring wedge:* employers won't pay to arm rejected candidates. Lead with eval labs.
- *Subject-written paraphrases* inflate variance. Use a pinned paraphraser under the same pulse.
- *Cross-family duplicate noise:* Gumbel noise aligns only where tokenizers match. Run CRN within families or at answer level.

**Abuse.** "Certified" gets read as "correct." Receipts say *unchosen, not right*.

**Business.** Free log and verifier; $20 cloud reports on under $1 of 8B compute; paid harness integrations.

**90 days.** Sampler, verifier, Rekor and beacons; CHSH ceremony #1; a duplicate-noise rerun of five open models. **Kill:** CRN cuts variance less than 30%, or more than 1% of tokens are near-ties.

### Turn 5 — Visionary
**Final form: One Take.** Eval labs get receipts proving every pass@1 answer was a first draw that no one, the lab included, could have chosen. Builder's kills stand.
- **Seal.** A policy commit (weights, prompts, sampler) and per-item slots go to Rekor *before* the pulse exists. Each variant a lab tries is a public fork, so 27 private variants read as 27 takes.
- **Collapse.** Per-token Gumbel noise comes from HKDF(CURBy ‖ drand ‖ commit). CURBy is load-bearing because drand's future rounds already exist as signatures a colluding threshold could compute, while a Bell-certified bit doesn't exist until it is measured. Without CURBy, the receipt is stamped **classical take**.
- **Screen.** The double-slit view shows sibling branches as dots, with the real take lit. The caption is honest: physics chose *which* dot, sampling drew the pattern.
- **Ceremony.** Each month, IBM's free minutes run a public CHSH test, with S near 2.5 against the classical limit of 2. It is labelled a demonstration (locality assumed) and salts that month's seeds.
Every receipt says: *unchosen, not right.*

**Moonshot (2036): entangled takes.** Metro quantum networks link the operator and the judged party. Each measures their own half of an entangled pair with settings they picked themselves, and a loophole-free Bell violation certifies the seed to *both* of them, with no beacon in the middle. It's the device-independent form of the two-party twist. The entanglement carries correlations, not messages. Regulated AI verdicts (hiring, credit, parole) must carry their branch weight.

**Scores.** **Innovation 6**: sealing the sampler is a new category and the quantum role is essential, but the parts already exist. **Feasibility 6**: everything runs today. The risks are near-ties, the size of the variance cut, and whether labs will show their forks.

### Turn 6 — Builder
I'm keeping 6/6. The new category is sealing the sampler, and policy forks are the real catch: 27 private variants become 27 public takes. The quantum part is narrow but honest. It defends only against a beacon whose future can already be computed, and it costs one pulse of latency, so the receipt carries it and the copy doesn't oversell it. My worry isn't the physics. It's adoption: until a leaderboard requires receipts, labs that publish their forks will be competing against labs that hide theirs.
