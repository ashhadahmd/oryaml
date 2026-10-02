import oryaml
import time
import tracemalloc
import os
from generate_yaml import generate_sample_yaml

SAMPLE_FILE = "input/sample.yaml"

if not os.path.exists(SAMPLE_FILE):
    print(f"Sample file not found. Generating {SAMPLE_FILE}...")
    generate_sample_yaml(SAMPLE_FILE, 100)

with open(SAMPLE_FILE, "rb") as f:
    yaml_content = f.read()

print(f"{'=' * 70}")
print(f"oryaml Benchmark - {SAMPLE_FILE} ({len(yaml_content) / 1024 / 1024:.2f} MB)")
print(f"{'=' * 70}")

# ============================================================
# 1. oryaml.loads() - Parse YAML to Python
# ============================================================
tracemalloc.start()
start = time.perf_counter()

data = oryaml.loads(yaml_content)

loads_time = time.perf_counter() - start
loads_current, loads_peak = tracemalloc.get_traced_memory()
tracemalloc.stop()

print(f"\n[loads] YAML string → Python object")
print(f"  Time:        {loads_time:.6f} seconds")
print(f"  Memory:      {loads_current / 1024:.2f} KB (current)")
print(f"  Peak Memory: {loads_peak / 1024:.2f} KB")

# ============================================================
# 2. oryaml.dumps() - Serialize Python to YAML string
# ============================================================
tracemalloc.start()
start = time.perf_counter()

yaml_output = oryaml.dumps(data)

dumps_time = time.perf_counter() - start
dumps_current, dumps_peak = tracemalloc.get_traced_memory()
tracemalloc.stop()

print(f"\n[dumps] Python object → YAML string")
print(f"  Time:        {dumps_time:.6f} seconds")
print(f"  Memory:      {dumps_current / 1024:.2f} KB (current)")
print(f"  Peak Memory: {dumps_peak / 1024:.2f} KB")

# ============================================================
# Summary
# ============================================================
print(f"\n{'=' * 70}")
print(f"{'Operation':<15} {'Time (s)':<15} {'Current (KB)':<15} {'Peak (KB)':<15}")
print(f"{'-' * 70}")
print(f"{'loads':<15} {loads_time:<15.6f} {loads_current/1024:<15.2f} {loads_peak/1024:<15.2f}")
print(f"{'dumps':<15} {dumps_time:<15.6f} {dumps_current/1024:<15.2f} {dumps_peak/1024:<15.2f}")
print(f"{'=' * 70}")
print(f"Total time: {loads_time + dumps_time:.6f} seconds")
