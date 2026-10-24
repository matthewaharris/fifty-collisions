"""Build the Fifty Collisions page from records/*.json and transcripts/*.md."""
import glob
import json
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def as_list(v):
    if isinstance(v, list):
        return [str(x) for x in v]
    return [s.strip() for s in re.split(r"[;,]\s*", str(v)) if s.strip()]


ideas, transcripts = [], {}
for path in sorted(glob.glob(os.path.join(ROOT, "records", "idea-*.json"))):
    d = json.load(open(path))
    d["id"] = int(d["id"])
    d["innovation"] = int(d["innovation"])
    d["feasibility"] = int(d["feasibility"])
    d["tier"] = str(d["tier"]).lower()
    d["domains"] = as_list(d.get("domains", []))
    d["risks"] = as_list(d.get("risks", []))
    ideas.append(d)
    tp = os.path.join(ROOT, "transcripts", "idea-%02d.md" % d["id"])
    if os.path.exists(tp):
        transcripts[d["id"]] = open(tp).read()

ideas.sort(key=lambda d: d["id"])
# The prototype shortlist, in priority order (see the plain-language guide).
POC = [47, 39, 45, 3, 9, 8, 24, 44, 1, 50, 10]
payload = json.dumps({"ideas": ideas, "transcripts": transcripts, "poc": POC}, ensure_ascii=False)
payload = payload.replace("</", "<\\/")  # keep </script> out of the JSON block

tpl = open(os.path.join(ROOT, "site", "template.html")).read()
out = tpl.replace("__DATA__", payload)
dest = os.path.join(ROOT, "fifty-collisions.html")
open(dest, "w").write(out)
print("wrote %s: %d ideas, %d transcripts, %.0f KB" % (dest, len(ideas), len(transcripts), len(out) / 1024))
