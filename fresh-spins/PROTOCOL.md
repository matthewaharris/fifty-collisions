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
