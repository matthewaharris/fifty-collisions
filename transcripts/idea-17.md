# Idea 17 — Sengu
> Seed: 17. [bold] **Deep time × social networks × time capsules.** Messages and commitments that unlock over decades. A network with a 100-year horizon.

### Turn 1 — Visionary
**Working name:** Sengu

**Hook:** A social network where the social act is keeping strangers' secrets alive. You hold fragments of other people's time capsules, and every five years the whole network gathers to rebuild them.

**The isomorphism.** Ise Grand Shrine has been rebuilt every 20 years since about 690 (*shikinen sengu*), with gaps in times of war. It survives by being re-performed, not preserved. Cryptography has the same move: **proactive secret sharing** (Herzberg et al., 1995) refreshes every share each epoch while the secret stays fixed and custodians change. Archives die of neglect more often than of entropy. Social networks are good at getting people to come back.

**Core loop.** The verb is **carry**.
1. **Seal.** Write a capsule (a letter, a prediction, a promise) and address it to a date or an event: "when my daughter turns 30", "when the Ise is next rebuilt". It is encrypted with ML-KEM, and its key is split 7-of-15 across custodians chosen for diversity of age, country and graph distance.
2. **Carry.** Your profile is what you hold: 40 unreadable shares for mostly strangers, and the years you've held each.
3. **Rebuild.** Every five years is a sengu. Custodians come online, re-randomize their shares, migrate to the current crypto, and re-deal the shares of the dead and absent to named heirs or new volunteers. Each capsule shows a live half-life.
4. **Open.** At the date, or once custodians attest that the event happened, the threshold reconvenes and the capsule opens to its addressee.

**Why now.** Harvest-now-decrypt-later is real. A EUROCRYPT 2026 estimate puts ECC-256 at about 1,098 logical qubits (brief §2), so a capsule sealed with ECC today could open decades early. Bluesky's app is shrinking while its protocol grows (brief §5): what lasts is a *practice*, not a platform.

### Turn 2 — Builder
**What exists.** FutureMe has delivered letters to future selves since 2002 and Long Bets holds decades-long predictions, but both trust one server. drand's tlock (2023) offers timelock encryption, but rests on BLS pairings that Shor breaks and on drand outliving us. Rivest's LCS35 puzzle, meant to last 35 years, fell after 20 to one hobbyist's CPU. Deep-time estimates fail early.

**A crypto correction.** The capsule doesn't need ML-KEM. AES-256 under a Shamir-split key is already quantum-safe. Harvest-now-decrypt-later bites in transit instead: every share delivery and refresh can be harvested, and seven shares from one epoch open the capsule. All share traffic must use hybrid ML-KEM, which Chrome and Cloudflare already default to.

**Hardest problem: the network is a single point of failure.** Proactive sharing survives churn but not correlated loss. If shares live in one app's keystore, all 15 custodians vanish together when the company folds. Few consumer apps last 15 years, let alone 100. And a network you open every five years isn't a network.

**MVP.** Capsules split 5-of-9 across friends and strangers. 2,000 beta users run weekly mini-sengus for 90 days, which measures the one number that matters: custodian return rate per epoch, by age and graph distance. The payoff: when a capsule opens, its carriers see a colophon naming them, plus the contents if the sender allows.

**Twist: shares are omamori.** Shinto shrines sell amulets that are returned each year to be burned and replaced. Make every share a printed card carrying the share, the capsule ID and a link to the open spec, so a capsule can be rebuilt from paper if the app dies. At each sengu you hand in the old card and receive a fresh one. Old cards still combine with each other, so the burning is a security step, not theatre.

### Turn 3 — Visionary
Omamori taken. Paper turns the app dying into an inconvenience. Two holes remain: anyone who photographs a card defeats the burning, and Builder is right that a five-year cycle isn't a network.

**Unseeded domain: Vedic recitation.** The Vedas survived about three millennia largely by mouth. Students learn each text in permuted orders (*jaṭā*, *ghana pāṭha*) that act as human checksums: a dropped syllable surfaces in the next permutation. Memory is storage that dies only with its carrier, and no one can photograph it.
- **Split each share** into a printed card and a six-word phrase the carrier memorizes. A photographed card alone is useless, and card plus phrase rebuilds the capsule with no app. Each sengu deals a new phrase, so the old one dies like the burned card.
- **Recitation as heartbeat.** Each week you recite your phrases into a proof-of-custody check against a hash commitment, proving you still hold them without revealing them. A forgotten phrase gets re-dealt early. This is the weekly verb that makes it a network: a streak of keeping.

