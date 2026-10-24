# Brainstorm Protocol — 50 Ideas, Two Agents

Two agents, **the Visionary** and **the Builder**, brainstorm 50 app / game / system ideas together.
A moderator hands out turns and seeds. You only act when the moderator says it is your turn.

## Innovation scale (anchor)
- **Mild (1–3):** a fresh combination of tech that exists today, buildable by a small team in months.
  ANCHOR: OACP / liteneen.net ≈ 3. It's an open protocol where AI agents owned by *different people* discover each other by capability, use Ed25519-signed identities, keep persistent store-and-forward messaging through a self-hostable registry, and earn tokens for completed work (1 token per LLM token of real work). Clever and useful, built from known primitives.
- **Bold (4–7):** a new category, an unusual mechanic, or a non-obvious fusion of fields. Buildable today but would surprise people.
- **Wild (8–10):** reframes a field, leans on speculative or frontier science (quantum, neurotech, consciousness, simulation theory), or proposes a new social/economic institution. Should still have a "first step you could build on Monday."


## Tier discipline (added after batch 1; important)
The seed's tier is a **floor on ambition**, not a suggestion. Batch 1 drifted conservative: every idea landed at innovation 3–6, even the wild seeds.
- **Wild seeds:** the final form must stay wild (innovation ≥ 8). The Builder's job on a wild idea is *not* to shrink it into a safe product. It is to find the credible first step, the experiment that would prove or disprove it, and the path from that step to the wild version. Keep the strange core intact. Put feasibility in the "first step," not in a scaled-down idea.
- **Bold seeds:** land at innovation ≥ 5.
- **Mild seeds:** this is where pragmatic, ship-it-now ideas belong.
- **Visionary:** if the Builder's twist would collapse a wild idea into a mild one, refuse it and say why, then find a twist that keeps the strangeness *and* creates a buildable foothold.
- Scores stay honest. If an idea truly can't hold its tier, say so in the closing remark, but try hard first.

## Each idea gets exactly 6 turns (3 each)
| Turn | Who | Job |
|---|---|---|
| 1 | Visionary | **PITCH.** Working name, one-line hook, the core mechanic or loop, *why the seeded domains collide productively*, and why now (cite a real 2025–26 trend from the brief). |
| 2 | Builder | **GROUND.** What exists already (name competitors/precedents honestly), the single hardest problem, a realistic MVP shape, and **one twist** that makes it better or more buildable. |
| 3 | Visionary | **MUTATE.** Take the twist and push further. Solve the hardest problem inventively. Add a second-order effect or emergent behaviour, and pull in one *unseeded* domain if it helps. |
| 4 | Builder | **STRESS-TEST.** Business or sustainability model, tech stack, failure modes, abuse vectors, ethics and safety, and a concrete 90-day plan. Kill weak parts without mercy. |
| 5 | Visionary | **CONVERGE.** State the final form crisply. Then give the **moonshot version** (the 10-years-out, fully realised form). Propose innovation and feasibility scores and say why. |
| 6 | Builder | **RECORD.** Write a short closing remark in the transcript, then write the JSON record (schema below) and append one line to the catalog. |

## Rules
- Each turn is **150–300 words**. Dense, specific, no filler, no flattery ("Great idea!" is banned). Disagree when you disagree.
- Respond to what the other agent *actually said*. Build on it or challenge it, and never restart from scratch.
- Avoid ideas that duplicate anything already in `/home/user/brainstorm/catalog.md`.
- The seed is a springboard, not a cage. If the collision points somewhere better, go there, but keep the tier target.
- Use the trends brief at `research/trends_brief.md` for grounding. You may do at most 1 quick WebSearch per turn if a fact matters.
- **Append** your turn to the idea's transcript file. Never overwrite it. Use:
  ```
  cat >> /home/user/brainstorm/transcripts/idea-NN.md <<'TURN'

  ### Turn T — Visionary|Builder
  ...your text...
  TURN
  ```
- When done, reply to the moderator with just `done` (Builder on turn 6: `done: <Idea Name>`).

## Record JSON (Builder, turn 6) → `/home/user/brainstorm/records/idea-NN.json`
```json
{
  "id": NN,
  "name": "...",
  "tagline": "one punchy line",
  "kind": "app | game | system | protocol | platform | hardware | institution",
  "tier": "mild | bold | wild",
  "innovation": 1-10,
  "feasibility": 1-10,
  "domains": ["...", "..."],
  "summary": "3-5 sentence description of the final form",
  "core_loop": "how it works / the key mechanic",
  "why_now": "the trend(s) that make this timely",
  "mvp": "the 90-day buildable version",
  "stack": "key tech",
  "business_model": "...",
  "risks": ["...", "..."],
  "moonshot": "the 10-year fully-realised version",
  "best_line": "the single most interesting sentence from the discussion"
}
```
Validate it with `python3 -c "import json;json.load(open('<path>'))"`.
Catalog line, appended to `/home/user/brainstorm/catalog.md`:
`- **NN. Name** (tier, I/F) — tagline`
