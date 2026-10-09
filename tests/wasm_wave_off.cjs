const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /wave_read_|wave_write_pcm_|wavef32|wavef64|wavebytes|wavewriteoptions/i.test(name)), 'all WAVE functions and owners absent without wasm');
console.log('Ciglet WAVE feature-disabled WASM: eight functions and four owners absent');
