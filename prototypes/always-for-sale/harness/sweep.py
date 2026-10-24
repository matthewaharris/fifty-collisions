"""Sweep harness for Always For Sale (plans/poc-07, Gate A).

  python3 harness/sweep.py explore   # Sobol sweep, exploratory seeds, ranks cells
  python3 harness/sweep.py confirm   # pre-registered lines around the gates.yaml centres
  python3 harness/sweep.py report    # gate decision from the confirmatory run

The CLI writes one JSON line per game. Results are stored as Parquet and the
gates are evaluated in DuckDB, with bootstrap confidence intervals over seed
pairs (each pair is one seed played twice with sides swapped).
"""
import json
import subprocess
import sys
from pathlib import Path

import duckdb
import numpy as np
import pandas as pd
import yaml
from scipy.stats import qmc

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
OUT = ROOT / "results"
CLI = ROOT / "target" / "release" / "afs-sim"
GATES = HERE / "gates.yaml"

# Exploratory axes: name -> (low, high). reprice_limit is 1 below 0.5, else 2.
AXES = {
    "tax_rate": (0.02, 0.20),
    "rent": (0.0, 3.0),
    "start_treasury": (10.0, 60.0),
    "prog_threshold": (20.0, 100.0),
    "late_slope": (0.0, 0.01),
    "coin_value": (60.0, 250.0),
    "reprice_limit": (0.0, 1.0),
}
FIXED = {"prog_rate": 0.1, "late_start": 30, "max_plies": 200}
MATCHUPS = ["self", "vs_no_buy", "vs_rote"]
EXPLORE_SEED = 1            # exploratory seeds: 1 .. 1e6
CONFIRM_SEED = 1_000_000_000  # confirmatory seeds never overlap the exploratory ones
DRAWS = ("PlyCap", "Stalemate", "Insufficient", "MutualBankruptcy")


def cell_params(point):
    p = dict(FIXED)
    for k, v in point.items():
        if k == "reprice_limit":
            p[k] = 1 if v < 0.5 else 2
        elif k != "coin_value":
            p[k] = float(v)
    return p


def write_cells(cells, path, seed_start, pairs):
    with open(path, "w") as f:
        for cid, point in cells:
            for m in MATCHUPS:
                f.write(json.dumps({
                    "cell_id": cid, "params": cell_params(point), "coin_value": float(point["coin_value"]),
                    "matchup": m, "seed_start": seed_start, "pairs": pairs}) + "\n")


def run_cli(cells_path, games_path, threads=4):
    subprocess.run(["cargo", "build", "--release", "--quiet"], cwd=ROOT, check=True)
    subprocess.run([str(CLI), "run", "--cells", str(cells_path), "--out", str(games_path),
                    "--threads", str(threads)], check=True)


def load_games(games_path, cells):
    rows = []
    with open(games_path) as f:
        for line in f:
            g = json.loads(line)
            gaps, prev = [], 0
            for b in g["buyouts"]:
                gaps.append(b["ply"] - prev)
                prev = b["ply"]
            rows.append({
                "cell_id": g["cell_id"], "matchup": g["matchup"], "seed": g["seed"],
                "full_color": g["full_color"], "full_score": g["full_score"], "reason": g["reason"],
                "plies": g["plies"], "n_buyouts": len(g["buyouts"]), "gaps": gaps,
                "reprices": sum(g["reprices"]), "buys": sum(g["buys"]),
            })
    df = pd.DataFrame(rows)
    pts = pd.DataFrame([{"cell_id": c, **p} for c, p in cells])
    return df.merge(pts, on="cell_id")


def bootstrap_ci(pair_scores, n_boot=2000, seed=0):
    """95% CI of the mean pair score (each pair = mean of its two games)."""
    rng = np.random.default_rng(seed)
    x = np.asarray(pair_scores)
    idx = rng.integers(0, len(x), size=(n_boot, len(x)))
    means = x[idx].mean(axis=1)
    return float(np.quantile(means, 0.025)), float(np.quantile(means, 0.975))


