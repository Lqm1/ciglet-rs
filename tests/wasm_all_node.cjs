const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_all_checks.mjs').then(async ({ bindingFamilies, verifyAllBindings }) => {
    const existing = (await fs.readdir(__dirname))
        .filter(name => /^wasm_.*_node\.cjs$/.test(name) && name !== 'wasm_all_node.cjs')
        .map(name => name.slice(5, -9)).sort();
    const declared = bindingFamilies.map(([family]) => family).sort();
    if (JSON.stringify(existing) !== JSON.stringify(declared)) throw new Error('All-family inventory differs from individual Node suites');
    const result = await verifyAllBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)),
        (family, completed, total) => console.log(`Passed ${completed}/${total}: ${family}`));
    if (process.argv[3]) await fs.writeFile(process.argv[3], JSON.stringify(result, null, 2) + '\n');
    console.log(`Ciglet all-family actual Node.js: ${Object.keys(result).length} families passed`);
}).catch(error => { console.error(error); process.exitCode = 1; });
