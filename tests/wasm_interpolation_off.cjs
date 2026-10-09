const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['linear', 'uniform', 'sinc', 'sinc_with', 'sinc_with_kernel', 'fill_missing']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`interpolate_${operation}_${suffix}`)), 'interpolation absent without wasm');
}
console.log('Ciglet interpolation feature-disabled WASM: all12 functions absent');
