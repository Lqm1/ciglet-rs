const fs = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const checks = fs.readdirSync(__dirname)
    .filter(name => /^wasm_.*_off\.cjs$/.test(name) && name !== 'wasm_all_off.cjs').sort();
if (checks.length !== 40) throw new Error('Feature-disabled suite inventory must contain all 40 families');
for (const name of checks) {
    const result = spawnSync(process.execPath, [path.join(__dirname, name), path.resolve(process.argv[2])], { encoding: 'utf8' });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`Feature-disabled family failed: ${name}\n${result.stdout}\n${result.stderr}`);
}
console.log(`Ciglet all-family feature-disabled WASM: ${checks.length} families absent`);
