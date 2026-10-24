# Idea 31 — Tombstone
> Seed: 31. [mild] **End-of-life × digital legacy × agents.** An executor agent that winds down a digital life gracefully.

### Turn 1 — Visionary
**Working name:** Tombstone

**Hook:** Decommission a digital life the way engineers retire a server: cordon, drain, tombstone, in an order worked out while you're alive.

**The second idea.** Closing a dead person's accounts is the easy part. What actually goes wrong is ordering and resurrection. Families cancel the phone line in week two, and every SMS recovery code dies with it. Months later LinkedIn suggests congratulating the deceased on a work anniversary. Distributed databases solved both: drain before deleting, then write a **tombstone**, a deletion marker that stops any copy resurrecting the record.

**Core loop. The verb is drain.**
1. **Map (while alive).** From receipts, statements and a password-manager export, the agent builds a **recovery graph** of which account recovers which. The phone number and primary email are roots, so they close last.
2. **Annotate.** For each account, the owner picks keep, transfer, memorialize or delete, and names an heir. The agent asks again yearly, like a will review.
3. **Cordon (on verified death).** Revoke agent mandates and keys, block new charges.
4. **Drain.** Work from the leaves inward. Move photos, domains and shared docs before their owning account closes. Cancel subscriptions, claim prorated refunds, settle open agent tabs.
5. **Tombstone.** Send a signed death notice to data brokers and marketing lists so the record stays dead.

**Why it fits.** Under RUFADAA, a provider's own online tool (Google Inactive Account Manager, Apple Legacy Contact) outranks the will. So the agent configures those tools to match the graph first. The FTC's click-to-cancel rule was vacated in 2025, so cancelling is still a maze, and mazes are agent work.

**Why now.** People now die holding live AP2 mandates, with agents "tethered to human owners" (§1). An agent whose owner has died is a zombie with a credit card.

### Turn 2 — Builder
**What exists.** Post-death admin is crowded. Empathy runs a bereavement concierge that closes accounts and sells it through life insurers such as MetLife. In the UK, Life Ledger and Settld notify hundreds of firms from one form, and Tell Us Once covers government. GoodTrust and Trustworthy store the plan. Even the tombstone half exists: the ANA's Deceased Do Not Contact list and the UK Bereavement Register. Nobody models the **recovery graph**. That part is new.

**Hardest problem: authority.** The day after a death, the agent has no legal standing. Logging in with the dead person's password breaks almost every provider's terms and arguably the CFAA. Without the owner's consent, RUFADAA gives a fiduciary the account's catalogue, not its contents. Letters testamentary take weeks or months. Meanwhile a vault mapping every account to its recovery root is a gift to identity thieves, and "ghosting" the dead is already routine fraud.

**Twist: drain while alive, and rehearse the death.** Most of the drain can happen before anyone dies. Move SMS 2FA to passkeys and a recovery email the heir can reach, add Legacy Contacts, and put shared photos and domains into joint ownership. Then borrow SRE's **game day**. Once a year the agent simulates the death: the heir tries to reach the ten root accounts using only what the plan gives them, and the agent scores the life's **bus factor**. After a real death the agent never logs in as the deceased. It acts for the executor through each provider's official bereavement channel.

**MVP.** An app reads a Bitwarden or 1Password export (account names only, no secrets) and scans one mailbox for receipts. It builds the recovery graph, configures the Google, Apple and Meta legacy settings, and runs one drill.
**Kill metric:** fewer than 30% of users finish a drill.

### Turn 3 — Visionary
**Taking the game day, and making it the source of authority.** RUFADAA gives a fiduciary the *contents* of an account only if the user consented, through the provider's tool or a will. So every drill ends by signing a dated **consent schedule**: one line per account naming the heir, the scope (contents, catalogue, or delete unread) and the disposition. The will incorporates the latest schedule by reference. At death, the executor reaches each bereavement channel with consent already on paper. The drill stops being homework and becomes how the paperwork gets made.

**The honeypot, solved with social-recovery wallets (unseeded).** Argent-style Ethereum wallets already solved "let my people in without one stealable key." The map holds topology only and is encrypted on the owner's devices. Heirs are **guardians**, each holding a Shamir share, and any two can reconstruct it. Reconstruction starts a 14-day timelock that pings every device the owner has, and any tap from the owner vetoes it. A thief has to compromise two relatives and also outlast a living owner. A dead owner never vetoes, and that silence is the death proof. That replaces death verification.

