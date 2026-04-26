"""
CemirCol Benchmark — CSV, JSON, Parquet ve CemirCol formatlarını karşılaştırır.
"""
import time
import os
import json
import csv
import pandas as pd
from cemircol import CemircolWriter, CemircolReader

ROWS = 10_000_000

def generate_data(rows: int) -> dict:
    return {
        "id": list(range(rows)),
        "value": [float(x) * 1.5 for x in range(rows)],
    }

def write_csv(path: str, data: dict):
    keys = list(data.keys())
    with open(path, "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(keys)
        for i in range(len(data[keys[0]])):
            w.writerow([data[k][i] for k in keys])

def read_csv_column(path: str, column: str) -> list:
    with open(path, "r") as f:
        r = csv.DictReader(f)
        return [float(row[column]) for row in r]

def write_json(path: str, data: dict):
    with open(path, "w") as f:
        json.dump(data, f)

def read_json_column(path: str, column: str) -> list:
    with open(path, "r") as f:
        return json.load(f)[column]

def fmt_size(size_bytes: int) -> str:
    mb = size_bytes / (1024 * 1024)
    return f"{mb:.2f} MB"

def benchmark():
    print(f"Generating {ROWS:,} rows of test data...")
    data = generate_data(ROWS)

    cemir_path = "bench.cemir"
    csv_path = "bench.csv"
    json_path = "bench.json"
    parquet_path = "bench.parquet"

    # --- Write ---
    CemircolWriter.write(cemir_path, data)
    write_csv(csv_path, data)
    write_json(json_path, data)
    pd.DataFrame(data).to_parquet(parquet_path)

    print("\n--- File Sizes ---")
    for label, path in [("CemirCol", cemir_path), ("Parquet", parquet_path), ("CSV", csv_path), ("JSON", json_path)]:
        print(f"  {label:10s}: {fmt_size(os.path.getsize(path))}")

    # --- Read (single column: "value") ---
    print("\n--- Read Times (column: 'value') ---")

    start = time.perf_counter()
    reader = CemircolReader(cemir_path)
    _ = reader.query("value")
    t_cemir = time.perf_counter() - start
    print(f"  {'CemirCol':10s}: {t_cemir:.5f} s")

    start = time.perf_counter()
    _ = pd.read_parquet(parquet_path, columns=["value"])
    t_pq = time.perf_counter() - start
    print(f"  {'Parquet':10s}: {t_pq:.5f} s")

    start = time.perf_counter()
    _ = read_csv_column(csv_path, "value")
    t_csv = time.perf_counter() - start
    print(f"  {'CSV':10s}: {t_csv:.5f} s")

    start = time.perf_counter()
    _ = read_json_column(json_path, "value")
    t_json = time.perf_counter() - start
    print(f"  {'JSON':10s}: {t_json:.5f} s")

    # Cleanup
    for p in [cemir_path, csv_path, json_path, parquet_path]:
        if os.path.exists(p):
            os.remove(p)

if __name__ == "__main__":
    benchmark()
