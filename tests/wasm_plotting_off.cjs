const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /plot_color_from_character|plotcommands/i.test(name)), 'plot color and command owner absent without wasm');
console.log('Ciglet plotting feature-disabled WASM: color function and command owner absent');