def cell_metrics(df, ci=False):
    con = duckdb.connect()
    con.register("g", df.drop(columns=["gaps"]))
    gaps = df[df.matchup == "self"][["cell_id", "gaps"]].explode("gaps").dropna()
    gaps["gaps"] = gaps["gaps"].astype(float)
    con.register("gp", gaps)
    m = con.execute(f"""
        WITH s AS (
          SELECT cell_id,
                 avg(CASE WHEN reason IN {DRAWS} THEN 1.0 ELSE 0.0 END) AS draw_share,
                 sum(reprices) * 1.0 / sum(plies) AS reprice_share,
                 sum(buys) * 1.0 / sum(plies) AS buy_share,
                 median(plies) AS plies_median
          FROM g WHERE matchup = 'self' GROUP BY cell_id),
        gap AS (SELECT cell_id, median(gaps) AS gap_median FROM gp GROUP BY cell_id),
        w AS (
          SELECT cell_id,
                 avg(CASE WHEN matchup = 'vs_no_buy' THEN full_score END) AS win_vs_no_buy,
                 avg(CASE WHEN matchup = 'vs_rote' THEN full_score END) AS win_vs_rote
          FROM g GROUP BY cell_id)
        SELECT * FROM s JOIN w USING (cell_id) LEFT JOIN gap USING (cell_id)
    """).df()
    if ci:
        for col, mu in [("vs_no_buy", "no_buy"), ("vs_rote", "rote")]:
            lo = {}
            for cid, grp in df[df.matchup == col].groupby("cell_id"):
                pairs = grp.groupby("seed").full_score.mean()
                lo[cid] = bootstrap_ci(pairs.values)[0]
            m[f"ci_lo_{mu}"] = m.cell_id.map(lo)
    return m


def passes(row, g):
    ok = g["gap_plies"][0] <= (row.gap_median if pd.notna(row.gap_median) else -1) <= g["gap_plies"][1]
    ok &= row.draw_share < g["max_draw_share"]
    for mu in ("no_buy", "rote"):
        ok &= row[f"win_vs_{mu}"] >= g["min_win_rate"]
        if f"ci_lo_{mu}" in row:
            ok &= row[f"ci_lo_{mu}"] > g["min_ci_lower"]
    return bool(ok)


def explore(n_cells=128, pairs=50):
    OUT.mkdir(exist_ok=True)
    sob = qmc.Sobol(d=len(AXES), scramble=True, seed=20261024)
    u = sob.random(n_cells)
    lo = np.array([a[0] for a in AXES.values()])
    hi = np.array([a[1] for a in AXES.values()])
    X = qmc.scale(u, lo, hi)
    cells = [(f"x{i:03d}", dict(zip(AXES, map(float, row)))) for i, row in enumerate(X)]
    write_cells(cells, OUT / "explore_cells.jsonl", EXPLORE_SEED, pairs)
    run_cli(OUT / "explore_cells.jsonl", OUT / "explore_games.jsonl")
    df = load_games(OUT / "explore_games.jsonl", cells)
    df.drop(columns=["gaps"]).to_parquet(OUT / "explore_games.parquet")
    m = cell_metrics(df).merge(pd.DataFrame([{"cell_id": c, **p} for c, p in cells]), on="cell_id")
    g = yaml.safe_load(open(GATES))["gate_a"] if GATES.exists() else DEFAULT_GATE
    m["pass_point"] = m.apply(lambda r: passes(r, g), axis=1)
    m.to_csv(OUT / "explore_cells.csv", index=False)
    show = ["cell_id", "gap_median", "draw_share", "win_vs_no_buy", "win_vs_rote", "reprice_share",
            "tax_rate", "rent", "start_treasury", "coin_value", "reprice_limit", "pass_point"]
    print(m.sort_values("win_vs_rote", ascending=False)[show].head(20).to_string(index=False))
    print(f"\n{m.pass_point.sum()} of {len(m)} exploratory cells pass the point estimates")
    return m


