# Idea 08 — While You're Here
> Seed: 08. **The carbon-copy job ticket and the estimate pad** (SMB app for solo tradespeople and small crews, run from a phone). Plumbers, electricians, painters, handymen and remodelers still quote on paper or in text messages, and disputes over scope and change orders cost them money. Jobber and Housecall Pro dominate scheduling and invoicing, so find the job they don't do well. Possible borrowings: #35 Lookahead (a forecast the contractor commits to), Posted's sealed records (idea 02), Counter's ranges from your own invoices (idea 04).

### Turn 1 — Visionary
**Working name: While You're Here.**

**The loss the owner already feels.** A homeowner says "while you're here, can you also…" The tech does it, unwritten, and at invoice hears "I never agreed to that." Then the extra is discounted, charged back or turned into a one-star review. I have no verified loss figure, so the test measures it.

**The gap, in Jobber's own forum.** Users have requested change orders for years. The official workaround, editing the job, changes the price with no approval attached, and editing the quote wipes the original signature. Add-ons and ScopeProof, a standalone app, fill the hole.

**The one change: every extra gets a yes on the homeowner's own phone before the work starts, in 30 seconds.** The tech taps "extra," snaps a photo and texts it; the homeowner approves a page with price, photo and the untouched original quote. It's sealed with a timestamp (as in Posted) and lands on the Jobber job as a signed line. **What makes it fast is the surprise menu.** At quote time the homeowner also signs ranges for likely discoveries ("rot behind tile: $300–600"), priced from the shop's past invoices (as in Counter). Mid-job, the tech picks a line, no negotiating. A dispute exports one evidence pack.

**First buyer:** a 2–6 person remodeler, handyman or electrical shop already paying Jobber, the week after an extra was refused. **Price:** $39/month, against one saved $400 extra.

**Why now:** business apps renew best (40% first annual renewal, app brief), and Jobber's AI Receptionist now books jobs from a phone description, before anyone sees the wall.

**Channel:** the Jobber App Marketplace (fees unverified), that forum thread, and Jobber user Facebook groups.

**Held below 7 if:** Jobber ships native change orders, or lost extras prove too rare to justify $39 a month.

### Turn 2 — Builder
**The gap is real, and someone's already in it.** Jobber's community thread confirms the workaround: a second quote, signed but separate from the job. A 2026 review says Jobber still has no change-order module. In that same thread, **JobAmend** sells exactly our approval flow on Jobber: free until the first approved change order, then $29/month flat (vendor post). I found nothing on ScopeProof.

**Others** (prices from memory, unverified): Buildertrend, which absorbed CoConstruct, and Knowify, Contractor Foreman, Houzz Pro and Projul all have native signed change orders for remodelers, at roughly $50–500/month. Housecall Pro and ServiceTitan handle it through revised estimates. Joist is a cheap estimate pad.

**Marketplace:** GraphQL API, App Review before listing, 100+ integrations, revenue share not public. Jobber leaves niches to apps; that's why it hasn't built this.

