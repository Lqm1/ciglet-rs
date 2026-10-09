const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_spectral_peak_checks.mjs').then(async ({ verifySpectralPeakBindings }) => {
    console.log('Ciglet spectral peak actual Node.js:', JSON.stringify(await verifySpectralPeakBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)))));
}).catch(error => { console.error(error); process.exitCode = 1; });
