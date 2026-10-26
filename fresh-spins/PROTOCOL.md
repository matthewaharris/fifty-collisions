# Fresh Spins Protocol: 5 ideas, two agents

Two agents, **the Visionary** and **the Builder**, develop 5 product ideas together. A moderator hands out turns and seeds. You act only when the moderator says it's your turn.

This round is different from the original fifty (`../catalog.md`). Those reached for novelty. These are ideas we intend to **build and sell within the next 12 months**.

## The brief

- **Fresh spin on an old product.** Start from something people already buy, use or put up with: a sponge, a chore chart, a door hanger, an answering machine. Change one thing that makes it obviously better. Scrub Daddy is the model: nobody knew they needed a new sponge, and the one change (texture that firms in cold water and softens in warm) sold itself in a five-second demo.
- **Boring on the surface is fine, even good.** The innovation can sit underneath: a mechanism, a business model, a channel, a ritual, a piece of software nobody sees.
- **Feasible now.** A team of 1–4 can ship a sellable version in 90 days and reach real paying customers within 12 months, with no regulator approval, no lab breakthrough and no partner you can't get.
- **Market fit within 12 months.** A specific first buyer who already spends money or time on the old product, a channel that reaches them, a price they'd pay, and a test that shows fit (or its absence) early.
- **Still innovative.** Not a me-too. Name what's new and why the incumbents haven't done it.
- **Use the fifty for inspiration.** You may cross an old product with one or more of the 50 ideas in `../catalog.md` (records in `../records/`), borrowing a mechanism and dropping the moonshot. Or come up with something new. Don't just shrink one of the fifty.
- **OACP limit.** Exactly one of the five ideas is directly about OACP (the open agent-to-agent protocol at liteneen.net). That's the seed marked OACP. The other four must not depend on OACP.

## Scores

- **Innovation (1–10), same scale as the fifty.** OACP is the anchor at 3. Target **3–6** here: a real, nameable improvement, not a new category.
- **Feasibility (1–10).** Target **7 or higher**: shippable in 90 days by a small team on known tech and suppliers.
- **Market fit in 12 months (1–10), new this round.** 10 means paying, returning customers are near certain within 12 months; 5 means plausible but unproven demand; below 5 means no clear buyer or channel. Target **7 or higher**. Score it honestly.

## Each idea gets exactly 6 turns (3 each)

