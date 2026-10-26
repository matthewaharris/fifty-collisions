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
