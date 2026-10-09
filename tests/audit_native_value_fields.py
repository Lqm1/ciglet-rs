"""Compare the native named-field value inventory with public module sources.

This checks inventory integrity, not behavior or platform acceptance. Opaque
owners, tuple errors and enum variants have separate functional inventories.
The extractor supports the current flat named-field declarations and rejects
unrecognized public field lines instead of silently ignoring them.
"""
import csv
from pathlib import Path
import re

root = Path(__file__).resolve().parent.parent
modules = re.findall(r'^pub mod (\w+);', (root / 'src/lib.rs').read_text(encoding='utf-8'), re.M)
original = {}
for module in modules:
    if module in ('c_api', 'wasm_api'):
        continue
    source = (root / 'src' / f'{module}.rs').read_text(encoding='utf-8')
    declarations = re.findall(r'pub struct (\w+)(?:<[^{}]*>)?\s*\{([^{}]*)\}', source)
    starts = re.findall(r'pub struct (\w+)(?:<[^{}]*>)?\s*\{', source)
    assert starts == [name for name, _ in declarations], (module, 'Unsupported declaration')
    for name, body in declarations:
        fields = []
        for line in body.splitlines():
            if re.match(r'\s*pub\s', line):
                match = re.match(r'\s*pub (\w+):\s*.+,\s*$', line)
                assert match, (module, name, line)
                fields.append(match[1])
        if fields:
            original[f'{module}::{name}'] = fields

with (root / 'docs/native-value-field-bindings.csv').open(encoding='utf-8', newline='') as stream:
    rows = list(csv.DictReader(stream))
mapped = {row['Type']: row['Fields'].split(';') for row in rows}
assert len(rows) == len(mapped), 'Duplicate types'
assert mapped == original, {'source': original, 'ledger': mapped}
for row in rows:
    assert row['NativeSource'] == f"src/{row['Type'].split('::')[0]}.rs", row
    for column in ('NativeSource', 'CSource', 'WasmSource'):
        for path in row[column].split(';'):
            assert (root / path).is_file(), (column, path)
    for path in row['Evidence'].split(';'):
        assert (root / path).is_file(), path
    assert row['CRepresentation'].strip() and row['WasmRepresentation'].strip(), row
print(f'Native named-field inventory: {len(rows)} types, '
      f'{sum(map(len, mapped.values()))} fields; source sets and paths agree')
