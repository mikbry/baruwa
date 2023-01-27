//
const importObject = {
    module: {
        // TODO
    },
    env: {
        memory: new WebAssembly.Memory({ initial: 256 }),
    }
};

WebAssembly.instantiateStreaming(fetch("./app/baruwa.wasm"), importObject).then(result => {
    result.instance.exports.main();
});
