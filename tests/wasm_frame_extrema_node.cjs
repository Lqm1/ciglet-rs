const fs = require('node:fs/promises');
const path = require('node:path');
const api = require(path.join(path.resolve(process.argv[2]), 'ciglet_rs.js'));
import('./wasm_frame_extrema_checks.mjs').then(async ({ verifyFrameExtremaBindings }) => {
    const result = await verifyFrameExtremaBindings(api, name => fs.readFile(path.join(__dirname, 'fixtures', name)));
    console.log('Ciglet frame/extrema actual Node.js:', JSON.stringify(result));
}).catch(error => { console.error(error); process.exitCode = 1; });
