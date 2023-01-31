mod graphics;
mod helpers;

use std::{rc::Rc, cell::RefCell};

use graphics::{gl, gl::GLEnum, glrs, winit::{winit::console_log, winit}};

use crate::helpers::JSObject;

#[no_mangle]
pub extern "C" fn main() {
    console_log("Hello, world!");
    unsafe { start() };
}

unsafe fn start() {
    // Create the vertex buffer
    let vertex_buffer: JSObject = gl::create_buffer();
    let index_buffer: JSObject = gl::create_buffer();
    gl::bind_buffer(GLEnum::ArrayBuffer, vertex_buffer);
    glrs::buffer_data_f32(
        GLEnum::ArrayBuffer,
        &[-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0],
        GLEnum::StaticDraw,
    );

    // Create the index buffer
    gl::bind_buffer(GLEnum::ElementArrayBuffer, index_buffer);
    glrs::buffer_data_u16(GLEnum::ElementArrayBuffer, &[0, 1, 2], GLEnum::StaticDraw);

    // Create the vertex shader
    let vertex_shader = gl::create_shader(GLEnum::VertexShader);
    glrs::shader_source(
        vertex_shader,
        r#"
                attribute vec3 vertex_position;
    
                void main(void) {
                    gl_Position = vec4(vertex_position, 1.0);
                }
            "#,
    );
    gl::compile_shader(vertex_shader);

    // Create the fragment shader
    let fragment_shader = gl::create_shader(GLEnum::FragmentShader);
    glrs::shader_source(
        fragment_shader,
        r#"void main() {
                    gl_FragColor = vec4(1.0, 0.5, 0.313, 1.0);
                  }
              "#,
    );
    gl::compile_shader(fragment_shader);

    // Create the shader program
    let shader_program = gl::create_program();
    gl::attach_shader(shader_program, vertex_shader);
    gl::attach_shader(shader_program, fragment_shader);
    gl::link_program(shader_program);


    // Get the location of the vertex_position attribute.
    // This is needed to later bind the data to it.
    let attrib_location = Box::new(glrs::get_attrib_location(shader_program, "vertex_position").unwrap());

    // Enable the attribute location.
    // This only needs to be done once per program, it's really just boilerplate.
    gl::enable_vertex_attrib_array(*attrib_location.to_owned());
    gl::vertex_attrib_pointer(*attrib_location.to_owned() as u32, 3, GLEnum::Float, false, 0, 0);

    let state = Rc::new(RefCell::new(shader_program));
    winit::set_main_loop(Box::new(move || {
        console_log("refresh");
        unsafe {
            gl::clear_color(0.3765, 0.3137, 0.8627, 1.0);
            gl::clear(GLEnum::ColorBufferBit);
            let shader_program = state.borrow_mut();
            gl::use_program(*shader_program);
            // gl::bind_buffer(GLEnum::ArrayBuffer, vertex_buffer);
            // gl::bind_buffer(GLEnum::ElementArrayBuffer, index_buffer);
            gl::draw_elements(GLEnum::Triangles, 3, GLEnum::UnsignedShort, 0);
        }
        console_log("update");
    }));
}

