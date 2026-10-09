const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_frequency_distance_checks.mjs').then(async ({ verifyFrequencyDistanceBindings }) => {
    const result = await verifyFrequencyDistanceBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)));
    console.log('Ciglet frequency/distance actual Node.js:', JSON.stringify(result));
}).catch(error => { console.error(error); process.exitCode = 1; });
