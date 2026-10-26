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
