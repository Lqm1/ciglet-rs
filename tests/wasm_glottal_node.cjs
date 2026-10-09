const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_glottal_checks.mjs').then(async ({ verifyGlottalBindings }) => {
    console.log('Ciglet LF model actual Node.js:', JSON.stringify(await verifyGlottalBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)))));
}).catch(error => { console.error(error); process.exitCode = 1; });
