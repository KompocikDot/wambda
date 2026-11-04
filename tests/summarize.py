"""Turns the raw results/*.csv (test,variant,run,metric,value,unit) into one table per test:
a row per metric, a column per variant, cells as 'median [min-max]' over the runs.
Usage: python3 tests/summarize.py [results dir]
"""
import csv
import statistics
import sys
from pathlib import Path

results_dir = Path(sys.argv[1] if len(sys.argv) > 1 else "results")

for path in sorted(results_dir.glob("*.csv")):
    with open(path) as file:
        rows = list(csv.DictReader(file))
    if not rows or "metric" not in rows[0]:
        continue  # not a measurement file (e.g. security.csv)

    values = {}  # (metric, unit) -> variant -> list of values
    variants = []
    for row in rows:
        if row["variant"] not in variants:
            variants.append(row["variant"])
        values.setdefault((row["metric"], row["unit"]), {}).setdefault(row["variant"], []).append(float(row["value"]))

    table = [["metric"] + variants]
    for (metric, unit), by_variant in values.items():
        cells = []
        for variant in variants:
            runs = by_variant.get(variant)
            if runs is None:
                cells.append("-")
            else:
                cells.append(f"{statistics.median(runs):.2f} [{min(runs):.2f}-{max(runs):.2f}]")
        table.append([f"{metric} [{unit}]"] + cells)

    widths = [max(len(row[i]) for row in table) for i in range(len(variants) + 1)]
    print(f"\n{rows[0]['test']} (median [min-max] of {len({r['run'] for r in rows})} runs)")
    for row in table:
        print("  ".join(cell.ljust(width) for cell, width in zip(row, widths)))
