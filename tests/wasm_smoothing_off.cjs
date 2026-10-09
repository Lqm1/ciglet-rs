const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['median_filter', 'moving_average', 'moving_average_with', 'moving_rms', 'moving_rms_with']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`${operation}_${suffix}`)), 'smoothing absent without wasm');
}
console.log('Ciglet smoothing feature-disabled WASM: all10 functions absent');
