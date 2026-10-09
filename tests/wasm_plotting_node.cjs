const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_plotting_checks.mjs').then(async ({ verifyPlottingBindings }) => {
    console.log('Ciglet plotting actual Node.js:', JSON.stringify(await verifyPlottingBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)))));
}).catch(error => { console.error(error); process.exitCode = 1; });