DEFAULT_GATE = {"gap_plies": [6, 10], "max_draw_share": 0.15, "min_win_rate": 0.60, "min_ci_lower": 0.55}


def line_cells(centre, axis, step, levels=2):
    out = []
    for k in range(-levels, levels + 1):
        p = dict(centre)
        p[axis] = centre[axis] + k * step
        lo, hi = AXES[axis]
        if lo <= p[axis] <= hi:
            out.append((k, p))
    return out


def confirm():
    pre = yaml.safe_load(open(GATES))
    cells, lines = [], []
    for c in pre["centres"]:
        centre = {k: float(v) for k, v in c["point"].items()}
        for axis, step in pre["lines"].items():
            ids = []
            for k, p in line_cells(centre, axis, step):
                cid = f"{c['name']}:{axis}:{k:+d}"
                cells.append((cid, p))
                ids.append(cid)
            lines.append({"centre": c["name"], "axis": axis, "cells": ids})
    write_cells(cells, OUT / "confirm_cells.jsonl", CONFIRM_SEED, pre["pairs_per_comparison"])
    json.dump(lines, open(OUT / "confirm_lines.json", "w"), indent=1)
    run_cli(OUT / "confirm_cells.jsonl", OUT / "confirm_games.jsonl")
    report()


def report():
    pre = yaml.safe_load(open(GATES))
    g = pre["gate_a"]
    lines = json.load(open(OUT / "confirm_lines.json"))
    cells = []
    for line in lines:
        for cid in line["cells"]:
            name, axis, k = cid.split(":")
            c = next(c for c in pre["centres"] if c["name"] == name)
            p = {kk: float(v) for kk, v in c["point"].items()}
            p[axis] += int(k) * pre["lines"][axis]
            cells.append((cid, p))
    cells = list(dict(cells).items())
    df = load_games(OUT / "confirm_games.jsonl", cells)
    df.drop(columns=["gaps"]).to_parquet(OUT / "confirm_games.parquet")
    m = cell_metrics(df, ci=True)
    m["pass"] = m.apply(lambda r: passes(r, g), axis=1)
    m.to_csv(OUT / "confirm_cells.csv", index=False)
    band_found = False
    out = ["# Gate A: confirmatory run", ""]
    for line in lines:
        rows = m.set_index("cell_id").loc[line["cells"]]
        run, best = 0, 0
        for ok in rows["pass"]:
            run = run + 1 if ok else 0
            best = max(best, run)
        band = best >= g["band_cells"]
        band_found |= band
        out.append(f"## Centre {line['centre']}, varying {line['axis']}: "
                   f"{'BAND' if band else 'no band'} ({best} adjacent passing cells)")
        out.append("")
        out.append("| cell | gap (plies) | draws | vs no-buy | CI low | vs rote | CI low | reprice share | pass |")
        out.append("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
        for cid, r in rows.iterrows():
            out.append(f"| {cid} | {r.gap_median} | {r.draw_share:.1%} | {r.win_vs_no_buy:.1%} | {r.ci_lo_no_buy:.1%} "
                       f"| {r.win_vs_rote:.1%} | {r.ci_lo_rote:.1%} | {r.reprice_share:.1%} | {'yes' if r['pass'] else 'no'} |")
        out.append("")
    out.insert(2, f"**Decision: {'GO' if band_found else 'NO-GO'}** "
                  f"(a band needs {g['band_cells']} adjacent passing cells on one axis).")
    out.insert(3, "")
    (OUT / "GATE_A.md").write_text("\n".join(out) + "\n")
    print("\n".join(out))


if __name__ == "__main__":
    {"explore": explore, "confirm": confirm, "report": report}[sys.argv[1]]()