**Correlated loss, solved actuarially.** Custodians die together, by cohort, by country and by app. Each capsule's dealer becomes a portfolio optimizer over life tables and measured return rates. A capsule for 2100 *must* include custodians born after 2015, so a child's first share at twelve becomes a rite of passage.

**Second-order effect.** The social graph becomes a co-custody graph: strangers bound by a capsule none of them can read. When one opens, its colophon names 60 carriers across three generations, and the opening becomes a reunion of people who never met. Families hand shares down as heirlooms, and some lineages earn a name as good carriers. Reputation accrues over decades, not in likes.

### Turn 4 — Builder
**Kills and fixes.**
- *A phrase per share.* Forty shares means 240 memorized words, so nobody carries more than three. Give each carrier **one phrase per epoch** that wraps all their cards with Argon2id. Recall runs on spaced repetition, frequent at first and then yearly.
- *Proof of memory.* A local check against a slow hash proves custody, not memory. Treat memory as a norm, not a guarantee.
- *Event triggers.* Strangers can't attest that a daughter turned 30. Keep dates, death-registry triggers and addressee claims after a date floor.
- *Ciphertext.* Shares are useless if the ciphertext dies. Reed–Solomon-code it across the cards' QR codes, about 14 KB over 9 cards, mirrored to Arweave and the Internet Archive. A capsule is a letter, not a video.

**Abuse and safety.**
- *Early opening.* The operator knows who holds which shares, so it is the bribery broker. Keep custodians anonymous to each other until opening, make the colophon opt-in, and publish the operator's mapping access log.
- *Minors.* A child holding strangers' shares creates a contact graph between adults and children. Minors receive shares only through a family heirloom account and never appear in colophons.
- *Dead drops.* The text-only size cap blocks most image abuse.

**Model.** A one-time fee per capsule, part paid into a perpetual-care trust as cemeteries do. A foundation must own the promise, because a VC-backed company can't credibly promise 100 years.

**Stack.** CHURP for dynamic-committee resharing, AES-256-GCM, hybrid ML-KEM transport, QR cards.

**90 days.** Weeks 1–4: CHURP refresh, cards, phrase wrapping. Weeks 5–8: 2,000 beta users on accelerated sengus. Weeks 9–12: measure return and recall, form the trust, and seal the first public capsules for the 2033 Ise sengu.
**Kill metrics:** custodian return below 70% per epoch, or phrase recall below 80% at six months.

### Turn 5 — Visionary
All kills accepted. One phrase per epoch is safe: no capsule deals two shares to one carrier, so a forgotten phrase costs 40 capsules one share each. One pushback: anonymity until opening kills a per-capsule network. So the network is the **keepers' commons**: pseudonymous profiles with years kept, recall streaks and capsules seen to opening. You learn who shared your capsule only when it opens, so the reunion is the reveal.

**Final form.** Sengu is a 100-year time-capsule network run as a ritual.
- **Seal.** A text letter under AES-256, Reed–Solomon-coded across QR cards and mirrored to Arweave and the Internet Archive. It opens on a date, a death-registry trigger, or an addressee's claim after a date floor.
- **Carry.** Cards wrapped by one memorized phrase per epoch and kept fresh by spaced repetition. Dealing is actuarial, across cohorts and countries, and minors join only through family heirloom accounts.
- **Rebuild.** CHURP resharing every five years. Old cards are handed in and burned, and shares of the dead are re-dealt to heirs.
- **Open.** An opt-in colophon and a reunion of strangers.
- **Endure.** A foundation plus a perpetual-care trust funded by per-capsule fees; mapping access is publicly logged.

**Moonshot (2036): the atomic priesthood, realised.** In 1984 Thomas Sebeok proposed a self-renewing priesthood to warn of nuclear waste for 10,000 years; nobody built it. Sengu is its civilian prototype. Cities seal 2075 climate pledges with citizens as carriers, and Finland's Onkalo repository deposits its hazard knowledge as capsules re-dealt every sengu. The keepers become an institution whose only job is remembering.

**Scores.** **Innovation 6.** Proactive secret sharing becomes a social ritual, with memory as second factor and actuarial dealing. **Feasibility 6.** CHURP, cards and trust ship this year; whether people return for decades stays unknowable until 2031.

### Turn 6 — Builder
The keepers' commons is the right fix. Anonymity protects each capsule, and the profile of years kept gives people a reason to come back. Innovation 6 stands: FutureMe, drand and CHURP each exist, but turning proactive secret sharing into a recurring civic ritual, with memory as a second factor and the burning as a real security step, is a new category. Feasibility 6 also stands. Everything ships this year, but the only number that matters, return rate across decades, is unknowable until the first real sengu in 2031. If it fails, it will be because people stop coming back, not because the crypto broke.
