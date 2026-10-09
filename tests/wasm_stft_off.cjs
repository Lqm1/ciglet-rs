const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /stft_analyze_|stft_synthesize_|stftplan|stftanalysis|stftframegrid|stftsynthesis/i.test(name)), 'all STFT functions and owners absent without wasm');
console.log('Ciglet STFT feature-disabled WASM: six functions and eight owners absent');