| Turn | Who | Job |
|---|---|---|
| 1 | Visionary | **PITCH.** Working name. The old product and what's stale about it. The fresh spin: the *one change*, said in a sentence. The first buyer and the moment they'd buy. Why now (cite the market brief or trends brief). Which of the fifty, if any, it borrows from and what it borrows. |
| 2 | Builder | **GROUND.** What exists (name real competitors and the incumbent's best version honestly, with prices). Why the incumbents haven't made this change. The single hardest problem. A realistic MVP. Rough unit economics (cost, price, margin, or for software, price and cost to serve). **One twist** that makes it more buildable or more sellable. |
| 3 | Visionary | **SHARPEN.** Make the change obvious in five seconds: the demo, the photo, the first-use moment. Solve the hardest problem. Propose the cheapest test of demand you could run in 2–4 weeks before building much. Pull in one unexpected idea if it helps. |
| 4 | Builder | **STRESS-TEST.** The 12-month plan: price, channel, customer acquisition cost, margin, first 100 customers and where they come from, copycat defence, regulation and liability, abuse and safety. A concrete 90-day build plan and a **kill metric with numbers**. Cut weak parts without mercy. |
| 5 | Visionary | **CONVERGE.** The final form, crisply. The "boring on the surface, but…" line. The 3-year version if it works (not a moonshot). Proposed innovation, feasibility and market-fit scores, with reasons. |
| 6 | Builder | **RECORD.** A short closing remark in the transcript, then the JSON record (schema below) and one catalog line. |

## Rules

- Each turn is **150–300 words**. Dense and specific, with real names, prices and numbers. No filler and no flattery. Disagree when you disagree.
- Respond to what the other agent actually said. Build on it or challenge it; never restart from scratch.
- Don't duplicate the fifty or the other fresh spins in `catalog.md` here.
- The seed is a springboard. If the old product points somewhere better, go there, but keep the brief.
- Ground claims in `research/market_brief.md` (this round) and `../research/trends_brief.md`. You may do at most 2 quick WebSearches per turn when a fact matters (a competitor's price, a sales figure, a rule). Never invent figures; say "unverified" when unsure.
- **Append** your turn to the idea's transcript. Never overwrite it:
  ```
  cat >> /home/user/fifty-collisions/fresh-spins/transcripts/idea-NN.md <<'TURN'

  ### Turn T — Visionary|Builder
  ...your text...
  TURN
  ```
- When done, reply to the moderator with just `done` (Builder on turn 6: `done: <Idea Name>`).

## Record JSON (Builder, turn 6) → `/home/user/fifty-collisions/fresh-spins/records/idea-NN.json`

```json
{
  "id": NN,
  "name": "...",
  "tagline": "one punchy line",
  "kind": "physical product | consumer app | SMB software | service | protocol | hybrid",
  "old_product": "the thing people already buy or use",
  "fresh_spin": "the one change, in one sentence",
  "boring_but": "the 'boring on the surface, but…' line",
  "innovation": 1-10,
  "feasibility": 1-10,
  "market_fit_12mo": 1-10,
  "oacp": true | false,
  "borrows_from": ["#NN Name: what it borrows", "..."],
  "summary": "3-5 sentences on the final form",
  "first_buyer": "who buys first, and the moment they buy",
  "why_now": "...",
  "competitors": ["name: what it does, price", "..."],
  "price_and_margin": "price, unit cost or cost to serve, margin",
  "channel": "how the first 100 and first 1,000 customers are reached",
  "demand_test": "the 2-4 week test before building much",
  "mvp_90_days": "...",
  "kill_metric": "the number that says stop",
  "risks": ["...", "..."],
  "three_year": "the 3-year version if it works",
  "best_line": "the single most useful sentence from the discussion"
}
```

Validate it with `python3 -c "import json;json.load(open('<path>'))"`.

Catalog line, appended to `/home/user/fifty-collisions/fresh-spins/catalog.md`:
`- **NN. Name** (I/F/M) — tagline`

---

# Round 2 addendum: ideas 06–08 and the Morning Tile supplement

Everything above still applies, with these changes.

- **Apps only.** No hardware product. The only devices are ones people already own: a phone, a watch or an existing wearable (prefer HealthKit and Health Connect over Oura, Whoop or Garmin APIs, which are restricted). Kind is `consumer app` or `SMB app`.
- **Stronger market fit.** Target **market fit 7 or higher** and earn it with evidence: people already pay for the old product or a close substitute; a dated trigger (a deadline, a new rule, a new OS or watch feature, a shutdown); a channel with a known install or acquisition cost; and a price that pays back that cost (the brief says a hard paywall or web funnel at about $30–60 a year, since freemium rarely pays back). Name what would make it fall short of 7.
- **New research:** read `research/app_brief.md` fully. It has subscription benchmarks, fast-fit app cases, 2026 distribution and payments, new phone and watch capabilities, and failure modes. Use it alongside `research/market_brief.md`.
- **Platform risk.** Ask "would WWDC 2027 or a free ChatGPT feature kill this?" Avoid rebuilding free OS features (iOS Check In, fall detection, sleep score). Keep health apps clear of diagnostic claims and finance apps clear of lending.
- **No OACP** in ideas 06–08. Idea 04 is the round's OACP idea.
- **Don't duplicate** ideas 01–05 here or the original fifty.

## The Morning Tile supplement (2 turns)

The question: could Morning Tile (idea 03) be just a mobile app, with no board and no phone line? Write it to `transcripts/idea-03-app.md`.

| Turn | Who | Job |
|---|---|---|
| A | Visionary | **APP VERSION.** Create the file with `# Idea 03 supplement — Morning Tile as an app` and write your turn: what the parent does each morning on her own phone (or watch), what replaces the tile and the fridge, what the family sees, how the voice-note reward and the one-person escalation work, what's lost and gained against the board, and how it avoids rebuilding iOS Check In. 200–350 words. |
| B | Builder | **VERDICT.** Ground it: competitors, App Store and older-user realities, retention, price and channel. Say whether the app should replace the board, sit beside it, or come first. Give innovation, feasibility and market-fit scores for the app version. Then add an `app_version` object to `records/idea-03.json` with keys `summary`, `parent_flow`, `family_flow`, `vs_board`, `price`, `verdict`, `innovation`, `feasibility`, `market_fit_12mo`. Validate the JSON. 200–350 words. |
