const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['windowed_fir', 'fir', 'bandpass_fir', 'convolve', 'filter', 'filter_with', 'forward_backward_filter', 'forward_backward_filter_with']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`${operation}_${suffix}`)), 'filtering absent without wasm');
}
console.log('Ciglet filtering feature-disabled WASM: all16 functions absent');
