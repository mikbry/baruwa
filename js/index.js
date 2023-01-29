//
import setupWebGL from './webgl.js';

(async () => {
    const wasm = {
        memory: undefined,
        decoder: new TextDecoder(),
        jsObjects: [],
    }
    const webGL = setupWebGL(() => wasm);
    const importedModules = {
        env: {
            memory: new WebAssembly.Memory({ initial: 256 }),
            ...webGL,
        }
    };

    const result = await WebAssembly.instantiateStreaming(fetch("./app/baruwa.wasm"), importedModules);
    wasm.memory = result.instance.exports.memory;
    result.instance.exports.main();
})();