**Emergent behaviour: deaths cascade.** Run drills across a household and the graphs interlock. Dad's email recovers Mum's bank, and both route through the daughter's phone plan. The real metric is the **household bus factor**: whose death locks out the most of the family. That person drains first.

**Second-order effect: a bereavement scorecard.** Each drill and each real drain logs days, documents and failures per provider. Pooled and anonymized, that data ranks companies by how hard they make it to bury someone, the way EFF's "Who Has Your Back" ranked privacy. Laggards get pressure to ship Legacy Contacts, so the tombstone improves the industry's bereavement channels.

### Turn 4 — Builder
**Kills and fixes.**
- *Silence as death proof.* Silence also means a coma, dementia, a stroke ward or a lost phone. The timelock stops thieves; it doesn't prove death. Silence triggers **incapacity mode** only: cordon, freeze new mandates, and hand off to whoever holds the durable power of attorney. Deletion and transfer need a death certificate. With dementia, incapacity is the bigger product.
- *Incorporation by reference.* It only reaches documents that existed when the will was signed, so a yearly schedule would need a yearly codicil. Skip the will: RUFADAA accepts consent in "a will, trust, power of attorney, or other record." The signed schedule is that record.
- *Heirs as the only guardians.* Elder financial abuse comes mostly from family. Use 2-of-3 instead, with the third share held by Tombstone or the family's lawyer and released only against a verified certificate.

**Household graph.** It can expose a spouse's hidden accounts, which matters most in abusive homes. Each person sees only the edges that cross into their own accounts.

**Money.** People won't pay up front for death prep; most US adults have no will. Sell through estate-planning platforms (Trust & Will, Wealth.com) and life insurers, who want beneficiaries engaged before a claim. About $60 a year per household, or a per-policy fee.

**Stack.** Takeout and IMAP parsed on the device, since a hosted Gmail scan triggers Google's restricted-scope audit. libsodium Shamir; ESIGN signatures. Legacy settings use guided deep links, not a bot driving the UI. Hard rule: no griefbot built from the data.

**90 days.** Weeks 1–4: graph and drill. Weeks 5–7: guardians, timelock, counsel review in three states. Weeks 8–10: 200 households via one estate-planning partner. Weeks 11–12: executor mode for 20 providers, plus scorecard v0.
**Kill metrics:** under 30% finish a drill, or partner conversion under 5%.

### Turn 5 — Visionary
**Final form.** Tombstone decommissions a household's digital life, sold through estate planners and life insurers. While you live, it maps the **recovery graph** on-device and moves you off fragile roots: SMS 2FA becomes passkeys, Legacy Contacts get set, shared photos and domains go into joint ownership. A yearly **game day** has an heir try to reach the root accounts and scores the **household bus factor**. Each drill signs a consent schedule, a RUFADAA "other record" naming the heir, scope and disposition for every account. The map is split 2-of-3: two heirs, plus Tombstone or counsel, whose share is released only against a certificate. Silence starts only **incapacity mode**: cordon, freeze, hand off to the power-of-attorney holder. Only a death certificate starts the **drain**, which runs from the leaves inward through official bereavement channels and ends with a tombstone notice to brokers. I accept every kill; incapacity matters most, since the graph serves the dementia decade before the funeral.

**Moonshot (2036).** "Principal status" becomes a protocol field. Electronic death and incapacity registrations publish signed events, and providers subscribe to them the way they subscribe to card-revocation lists. Providers read machine-readable consent schedules and drain their own nodes in days, not months. Every AP2 mandate and agent identity carries a liveness reference, so a dead principal's agents cordon themselves within the hour. The scorecard grows into regulation: a "right to be buried" alongside the right to be forgotten.

**Scores.** Innovation **3**. SRE drain semantics, social-recovery cryptography and RUFADAA consent records all exist. What's new is the recovery graph and the drill that produces legal authority as a by-product. That fits a mild seed. Feasibility **6**. The software is simple. The pace is set by state legal review, partner distribution, and getting people to rehearse their own death yearly.

### Turn 6 — Builder
Innovation 3 and feasibility 6 stand. Each part exists somewhere: bereavement concierges, social-recovery wallets, RUFADAA consent. The real invention is the recovery graph, plus a drill that turns a morbid chore into the paperwork that gives the executor authority. The biggest risk isn't technical. It's whether anyone rehearses their own death every year, which is why incapacity mode and distribution through insurers carry the business.
