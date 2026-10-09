const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_smoothing_checks.mjs').then(async ({ verifySmoothingBindings }) => {
    const result = await verifySmoothingBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)));
    console.log('Ciglet smoothing actual Node.js:', JSON.stringify(result));
}).catch(error => { console.error(error); process.exitCode = 1; });
