//
import setupWebGL from './webgl.js';

const startMainLoop = (wasm) => {
    const { main_loop } =  wasm.instance.exports;
    console.log("start main_loop=", main_loop, wasm);
    const update = () => {
        console.log("js main_loop=", main_loop);
        main_loop();
        console.log('hop');
        window.requestAnimationFrame(update);
    };
    update();
};

const consoleLog = (wasm, pointer, length) => {
    let string = new Uint8Array(wasm.memory.buffer, pointer, length);
    string = wasm.decoder.decode(string);
    console.log(string);
};

(async () => {
    const wasm = {
        memory: undefined,
        decoder: new TextDecoder(),
        jsObjects: [],
        instance: undefined,
    }
    const webGL = setupWebGL(() => wasm);
    const importedModules = {
        env: {
            memory: new WebAssembly.Memory({ initial: 256 }),
            consoleLog: (pointer, length) => consoleLog(wasm, pointer, length),
            startMainLoop: () => startMainLoop(wasm),
            ...webGL,
        }
    };

    const result = await WebAssembly.instantiateStreaming(fetch("./app/baruwa.wasm"), importedModules);
    wasm.memory = result.instance.exports.memory;
    wasm.instance = result.instance;
    result.instance.exports.main();
})();
