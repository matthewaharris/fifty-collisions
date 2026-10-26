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
