# Idea 02 — Posted
> Seed: 02. **The construction notice taped to the lobby door** (SMB software or service for small contractors and property managers). The paper notice and the door hanger that warn neighbours about noise, water shut-offs and parking. Possible borrowings: #35 Lookahead (a noise forecast the contractor commits to and is held to), #14 Second Shift (who's on site and when). Who pays, and why they'd pay this year, matters more than the tech.

### Turn 1 — Visionary
**Working name: Posted.** The old product is the paper door hanger and lobby notice: "Water off Tuesday 10–2," "Pest control will enter your unit." It's stale on both sides. Residents miss it or claim they never saw it. The manager has no proof it went up, and nobody learns that the water came back at 3:40, not 2:00.

**Who pays this year, and why:** multifamily property managers, because the paper notice is a legal act. California Civil Code §1954 requires written notice of entry with date, time and purpose, and "on, near, or under the usual entry door" counts as delivery. When it's disputed, the landlord has to prove it. In a 2024 New York decision (nycourts.gov), a landlord's photos were admitted but their GPS and timestamps were disregarded because a third-party server couldn't authenticate them. Los Angeles reissued its 24-hour-notice flyer under the Tenant Anti-Harassment Ordinance in January 2025. Add failed entries and shut-off complaint calls.

**The one change:** the hanger stays paper, but every one carries a unique per-door code. The tech snaps it on the door, and that photo's hash is sealed to a public timestamp at posting. That gives proof of service per door, per minute. A resident who scans gets live updates ("water back on") with no app.

**Borrows:** from #35 Lookahead, the notice is a forecast the vendor commits to. Posted logs whether the window was kept and grades every contractor on it. From #36 Settlement, seal it before anyone knows it'll be disputed.

**Recurring revenue:** about $0.40 per unit per month, with the coded hanger stock as a consumable refill. First buyer: a California manager with 500–3,000 units, the week after a tenant disputes an entry. Channels: the California Apartment Association and the AppFolio and Buildium marketplaces. Every hanger advertises itself.

### Turn 2 — Builder
**Legal hooks.** §1954's "on, near, or under the usual entry door" language is real, and §1940.2 lets tenants recover up to $2,000 for intentional entry violations (verify). The NY case checks out (2024 NY Slip Op 50219(U)): photos admitted, GPS and timestamps thrown out as unauthenticated third-party data. It was an eviction posting, not an entry notice. I couldn't verify the LA flyer.

**What exists:**
- **AppFolio and Buildium:** templates plus mass email and text; no per-door proof.
- **RentCafe (Yardi) and Entrata resident apps:** portal announcements.
- **ButterflyMX:** photos and logs at the building door, not the unit.
- **SmartRent:** unit lock logs prove entry, not notice.
- **Latchel:** schedules vendor entry with residents.
- **Proof (proofserve.com):** already sells managers posted notices with timestamped, geotagged photos and 72-hour affidavits (pricing unverified).

Missing: cheap self-serve proof made by on-site staff.

**Would they pay $0.40/unit/month for entry notices?** Doubtful. Entry disputes are rare; a 1,000-unit manager won't pay $4,800/yr against one. The money is in eviction notices. California 3-day notices served by post-and-mail (CCP §1162) fail on bad service, and a re-serve costs weeks of rent.

**Hardest problem:** techs actually photographing every door, and pulling notices from AppFolio (gated partner API).

**Twist:** charge per legal notice and give the rest away. Entry and shut-off notices are free; they build the habit and the resident page. 3-day, 30/60-day and lease-violation notices cost $6 each with a court-ready proof bundle (photo, door code in frame, RFC 3161 timestamp, declaration draft) that staff produce themselves.

**MVP (6 weeks, one dev):** a PWA with live-camera-only capture, QR stickers on existing hangers, a hash timestamp and a PDF proof bundle.

**Economics:** about $0.01 per notice to serve, so roughly 95% gross margin; 1,000 units at 30 legal notices a month (assumed) is about $2,200/yr.

### Turn 3 — Visionary
**Accepted:** charge $6 per legal notice and make entry and shut-off notices free. **Changed:** the first buyer is affordable housing, because HUD's 30-day nonpayment rule (89 FR 101270, Dec 2024) applies now. It requires written notice 30 days before filing, with itemized amounts and cure and hardship information. Owners: since January 13, 2025; PHAs: since June 15, 2026. Telling residents their rights is already the law there.

**The resident copy is the product.** Every coded notice opens to the same sealed photo and timestamp the manager holds, plus a plain-language page: amount, deadline, how to cure, local legal aid. Nobody can backdate it, so sewer service is structurally hard. The resident's scan logs "opened 9:40pm," disclosed on the page. Actual notice beats any affidavit about service mechanics, so honesty is the manager's strongest evidence. Housing courts and legal aid get a free public verify link on every notice.

