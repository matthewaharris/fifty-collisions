# Idea 05 — Afterlist
> Seed: 05. **The shoebox of receipts, warranty cards and owner's manuals** (personal or household admin; app, service or hybrid). Every home has a drawer of paper that matters exactly once: a warranty claim, a recall, an insurance claim after a burglary, a resale. Possible borrowings: #31 Tombstone (map what recovers what), #22 Unaccounted (product passports scanned from the object itself; the EU Digital Product Passport rollout), #36 Settlement (seal what you expected, check it later).

### Turn 1 — Visionary
**Working name: Afterlist.** The old product is the shoebox of receipts and the home-inventory app. Nobody keeps either current, and the shoebox burns with the house. Receipt apps died because they asked for daily effort against a maybe-someday payoff.

**The one change:** stop asking people to keep a shoebox and rebuild it after the loss from data that already exists. That means Amazon's "request my data" export, Target, Costco and Apple order histories, a Google Takeout mailbox of e-receipts, card statements, and old phone photos where AI spots the sofa in a 2022 birthday picture. Afterlist returns the insurer's contents spreadsheet: item, room, age, price paid, replacement link, and evidence on every line. Export uploads only at launch, no inbox OAuth.

**First buyer and moment:** a total-loss policyholder, the week the insurer asks for the list. The money is visible. California's SB 495 (signed October 10, 2025, effective 2026) makes insurers advance 60% of contents limits, capped at $350k, with no inventory for 100 days. The other 40% needs the list. On a $300k contents limit, that's $120k waiting on a spreadsheet that takes hundreds of hours. Second buyer: public adjusters, whose bottleneck is the inventory.

**Why it won't die like receipt apps:** no habit before the loss, one payment at the moment of maximum value, and the output is accepted currency (the insurer's own template). Competitors found: Bevel (free, photo-to-list, built after the Palisades fire) and ProofList (paid, photos). Neither imports order histories or statements (unverified).

**Borrows:** #22 Unaccounted's mass balance. Statements show $4,200 at REI from 2019 to 2024, and unmatched spend flags the camping gear memory forgot. #31 Tombstone maps which source recovers which room.

**Price:** $399 flat per household, never a percentage, to stay clear of public-adjuster licensing. Adjusters pay $249 per file.

### Turn 2 — Builder
**SB 495 checks out:** Chapter 542, effective Jan 1, 2026. In a declared emergency, total-loss policyholders get 60% of contents limits (capped at $350k) with no itemised list for at least 100 days. It cuts both ways: by March 2025, most LA-fire insurers had agreed to pay 75%+ of contents limits without a list, some 100% (CDI). The list matters less in big declared disasters, and fully in single-home fires and burglaries, which get no waiver.

**What exists:**
- **Bevelmade:** free.
- **United Policyholders:** free guides and app.
- **ProofList:** pricing unverified.
- **Encircle:** B2B field documentation for contractors and adjusters.
- **Sortly:** business inventory.
- **Contents specialists working for public adjusters:** fees unverified.

None I found import order histories (unverified).

**Volume (unverified estimate):** probably tens of thousands of total-loss contents claims a year. Steady house fires, plus lumpy disaster spikes (LA 2025 destroyed about 16k structures). Consumer demand comes in bursts.

**Fraud is the hardest problem.** AI that "spots" and prices a sofa is a padding machine, and adjusters will discount the whole list. Evidence tiers per line:
- **A:** order record, exact price and date.
- **B:** statement line, item user-attested.
- **C:** photo only, price labelled as an estimate.
- **D:** memory, attested.

No line exists until the user confirms it. Mass-balance gaps only prompt memory. Every line shows its tier, which makes the list more credible to the adjuster.

**Survivors:** no burn-zone geo-targeting for 30 days, payment only once the export is ready, and a refund if the insurer waives the list.

**Twist: public adjusters first.** They hold the steady, unwaived losses, run dozens of files a year, and inventory is their bottleneck. Charge $199/file and reach consumers through them.

**Economics:** about $30–80/file in vision and parsing (unverified), so 60–85% margin.

### Turn 3 — Visionary
**Accepted:** adjusters first at $199 per file, evidence tiers, user-confirmed lines, refund if waived. Big disasters waive the list; the steady market is the unwaived loss.

**The tiers are the demo.** Drag in an Amazon export and a card-statement CSV. Lines pour in and sort themselves by colour, and a header totals them: **A $61,380 (214 items, order numbers) · B $23,100 · C $9,400 (estimate) · D $4,200.** Tap any green line and the original order record opens. The adjuster's five-second takeaway is that two-thirds of the claim arrives already proven. The photo that sells it: one line, the sofa with its 2022 order number and its 2022 birthday picture side by side.

**Steadier buyer: theft, not fire.** Burglary and theft claims come every week, carry no waiver, and are mostly electronics, bikes and tools, the items order records prove best. Adjusters rarely take a $12k theft file, so this is a direct consumer lane: a $99 self-serve tier (A and B only, no photo AI) sold on constant search intent ("list of stolen items for insurance," "renters theft claim"). Estates need fair market value for probate, a different job, so park them.

**Demand test (4 weeks, about $1.5k):**
- **Adjusters:** pull 60 public adjusters from California, Florida and Texas licence rolls and NAPIA. Offer each a free 72-hour concierge rebuild of one live file, done by hand plus LLM parsing.
- **Measure:** hours saved against their usual process, the share of value landing in tiers A and B, and whether the carrier's desk adjuster accepts tier-A lines without asking for receipts.
- **Go:** 5+ adjusters send a second file at $199, and 50%+ of the value lands in A/B.
- **Consumer theft lane:** $500 of search ads to a $99 pre-order page. Go at 2%+ conversion and CAC under $40.

### Turn 4 — Builder
**Theft lane: cut for year one.** The average theft claim is about $5,500 (III 2019–23, via secondary sources). After a $500–1,000 deductible that's a 15–30-item list typed in an evening, and many renters have no policy or skip small claims. $99 is a hard sell. If ever tested: $29, CAC under $15.

**Licensing is a real consumer-side risk.** Florida's public-adjuster definition covers anyone paid to "prepare, complete, or file" a claim form (§626.854, verify); California's §15027 is similar. A paid human concierge on a consumer's live claim looks like unlicensed adjusting. So:
- **Concierge work:** only as a vendor under a licensed public adjuster's file.
- **Consumers:** self-serve software only. No human edits, no advice, no carrier contact.

**Data rule:**
- **Raw files:** parsed on upload, deleted within 72 hours.
- **Kept:** line items, card last-4 only, and a SHA-256 hash per source file so tier-A lines stay checkable.
- **Never:** inbox OAuth or stored credentials.
- **Deletion:** everything 90 days after export or on request; SOC 2 Type I by month 12 if adjusters ask.

**Market:** Florida had about 1,800 licensed public adjusters, about 700 practising (2012). The national count is unverified; assume 40–100 files per adjuster a year, a third with sizeable contents.

**12 months:**
- **Price:** $199 self-serve, $499 concierge.
- **Channel:** NAPIA and state-association events, founder outreach; CAC about $1,000 per firm (assumed).
- **Target:** 30 firms × 15 files = 450 files, about $110k.

**90 days:** weeks 1–4, concierge test. Weeks 5–10, parsers for Amazon, Costco, Target, Walmart, Apple, Home Depot and statements, plus tiering and template export. Weeks 11–13, deletion jobs, adjuster dashboard, 10 paying firms.

**Kill:** by month 6, under 15 paying firms, under 40 files a month, or desk adjusters accepting tier-A lines without receipts less than 70% of the time.
