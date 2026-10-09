const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['fetch_frame', 'extremum', 'peak']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`${operation}_${suffix}`)), 'frame/extrema absent without wasm');
}
console.log('Ciglet frame/extrema feature-disabled WASM: all6 functions absent');
