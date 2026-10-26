"""Build the Fresh Spins page from records/*.json and transcripts/*.md.

  python3 site/build.py   ->  fresh-spins.html
"""
import glob
import json
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# "Where to start", in order. Ranked by business shape and the cost of finding out.
ORDER = [
    {"when": "Start now", "title": "While You're Here",
     "why": "Shops already pay for Jobber, business apps renew best, and the test is 10 shops paying $99 for a hand-built menu. Measure dollars per job."},
    {"when": "Start now", "title": "Morning Tile, board and app",
     "why": "Recurring revenue from a payer who already spends on pendants. The phone line and printed board come first; the app widget is a second door on the same backend."},
    {"when": "In parallel", "title": "Afterlist and Posted",
     "why": "Both are concierge tests that cost founder time, not money, sold to people facing a legal or financial deadline."},
    {"when": "Only if the pre-sale clears", "title": "Seated and Cashout",
     "why": "Consumer apps with a real trigger but no proven spend. Run the $2k pre-sales; build only if customers cost $30 or less."},
    {"when": "Gated", "title": "Ran It and Counter",
     "why": "Ran It needs short-term-rental hosts to reorder. Counter's price list stands alone; its OACP lane waits for agents that speak the protocol."},
]

ideas, transcripts = [], {}
for path in sorted(glob.glob(os.path.join(ROOT, "records", "idea-*.json"))):
    d = json.load(open(path))
    for k in ("id", "innovation", "feasibility", "market_fit_12mo"):
        d[k] = int(d[k])
    ideas.append(d)
    tp = os.path.join(ROOT, "transcripts", "idea-%02d.md" % d["id"])
    if os.path.exists(tp):
        transcripts[d["id"]] = open(tp).read()
ideas.sort(key=lambda d: d["id"])
app_tp = os.path.join(ROOT, "transcripts", "idea-03-app.md")
payload = {"ideas": ideas, "transcripts": transcripts, "order": ORDER,
           "app_transcript": open(app_tp).read() if os.path.exists(app_tp) else ""}
data = json.dumps(payload, ensure_ascii=False).replace("</", "<\\/")
out = open(os.path.join(ROOT, "site", "template.html")).read().replace("__DATA__", data)
dest = os.path.join(ROOT, "fresh-spins.html")
open(dest, "w").write(out)
print("wrote %s: %d ideas, %.0f KB" % (dest, len(ideas), len(out) / 1024))
