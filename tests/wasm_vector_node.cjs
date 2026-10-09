const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_vector_checks.mjs').then(async ({ verifyVectorBindings }) => {
    const result = await verifyVectorBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)));
    console.log('Ciglet vectors actual Node.js:', JSON.stringify(result));
}).catch(error => { console.error(error); process.exitCode = 1; });