**Photo adoption: make the camera the act of posting.** In walk mode, staff sweep the phone down a hallway, and each door's code and number auto-capture as it ticks green, about 2 seconds a door. That's also the 5-second demo. Only legal notices need a door photo; bulk shut-offs need one lobby shot.

**AppFolio gap: skip the API.** Staff already export notices as PDFs. They email the PDF to Posted, and it comes back stamped with per-door codes, ready to print. Same for Yardi Voyager. APIs wait for revenue.

**Demand test (3 weeks):** concierge pilots with 3 HUD-assisted operators found through NAHMA and state affordable-housing associations. We stamp and verify every 30-day notice for a month, free. Plus $500 of search ads on "HUD 30-day notice rule." **Go** if 2 of 3 use it on 80%+ of legal notices, 1 signs at $6 per notice, and 15+ demo requests come in.

### Turn 4 — Builder
**The HUD rule is live but fragile.** HUD rescinded it by interim rule on Feb 26, 2026, then, sued over the skipped comment period, delayed that on Mar 13 pending a final rule. The 30-day notice applies for now; rescission could come any month (post-March status unverified). And it mandates notice, not proof of posting. Door-opener only; California 3-day notices stay the second market.

**Market size:** about 5M HUD-subsidized units (2018); roughly 1.25M project-based Section 8 plus 1M public housing fall under the rule, across about 3,300 PHAs and thousands of owners (unverified).

**Resident-open logging: cut.** It tracks tenants, and any passer-by's scan counts as an "open", so it proves nothing. Keep an optional resident-pressed "I got it / I need help" button routing to legal aid or rental assistance. The QR code on the door shows only notice type, door and time. Amounts stay inside the sealed notice; neighbours never see debts.

**Email-the-PDF:** a mis-parsed unit number puts proof on the wrong door, which is fatal. Release each notice only when the photographed door number matches the printed unit. Setup per customer.

**Walk mode:** 2 seconds a door is a demo; plan on 10. Offline in stairwells: hash on capture, seal on sync, log the gap.

**12 months:** $6 per notice. Assuming 5 notices per 100 units a month, a 1,000-unit operator pays about $3.6k/yr. Founder-led sales via NAHMA, NAHRO and state associations; CAC about $2k (assumed). Target: 25 operators and about $90k ARR. E&O insurance; declarations factual, never legal advice. Copycats: Proof or Yardi can add this, so speed and accepted-in-court bundles are the only moat.

**90 days:** weeks 1–3, concierge pilots. Weeks 4–9, PWA and email intake. Weeks 10–13, 3 paying sites.

**Kill:** by month 6, under 8 paying operators or under 400 paid notices a month.

### Turn 5 — Visionary
**Accepted:** open-logging cut for an "I got it / I need help" button; debts off the door; release only on a door-number match; 10 seconds a door.

**Final form, built not to need HUD.** Posted is self-serve proof of service for any notice a state lets you post on a door. Staff email the PDF, print coded notices and post them in walk mode. Each posting is sealed to an RFC 3161 timestamp in a court-ready bundle. The resident gets the same record plus a plain-language page on deadlines, cure and legal aid. Pricing: $6 per legal notice, with entry and shut-off notices free.

Two markets, sold in parallel from month one:
1. **California 3-day, 30/60/90-day and lease-violation notices** (CCP §1162 post-and-mail), replacing a process server's fee (unverified) or an unprovable staff posting.
2. **HUD-assisted operators** while the 30-day rule stands. If it's rescinded, those operators still post state notices.

Then states with similar posting statutes, by eviction volume.

**Boring on the surface,** it's a door hanger with a QR code. **But** every posting is sealed, so neither the landlord nor the tenant can fake what happened at the door.

**3-year version:** the default proof layer for posted notices in 4–5 states. An AppFolio and Yardi partner once volume earns the API, with the verify link accepted by a few court self-help centres, adding HOA and code-enforcement notices. Target: $1–2M ARR, with free operational notices as the funnel.

**Scores.** Innovation **4**: sealed photos exist (Proof). What's new is the resident's identical copy, door-matched release and staff self-serve. Feasibility **7**: simple build; setup and court acceptance unproven. Market fit **5**: a buyer with real legal pain, but about $90k ARR from founder-led sales against $2k CAC is plausible, not proven. It reaches 6 if two concierge pilots convert.

### Turn 6 — Builder
Scores as proposed: 4/7/5. The real asset is the California post-and-mail market, where bad service already costs landlords weeks of rent. The HUD rule is a sales hook that could disappear in any Federal Register issue. What decides it isn't the software. It's whether two concierge pilots keep posting through month two, and whether the first contested bundle holds up in court. Run the pilots before writing much code.
