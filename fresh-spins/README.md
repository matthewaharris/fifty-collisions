# Fresh Spins

Eight near-term product ideas: fresh spins on old, boring products that a small team could build in 90 days and sell within 12 months. The same two-agent format as the original fifty (a **Visionary** and a **Builder**) worked each idea through six turns, retuned for this brief: pitch, ground, sharpen, stress-test, converge, record.

- **Round 1 (ideas 1–5):** any product, with exactly one idea about OACP.
- **Round 2 (ideas 6–8):** apps only, with the phone, watch or an existing wearable as the device, and a higher bar for market fit. It also includes a two-turn supplement asking whether Morning Tile could be just an app.

**Page:** [Fresh Spins](https://claude.ai/artifact/3fX3cqisXuWbY9o94MCqiq) (private Claude artifact) has the scoreboard, where to start, and every dossier and discussion. Rebuild it with `python3 site/build.py`, which writes `fresh-spins.html`.

## The eight

| # | Idea | Old product | The one change | First buyer | I / F / M |
| --- | --- | --- | --- | --- | --- |
| 01 | [Ran It](transcripts/idea-01.md) | Clean/dirty dishwasher magnet | A clip that flips itself to CLEAN after a real wash (a thermal mass ignores a hot-tap rinse) | Households; then short-term-rental hosts in 10-packs | 3 / 8 / 5 |
| 02 | [Posted](transcripts/idea-02.md) | Paper notice on the door | Each posting is photographed and sealed per door, and the tenant gets the identical record plus a rights page | California and HUD-assisted property managers, per legal notice | 4 / 7 / 5 |
| 03 | [Morning Tile](transcripts/idea-03.md) | Fridge calendar and the "call Mum" routine | She drops a tile and presses its number, which unlocks the grandkids' voice note of the day | Adult children of parents living alone | 4 / 8 / 6 |
| 04 | [Counter](transcripts/idea-04.md) **(OACP)** | Price list taped by the phone | Ranges built from the clinic's own invoices, signed once, and quoted identically to callers, Google and agents | Independent vet clinics | 4 / 7 / 5 |
| 05 | [Afterlist](transcripts/idea-05.md) | Shoebox of receipts and the home-inventory app | Rebuild the contents list after a loss from order histories and statements, with an evidence tier on every line | Public adjusters on unwaived total losses | 4 / 7 / 5 |
| 06 | [Cashout](transcripts/idea-06.md) | The tip log and mileage notebook | A ten-second log at close time; each night is sealed, and a co-worker's tip-out confirms it | Cash-tipped bartenders, servers and stylists | 4 / 8 / 5 |
| 07 | [Seated](transcripts/idea-07.md) | The paper blood-pressure log | The watch times five still minutes before each reading, and lifestyle experiments get a fair, sealed test | Apple Watch owners told to keep a week of readings | 4 / 7 / 5 |
| 08 | [While You're Here](transcripts/idea-08.md) | The carbon-copy job ticket and estimate pad | A capped menu of likely surprises, priced from the shop's own invoices, signed with the quote; her tap is the change order | 2–6 person shops already paying for Jobber | 4 / 7 / 6 |

I / F / M = innovation, feasibility, and market fit within 12 months (1–10). Innovation uses the original scale, with OACP at 3. Full records are in [`records/`](records/), one-line summaries in [`catalog.md`](catalog.md), and the rules in [`PROTOCOL.md`](PROTOCOL.md) (round 2 addendum at the end) and [`seeds.md`](seeds.md).

## Morning Tile as an app

[The supplement](transcripts/idea-03-app.md) designed an app-only Morning Tile:
- **Her side:** a three-tile home-screen widget that plays the day's family voice note when she taps it.
- **The family's side:** a lock-screen Live Activity ("Mum: green, 8:12").
- **Alerts:** an opt-in step count can soften the alert, but never raise it.

The Builder's verdict: **build it beside the board, never instead of it.**
- **As an app alone,** it scores 3/8/5. It competes with free Snug and the family WhatsApp group, has no dated trigger, and loses the parents over 80 who need it most.
- **As a second way in** on the same backend, it adds about three weeks to the family app the product needs anyway. Every family still gets the printed card with the phone number, and market fit stays at 6.
- **The new risk** is silent failure. A deleted widget or a new phone sends false "no tile" alerts.

## An honest read on market fit

None of the eight reached the target of 7 on paper: six scored 5 and two scored 6. That held across both rounds, even after round 2 raised the bar and added app research. The agents wouldn't score demand nobody had shown. The pattern:

- **Consumer apps stalled at 5.** Each had a real trigger (the tips deduction, the Watch's hypertension alerts), but none had proven spend on the old product, and winning a payer costs about a year's revenue.
- **B2B ideas did better.** Morning Tile and While You're Here reached 6 because their buyers already pay for something close: alert pendants, and Jobber.

Each idea names the result that would move it up. Each also has a demand test of about $1,500–2,500, or a few weeks of founder time, that runs before any real code.

| Idea | Demand test | Go if | Kill if |
| --- | --- | --- | --- |
| Ran It | 15 hand-built clips, silent videos, $600 of ads to a refundable pre-order page (3 weeks) | 1.5%+ of visitors pre-order at under $8 per customer | Eco cycles missed more than 2% of the time, or under 8 orders a day 60 days after launch |
| Posted | Free 3-week concierge pilots with 3 operators | 2 of 3 use it on 80%+ of legal notices and 1 pays $6 a notice | Under 8 paying operators or 400 paid notices a month by month 6 |
| Morning Tile | Meta ads to a $12/month card-on-file pre-sale; 30 families on a printed board and phone line (3 weeks) | $60 or less per customer, and 70%+ of parents calling 5 of 7 days in week 3 | Under 60% calling by day 60, or churn over 8% a month |
| Counter | Desk audit of 200 clinics' visible prices, sent as a free report with a $49 offer, plus 40 founder calls (3 weeks) | 10+ clinics pay | Under 40 paying clinics by month 6. The OACP lane has its own kill: under 50 agent quote requests in 90 days |
| Afterlist | Free 72-hour concierge rebuild of one live file for 60 public adjusters (4 weeks) | 5+ adjusters send a second file at $199, and 50%+ of claim value lands in the proven tiers | Under 15 paying firms by month 6, or desk adjusters accepting order-backed lines less than 70% of the time |
| Cashout | Ten bartending creators to a $39 pre-sale; 30 bartenders get the close-time prompt (3 weeks) | 150+ pre-orders at under $25 each, and 70% of worked shifts logged | Under 100 pre-orders, cost per customer over $45, or under 1,000 payers by April 30 |
| Seated | Search ads and seeded clinic pads to a $39.99 pre-sale; 20 users run the walk experiment (3 weeks) | $30 or less per pre-sale, and 75% of scheduled mornings recorded | Over $40 per pre-sale, or under 35% starting a second experiment by month 6 |
| While You're Here | 10 Jobber shops pay $99 for a hand-built menu; menu and plain quotes alternate (4 weeks) | 6 of 10 pay, and $150+ more per menu job or a 5-point higher close rate | Under 75 shops or churn over 5% a month by month 6 |

## Where to start

1. **While You're Here.** It has the soundest economics of the eight: shops already pay for Jobber, business apps renew best, and the test makes money. Build the capped, odds-labelled menu first; treat approval as plumbing.
2. **Morning Tile,** board and phone line first, with the app widget as a second way in on the same backend.
3. **Afterlist and Posted** together. Both are concierge tests that cost founder time, sold to people facing a legal or financial deadline.
4. **Seated and Cashout,** only if their pre-sales clear $30 or less per customer.
5. **Ran It and Counter** are gated. Ran It needs short-term-rental hosts to reorder. Counter's price list stands on its own; its OACP lane waits for agents that speak the protocol.

## Themes worth noting

- **Evidence was the hidden innovation in five of eight:** sealed postings, signed invoice-backed quotes, per-line evidence tiers, sealed tip nights and verified-rest readings. Each turns a boring record into something two parties can both trust. That's the mild, sellable descendant of several of the fifty (Piepowder, Settlement, Recount, Washout).
- **The agents cut their own best-sounding features when they failed on ethics or law:**
  - logging when tenants open a notice;
  - outbound calls to elderly parents;
  - AI test calls to clinics;
  - a consumer concierge that would count as unlicensed adjusting;
  - always-on location tracking;
  - income proof for landlords;
  - a 28-day blood-pressure result the statistics couldn't support.
- **Physical products and consumer apps share one problem:** without proven spend or a consumable, the first sale doesn't cover the cost of winning the customer.

## Files

| Path | Contents |
| --- | --- |
| `PROTOCOL.md` | The brief, the scoring scales, the six-turn table and the round 2 addendum |
| `persona_visionary.md`, `persona_builder.md` | The agents' role prompts |
| `seeds.md` | The eight seeds, each pointing at mechanisms in the fifty |
| `research/market_brief.md` | Sourced brief on fresh-spin hits, channels, manufacturing, tariffs and demand (October 2026) |
| `research/app_brief.md` | Sourced brief on subscription-app benchmarks, fast-fit apps, payments, phone and watch capabilities, and failure modes |
| `transcripts/idea-NN.md`, `transcripts/idea-03-app.md` | The discussions, and the Morning Tile app supplement |
| `records/idea-NN.json` | Structured records: price and margin, channel, demand test, 90-day MVP, kill metric, risks (idea 03 adds `app_version`) |
| `catalog.md` | One line per idea |
| `site/` | Page template and build script; `fresh-spins.html` is the built page |