**Law:** California B&P §7159.6, Pennsylvania's HICPA, New Jersey's home-improvement rules and Massachusetts 142A require written change orders (from memory; I didn't reopen the statutes). None is new, so there's no dated trigger. I found no residential data on how often extras go unpaid.

**Hardest problem:** the tech, not the homeowner. With her standing there, he says yes and skips the app rather than look petty.

**MVP (8 weeks):** Jobber OAuth; a mobile-web Extra button (no App Store); a homeowner page with photo, price and the original quote snapshot; e-signature and timestamp; a PDF and line written back to the job.

**Economics:** $39/month, about $1 to serve (SMS, storage). Assume marketplace CAC around $150 and 3% monthly churn: unverified.

**Twist: lead with the surprise menu, not approval.** Approval is JobAmend's at $29. Signing likely extras with the original quote means the tech never asks mid-job; he picks a line and she gets a notice. Check per state whether a pre-signed range satisfies §7159.6.

### Turn 3 — Visionary
**Accepted:** JobAmend owns approval at $29; the surprise menu is the product. Off-menu extras get a plain approval link.

**The five-second demo is the menu being built.** Connect Jobber; it diffs every quote against its final invoice, and lines found only on invoices are the shop's real extras: "Bathrooms, last 80 jobs: subfloor rot on 31, $380–720; shutoff valves on 44, $120." The owner prunes and approves: Counter's ranges plus Lookahead's committed forecast.

**At quote time it sells the job.** The homeowner sees "If we find…" lines with honest odds and signs the range with the quote. A visible worst case beats a suspiciously low rival bid. It also pre-prices "while we're here" items (valves, GFCIs). Jobber's optional items are static; these come from what this shop actually found.

**Saying yes and recording it become one act.** The signed menu stays live on her phone as "Your job list." When she asks "while you're here, the valve?", the tech says "It's on your list, add it," and she taps it herself. His phone buzzes: "Dana added valve, $120." Finding rot, he taps the line plus a photo; she gets a notice within her signed range. In states that need written change orders, the notice takes one tap from her, fast because the price was agreed (legal check per state).

**Demand test (4 weeks, 10 Jobber shops from the forum and Facebook groups):**
- **Offer:** $99 for a hand-built menu from their invoice export.
- **Method:** alternate menu and plain quotes.
- **Measure:** close rate, add-on dollars per job, extras collected vs written off.
- **Go:** 6 of 10 shops pay, plus either $150+ more per menu job or a close rate 5+ points higher. On about 200 quotes, close rate is only directional; dollars per job leads.

**Price:** $49/month.

### Turn 4 — Builder
**The diff is messier than the demo.** Jobber's GraphQL API exposes quote and invoice line items. Catalog lines match cleanly; free-text lines ("addl labor 2hr") need fuzzy matching, and lump-sum invoices hide extras entirely. The diff proposes, the owner approves. Measure how many shops yield five usable lines.

**Fairness rules, or it's lowball-then-upcharge** (an FTC Act and UDAP problem):
- **Full base:** covers the described scope; the menu holds only hidden conditions and add-ons.
- **Locked odds:** shown with counts ("31 of 80 jobs"); the owner can delete a line, not edit its odds.
- **Cap:** the worst case, shown beside the base, is capped at 25% of it.
- **Plain top line:** "Your price $8,400. At most $10,100, only if we find…"
- **Proof and choice:** hidden conditions bill only with a photo; she can decline any line.

**Her tap as a change order.** Under ESIGN and UETA, an intentional tap on an authenticated link can be a signature. Where statutes require written disclosures, ESIGN needs her e-consent first: collect it at quote signing. Each notice carries §7159.6's scope, price and schedule fields and the contractor's signature. Require her tap everywhere, rot included; get counsel in California and Pennsylvania first.

**12 months:** $49/month plus a $99 menu build. In scope: 2–6-person remodelers, handymen and electricians on Jobber, guessed at 15–40k shops. CAC about $250 (forum, Facebook groups, NARI, then the marketplace). At 3.5% monthly churn, lifetime revenue is about $1,400. Month 12: 300 shops, about $175k ARR. Jobber going native within two years is likely.

**90 days:** weeks 1–4, concierge test. Weeks 3–9, OAuth, diff, capped menu, "Your job list," write-back. Weeks 10–13, App Review, 25 paying shops.

**Kill:** under 6 of 10 paying; month 6, under 75 shops, churn over 5%, or under 40% of menu jobs with a tap.

### Turn 5 — Visionary
**Accepted, all of it:** proposals only, locked odds, a 25% cap on the top line, photo proof, declinable lines, her tap everywhere after e-consent. The cap is also the sales line: "At most $10,100" is a promise a lowballer can't print.

**Final form.** While You're Here is a $49-a-month Jobber app plus a $99 menu build, for 2–6-person remodelers, handymen and electricians. It diffs quotes against final invoices and proposes a surprise menu: hidden conditions with locked odds and capped ranges, plus pre-priced add-ons. The homeowner signs it with the quote. It stays live on her phone as "Your job list"; her tap is the change order, written back to Jobber.

**Boring on the surface,** it's a change-order form. **But** the shop prices its surprises from its own history before the job starts, so saying yes and getting paid are the same tap.

**3-year version:**
- **More platforms:** Housecall Pro and QuickBooks histories.
- **Regional odds:** pooled, anonymised odds ("rot in 38% of pre-1970 bathrooms in this county") let new shops quote honestly.
- **If Jobber goes native:** approval goes free; we keep selling the odds engine and regional data, or sell them to Jobber.
- **Target:** 3–5k shops, about $2–3M ARR.

**Scores.**
- **Innovation 4:** change orders are commodity. Conditional ranges with honest odds from the shop's own invoices are new to the trades.
- **Feasibility 7:** a known API, but a fuzzy diff and state-by-state change-order law.
- **Market fit 6:** proven spend (shops pay Jobber, and JobAmend sells the adjacent flow) and business apps renew best. But there's no measured loss figure, no dated trigger, a $250 CAC, and Jobber may go native. It earns 7 if the concierge test shows $150+ more per menu job with 6 of 10 shops paying.
