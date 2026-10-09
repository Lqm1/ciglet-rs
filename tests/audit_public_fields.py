"""Check the explicit own-header field ledger against the pinned upstream source.

This validates inventory integrity and evidence paths, not runtime correctness.
Run the referenced Native/C/WASM suites separately as documented in the audit.
"""
import argparse
import csv
from pathlib import Path
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('header', type=Path, help='Pinned upstream ciglet.h')
args = parser.parse_args()
root = Path(__file__).resolve().parent.parent
source = args.header.read_text(encoding='utf-8')
declarations = re.findall(r'typedef\s+struct\s*\{([^}]+)\}\s*(\w+)\s*;', source)
original = {(name, field) for body, name in declarations
            for field in re.findall(r'\b(?:FP_TYPE|int|FILE)\s*\*{0,2}\s*(\w+)\s*;', body)}
with (root / 'docs/public-field-bindings.csv').open(encoding='utf-8', newline='') as stream:
    rows = list(csv.DictReader(stream))
mapped = {(row['Type'], row['Field']) for row in rows}
assert len(declarations) == 5 and len(original) == 20
assert len(rows) == len(mapped) == 20
assert mapped == original, {'missing': sorted(original - mapped), 'extra': sorted(mapped - original)}
for row in rows:
    for field in ('NativeSource', 'CSource', 'WasmSource'):
        assert (root / row[field]).is_file(), row
    for field in ('NativeAccess', 'CAccess', 'WasmAccess', 'Contract'):
        assert row[field].strip(), row
    for evidence in row['Evidence'].split(';'):
        assert (root / evidence).is_file(), evidence
print('Own-header field ledger: all 5 types and 20 fields mapped; all evidence paths exist')
