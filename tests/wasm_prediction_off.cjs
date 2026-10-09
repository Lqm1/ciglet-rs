const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /prediction_levinson_|prediction_from_|prediction_gain_|prediction_resonances_|predictionf32|predictionf64|scaledprediction|predictionspectrumplan|predictionresonance|predictioncorrelationparts|predictiongainparts/i.test(name)), 'all prediction functions and owners absent without wasm');
console.log('Ciglet prediction feature-disabled WASM: twenty-four functions and twelve owners absent');
