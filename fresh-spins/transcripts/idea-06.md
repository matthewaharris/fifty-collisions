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

### Turn 4 — Builder
**Cuts.**
- **Geofence: out of v1.** It needs "Always" location, which App Review wants justified, iOS keeps re-asking about, and Google Play reviews. Exits fire late and miss inside malls. Battery is fine; trust isn't. Prompt at her usual close time instead.
- **Sam's invite:** our text puts his income on his lock screen before he consents. She sends an amount-free link from her own Messages; he sees the line after accepting.
- **Form 4070:** a PDF for *her* to hand over. No employer accounts, no sending. It mustn't contradict her Toast clock-out declarations.
- **Lease proof: cut.** Vouching for income to landlords starts to look like a consumer report under FCRA (my reading, unverified) and invites inflated logs. She can export her own PDF marked "self-reported."

**CAC under $25 is optimistic.** A $150 video at 20k views, 0.5% clicks and 5% conversion yields 5 orders: $30 each. Plan on $35–45 against $38 net revenue (web) or $34 (Apple). Year one breaks even; renewals and the rebuild make the profit.

**Copycats:** Gridwise or Stride can add a tips field in a month; only sealed history and tip-out links make leaving costly.

**12 months:** $39.99/yr annual only, web plus in-app purchase; Rebuild 2026 at $29, credited. First 100 from the pre-sale; 1,500 by April through creators ($40k), USBG chapters and Sola/Phenix perks; about 2,500 payers (about $90k ARR) by month 12. That's below 7.

**Safety:** no deduction maths, no "you qualify," no bank feeds.

**90 days:** weeks 1–3, test. Weeks 4–8, log, seal, close-time prompt, 4070 PDF, tip-outs. Weeks 6–12, rebuild importers (Venmo, Cash App, DoorDash, Uber, payslips) live by February 1.

**Kill:** under 100 pre-orders, CAC over $45 or under 60% of shifts logged in the test; by April 30, under 1,000 payers or under 30% rebuild-to-annual.

### Turn 5 — Visionary
**Accepted, all of it:** close-time prompt, amount-free tip-out links, a 4070 PDF she hands over, no lease proof.

**Final form.** Cashout is a $39.99-a-year sealed tip log for workers paid in cash and Venmo: bartenders, servers, booth-renting stylists and valets. At her usual close time a keypad asks for cash, tip-out and Venmo; ten seconds later the night is sealed, and corrections show as amendments. An accepted tip-out link lets a co-worker's entry confirm hers. She gets a 4070-style PDF on the 1st and an evidence-tiered ledger in January. Rebuild 2026 ($29, credited) covers earlier months. It never does deduction maths and never says "you qualify."

**Boring on the surface,** it's the tip log in Publication 531. **But** every night is sealed before she sleeps, and every tip-out has a second witness.

**3-year version:**
- **The pay map:** "Friday close $212, Tuesday $64": which shifts to swap or keep, and the reason to renew if the deduction lapses.
- **The house sheet:** small bars without Toast run tip pools on paper. A $19/month house sheet, with each line confirmed by staff, is a second, SMB buyer.
- **Target:** 15–20k worker payers and 500 bars, about $0.7–0.9M ARR.

**Scores.**
- **Innovation 4:** tip logs are commodity. What's new is sealing at close plus double-entry tip-outs between co-workers.
- **Feasibility 8:** text entry, OCR and a PDF on known stacks, with no bank feeds and no advice.
- **Market fit 6, accepted.** The deadline is real and monthly. But the cash-tipped buyer has no proven spend on tip logs, CAC is roughly a year's revenue, and the window closes in 2028. A salon-suite chain paying per member, or test CAC under $25, would earn it; neither is shown.

### Turn 6 — Builder
Scores: 4/8/5. Market fit drops to 5, not 6: the buyer we chose keeps her tip log free on paper or in Toast, and the people who do pay for logs (MileIQ, Gridwise) are the drivers we dropped. Build cost is low, the deadline is real and monthly, and the double-entry tip-out is a real idea. But a year's revenue per customer in CAC and a deduction that ends after 2028 make this a careful $90k-ARR test until a salon-suite chain or a sub-$25 CAC shows up in the data.
