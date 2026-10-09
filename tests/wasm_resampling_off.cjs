const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['resample', 'resample_with', 'resample_with_kernel', 'resample_output_length']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`${operation}_${suffix}`)), 'resampling absent without wasm');
}
console.log('Ciglet resampling feature-disabled WASM: all8 functions absent');
