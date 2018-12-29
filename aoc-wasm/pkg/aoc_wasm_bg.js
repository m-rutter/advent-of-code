const path = require("path").join(__dirname, "aoc_wasm_bg.wasm");
const bytes = require("fs").readFileSync(path);
let imports = {};
imports["./aoc_wasm"] = require("./aoc_wasm");

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
