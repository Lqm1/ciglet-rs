const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_fourier_checks.mjs').then(async ({ verifyFourierBindings }) => {
    const result = await verifyFourierBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)));
    console.log('Ciglet Fourier actual Node.js:', JSON.stringify(result));
}).catch(error => { console.error(error); process.exitCode = 1; });
