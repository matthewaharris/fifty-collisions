# Plain-language summaries: brief

The user finds many of the 50 ideas complex and abstract. Write a **plain-language companion entry** for each idea assigned to you. Do NOT edit any existing file; only create your output file.

Inputs per idea NN: `/home/user/brainstorm/records/idea-NN.json` (the structured record) and `/home/user/brainstorm/transcripts/idea-NN.md` (the 6-turn discussion; skim it for anything the record leaves unclear, especially the Builder's closing verdict in turn 6).

## Voice
Explain it like you would to a smart friend outside the field, over coffee. Short sentences. No jargon unless you immediately explain it in plain words. Use a concrete everyday example or analogy wherever the idea is abstract. Be honest about weak spots; don't hype. Don't copy the record's wording. Rephrase from scratch.

## Output
A single JSON file `/home/user/brainstorm/plain/batch-K.json` containing an array of objects, one per idea, in order:
```json
{
  "id": 13,
  "name": "Canary Ladder",            // unchanged from record
  "plain_one_liner": "≤ 20 words. What it is, in words a 12-year-old would get.",
  "gist": "2–3 sentences: what it actually is and does. Lead with the everyday picture.",
  "how_it_works": ["3–4 short steps a user or the system goes through"],
  "example": "1–2 sentences: one concrete scenario, e.g. 'Maya, a dairy farmer in Kenya, ...'",
  "who_its_for": "1 sentence: who uses it / who pays",
  "why_it_matters": "1–2 sentences: the value, the problem it solves, why anyone should care",
  "verdict": "exactly one of: 'Buildable now' | 'Buildable with effort' | 'Long shot' | 'Moonshot'",
  "verdict_reason": "1 sentence explaining the verdict (cite the biggest blocker or enabler)",
  "biggest_hurdle": "1 sentence, plain",
  "first_step": "1 sentence: the smallest real thing someone could build or test first",
  "jargon": [{"term": "cat bond", "meaning": "≤ 15 words"}]   // 0–3 terms that appear in the original record and would confuse a layperson
}
```
Verdict guide: feasibility 7+ → usually 'Buildable now'; 5–6 → 'Buildable with effort'; 4 → 'Long shot'; ≤3 → 'Moonshot'. Use judgment from the transcript if it clearly disagrees, but stay close.

Validate with `python3 -c "import json;d=json.load(open('<path>'));print(len(d))"`. Reply with only `done: <count>`.
