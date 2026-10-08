// Verify the optional bindings are absent from an actual feature-disabled module.
const fs = require('node:fs');
const assert = require('node:assert/strict');
const moduleBytes = fs.readFileSync(process.argv[2]);
const artifact = new WebAssembly.Module(moduleBytes);
const exportsFound = WebAssembly.Module.exports(artifact).map(entry => entry.name);
for (const name of ['window_f32', 'window_f64', 'window_custom_f32_f32', 'window_custom_f32_f64', 'window_custom_f64_f32', 'window_custom_f64_f64']) {
    assert(!exportsFound.some(entry => entry.includes(name)), name + ' must be absent without wasm');
}
console.log('Ciglet windows feature-disabled WASM: all6 exports absent');
