"""Build the playable page: compile the rules core to WebAssembly and inline it.

  python3 web/build.py   ->  web/always-for-sale.html
Presets are the settings that passed Gate A (results/GATE_A.md).
"""
import base64
import json
import subprocess
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
subprocess.run(["cargo", "build", "--release", "--lib", "--target", "wasm32-unknown-unknown", "--quiet"], cwd=ROOT, check=True)
wasm = (ROOT / "target/wasm32-unknown-unknown/release/afs.wasm").read_bytes()

FIXED = {"prog_rate": 0.1, "late_start": 30, "max_plies": 200}
# The passing band from the confirmatory run (results/GATE_A.md): the x033 centre
# plus its passing neighbours on the tax-rate and starting-treasury lines.
pre = yaml.safe_load(open(ROOT / "harness/gates.yaml"))
centre = next(c for c in pre["centres"] if c["name"] == "x033")["point"]
VARIANTS = [
    ("Lean", {}),
    ("Lean, lower tax", {"tax_rate": centre["tax_rate"] - 2 * pre["lines"]["tax_rate"]}),
    ("Lean, richer start", {"start_treasury": centre["start_treasury"] + 2 * pre["lines"]["start_treasury"]}),
]
presets = []
for label, delta in VARIANTS:
    pt = {**centre, **delta}
    limit = 1 if pt["reprice_limit"] < 0.5 else 2
    params = {"tax_rate": pt["tax_rate"], "rent": pt["rent"], "start_treasury": round(pt["start_treasury"], 1),
              "prog_threshold": pt["prog_threshold"], "late_slope": pt["late_slope"], "reprice_limit": limit, **FIXED}
    label = (f"{label}: {params['start_treasury']:.0f} coins, {pt['tax_rate'] * 100:.1f}% tax, "
             f"{limit} reprice{'s' if limit > 1 else ''} a turn")
    presets.append({"label": label, "params": params, "coin_value": pt["coin_value"]})

tpl = (ROOT / "web/template.html").read_text()
out = tpl.replace("__WASM__", base64.b64encode(wasm).decode()).replace("__PRESETS__", json.dumps(presets))
dest = ROOT / "web/always-for-sale.html"
dest.write_text(out)
print(f"wrote {dest} ({len(out) / 1024:.0f} KB, wasm {len(wasm) / 1024:.0f} KB)")
