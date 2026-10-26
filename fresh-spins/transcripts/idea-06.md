# Idea 06 — Cashout
> Seed: 06. **The tip log and the mileage notebook** (consumer app for tipped and gig workers). Servers, bartenders, stylists, delivery and rideshare drivers track cash tips and miles on paper, in notes apps, or not at all. The 2025 tax law's "no tax on tips" deduction (up to $25k a year for 2025–2028, for listed occupations) and the higher 1099 thresholds mean more people must keep their own records, and the 2027 filing season is the deadline. Possible borrowings: Afterlist's evidence tiers (idea 05), #11 Washout (records you can't fool by peeking), #36 Settlement (seal it at the time). Stay clear of tax-advice claims and lending.

### Turn 1 — Visionary
**Working name: Cashout.**

**The deadline.** Tax year 2026 is the first year of no-tax-on-tips without the 2025 transition relief, and new W-2 fields report qualified tips. Schedule 1-A deducts up to $25k a year through 2028. Cash tips, and tips from apps that send no form, are the worker's to prove. With the 1099-K threshold back at $20k and 200 transactions, most part-time drivers get no form at all.

**The spend.** Drivers already pay for the mileage notebook: MileIQ about $60–90 a year, Everlance about $70–100 (blog figures, unverified). **The free substitute I'm beating is Stride** for miles, plus the Notes app for tips. Both are written whenever you remember. Keeper scans bank feeds, and cash tips never touch one.

**The one change: the log closes itself at the end of every shift, and a closed shift stays closed.** Miles track automatically. After the car has sat for 30 minutes, a Live Activity asks "Shift done?" She screenshots the DoorDash earnings screen or snaps a cashout slip. The on-device model (multimodal in iOS 27, free) splits fares from tips, and she adds cash with one tap. The line seals with a timestamp; any correction appears as a visible amendment. In January she gets totals and a shift-by-shift ledger for her preparer. It's a record, never advice.

**Demo:** a crumpled cashout slip, one photo, then "Fri 11:52pm · $142 cash · sealed."

**First buyer:** a multi-app delivery driver who pays for MileIQ, in the January week she learns DoorDash won't send a 1099-K. **Price:** $39.99 a year, hard paywall.

**Borrows:** Afterlist's evidence tiers (photo > same night > later), #36 Settlement (seal it at the time), and Ran It (the event sets the record, not memory).

**What keeps it below 7:** Stride adding a free tips field before January.

### Turn 2 — Builder
**Rules, checked.** Final regulations (TD 10044, April 2026) list 70+ occupations; rideshare drivers and goods delivery people are on it (proposed list; final list not opened). From tax year 2026, W-2s show qualified tips in Box 12 code TP and the occupation in Box 14b (draft instructions); 1099 fields are unverified. FICA still applies. Employees keep a daily tip record and report $20+ a month on Form 4070 (Pub 531); unreported tips go on Form 4137.

**That moves the buyer.** App tips already sit in DoorDash and Uber statements. DoorDash sends a 1099-NEC, not a 1099-K, so her January surprise doesn't happen. The worker with no record is paid in cash or Venmo: the bartender, the booth-renting stylist, the valet.

**What exists:** Stride, free mileage; Gridwise Pro, $79.99/yr, imports platform earnings and tips; MileIQ $59.99–139.99/yr and Everlance $69.99–89.99/yr (sources disagree); Solo and Keeper, prices unverified. Toast asks for cash tips at clock-out; TurboTax and TaxAct handle filing. Incumbents are mileage-first or bank-feed-first, and cash never touches a feed. Mileage is solved and free: drop it as the lead.

**Multimodal:** the iOS 27 claim is vendor-sourced and needs an iPhone 15 Pro or later. Vision text recognition and ML Kit read a slip free on any phone.

**Hardest problem:** it's October. Most of 2026 is already unrecorded, and the deduction ends after 2028.

**MVP:** iOS and Android log, geofence prompt at shift end, sealed entries, monthly Form-4070-style report.

**Economics:** $39.99/yr, $34 after Apple's 15%, under $1 to serve. At Finance CPI ($9.65) and 10.7% hard-paywall conversion, a payer costs about $90, so it needs creators and word of mouth.

**Twist: rebuild 2026, seal 2027.** In January, sell a $29 reconstruction from platform statements, Venmo exports and payslips, every line tier-labelled, credited toward the annual log. Never show estimated savings.

### Turn 3 — Visionary
**Accepted:** the cash-and-Venmo buyer, Vision text recognition and the $29 rebuild of 2026.

**The bartender's moment is the walk to her ride, not the count.** When she leaves the bar's geofence after 9pm, a notification opens a keypad: cash, tip-out, Venmo. Ten seconds, then a wax-seal stamp: "Fri · 1:47am · $212 cash · −$40 to Sam · sealed." **The demo is a genre that already exists.** "What I made in tips tonight" videos are a staple of #bartok and #servertok. We pay those creators (about $150 each) to end on the stamp.

**A monthly trigger beats an annual one.** Employees must report cash tips of $20+ to the employer by the 10th (Form 4070). On the 1st, Cashout drafts it from sealed nights for her manager. The tips then land on her W-2 under code TP: better proof than any app, and twelve deadlines a year instead of one.

**Unexpected: double-entry tip-outs.** Her "−$40 to Sam" sends Sam an invite with that line already in it. Once he accepts, each entry confirms the other. Every tip-out reaches a non-user, like a Calendly link.

**After 2028:** the ledger outlives the deduction. "Friday close averages $212, Tuesday $64" tells her which shifts to swap or keep. It also proves income for a lease application, shared by her, with no lending. Taxes recruit her; the pay map keeps her.

**Channel:** #bartok creators first, then USBG chapters, then salon-suite chains (Sola, Phenix) as a member perk for booth renters. Toast and 7shifts come later, since they are the employer's tools.

**Test (3 weeks, about $2.5k):** ten creators send viewers to a $39 pre-sale, and 30 bartenders get the 2am prompt by SMS. Go: at least 150 pre-orders at a cost under $25 each, and 70% of worked shifts logged in week 3.
