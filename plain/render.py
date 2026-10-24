"""Render plain-language batch JSON into markdown sections for the companion doc.

Usage: python3 render.py section K   -> markdown for ideas in batch-K
       python3 render.py glance      -> the at-a-glance table for all batches present
       python3 render.py jargon      -> merged jargon decoder
"""
import glob
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)


def records():
    out = {}
    for p in glob.glob(os.path.join(ROOT, "records", "idea-*.json")):
        d = json.load(open(p))
        out[int(d["id"])] = d
    return out


def batch(k):
    return json.load(open(os.path.join(HERE, "batch-%d.json" % k)))


def all_entries():
    items = []
    for p in sorted(glob.glob(os.path.join(HERE, "batch-*.json"))):
        items += json.load(open(p))
    return sorted(items, key=lambda e: int(e["id"]))


def cell(s):
    return str(s).replace("|", "/").replace("\n", " ").strip()


def section(k):
    rec = records()
    lo, hi = (k - 1) * 10 + 1, k * 10
    lines = ["## Ideas %d–%d" % (lo, hi), ""]
    for e in batch(k):
        r = rec[int(e["id"])]
        lines += [
            "### %d. %s" % (int(e["id"]), r["name"]),
            "",
            "*%s*" % e["plain_one_liner"].strip(),
            "",
            "**%s %s · innovation %s/10 · feasibility %s/10 · %s**"
            % (r["tier"].capitalize(), r["kind"], r["innovation"], r["feasibility"], e["verdict"]),
            "",
            "**The gist.** " + e["gist"].strip(),
            "",
            "**How it works.**",
            "",
        ]
        lines += ["%d. %s" % (i + 1, s.strip()) for i, s in enumerate(e["how_it_works"])]
        lines += [
            "",
            "**Example.** " + e["example"].strip(),
            "",
            "**Who it's for.** " + e["who_its_for"].strip(),
            "",
            "**Why it matters.** " + e["why_it_matters"].strip(),
            "",
            "**Viability: %s.** %s" % (e["verdict"], e["verdict_reason"].strip()),
            "",
            "- **Biggest hurdle:** " + e["biggest_hurdle"].strip(),
            "- **First step:** " + e["first_step"].strip(),
        ]
        if e.get("jargon"):
            lines += ["", "**Jargon:** " + "; ".join(
                "*%s* = %s" % (j["term"].strip(), j["meaning"].strip().rstrip(".")) for j in e["jargon"])]
        lines += [""]
    return "\n".join(lines).strip()


def glance():
    rec = records()
    rows = ["## At a glance", "",
            "Every idea in one line, in discussion order. Innovation (I) and feasibility (F) are scored out of 10, with OACP at innovation 3.", "",
            "| # | Idea | In plain words | Tier | I | F | Verdict |",
            "| --- | --- | --- | --- | --- | --- | --- |"]
    for e in all_entries():
        r = rec[int(e["id"])]
        rows.append("| %d | **%s** | %s | %s | %s | %s | %s |" % (
            int(e["id"]), cell(r["name"]), cell(e["plain_one_liner"]), r["tier"].capitalize(),
            r["innovation"], r["feasibility"], cell(e["verdict"])))
    return "\n".join(rows)


def jargon():
    seen = {}
    for e in all_entries():
        for j in e.get("jargon", []):
            key = j["term"].strip().lower()
            if key not in seen:
                seen[key] = (j["term"].strip(), j["meaning"].strip().rstrip("."), [int(e["id"])])
            else:
                seen[key][2].append(int(e["id"]))
    rows = ["## Jargon decoder", "",
            "Technical terms from the original records, in plain words. The last column lists the ideas that use each one.", "",
            "| Term | Plain meaning | Used in |", "| --- | --- | --- |"]
    for _, (t, m, ids) in sorted(seen.items()):
        rows.append("| %s | %s | %s |" % (cell(t), cell(m), ", ".join("#%d" % i for i in ids)))
    return "\n".join(rows)


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "section":
        print(section(int(sys.argv[2])))
    elif cmd == "glance":
        print(glance())
    elif cmd == "jargon":
        print(jargon())
