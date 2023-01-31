//

const setup = (wasm) => {
    let canvas = document.getElementById("canvas");
    console.log("Setting up canvas");
    const gl = canvas.getContext("webgl2");
    if (!gl) {
        console.log("Failed to get WebGL context.");
    }

    return {
        setup,
        create_buffer: () => wasm().jsObjects.push(gl.createBuffer()) - 1,
        bind_buffer: (gl_enum, id) => {
            console.log('bindBuffer', id);
            gl.bindBuffer(gl_enum, wasm().jsObjects[id]);
        },
        buffer_data_f32: (target, pointer, length, usage) => {
            const data = new Float32Array(wasm().memory.buffer, pointer, length);
            gl.bufferData(target, data, usage);
        },
        buffer_data_u16: (target, pointer, length, usage) => {
            const data = new Uint16Array(wasm().memory.buffer, pointer, length);
            gl.bufferData(target, data, usage);
        },
        create_shader: (gl_enum) => wasm().jsObjects.push(gl.createShader(gl_enum)) - 1,
        compile_shader: (shader) => gl.compileShader(wasm().jsObjects[shader]),
        shader_source: (shader, pointer, length) => {
            const string_data = new Uint8Array(wasm().memory.buffer, pointer, length);
            const shader_string = wasm().decoder.decode(string_data);
            gl.shaderSource(wasm().jsObjects[shader], shader_string);
        },
        create_program: () => wasm().jsObjects.push(gl.createProgram()) - 1,
        attach_shader: (program, shader) => {
            gl.attachShader(wasm().jsObjects[program], wasm().jsObjects[shader]);
        },
        link_program: (program) => {
            gl.linkProgram(wasm().jsObjects[program]);
        },
        use_program: (program) => {
            console.log('glProgram', program, wasm().jsObjects[program]);
            gl.useProgram(wasm().jsObjects[program]);
        },
        get_attrib_location: (program, pointer, length) => {
            const string_data = new Uint8Array(wasm().memory.buffer, pointer, length);
            const string = wasm().decoder.decode(string_data);
            return gl.getAttribLocation(wasm().jsObjects[program], string);
        },
        vertex_attrib_pointer: (index, size, type, normalized, stride, offset) => gl.vertexAttribPointer(index, size, type, normalized, stride, offset),
        enable_vertex_attrib_array: (index) => gl.enableVertexAttribArray(index),
        clear_color: (r, g, b, a) => gl.clearColor(r, g, b, a),
        clear: (gl_enum) => gl.clear(gl_enum),
        draw_elements: (mode, count, type, offset) => gl.drawElements(mode, count, type, offset)
    };
}

export default setup;