// WASM bindings to WebGL Javascript

pub mod gl {
    use std::ffi::c_void;
    use crate::helpers::JSObject;

    extern "C" {
        pub fn create_buffer() -> JSObject;
        pub fn bind_buffer(target: GLEnum, gl_object: JSObject);
        pub fn buffer_data_f32(
            target: GLEnum,
            data: *const c_void,
            data_length: usize,
            usage: GLEnum,
        );

        pub fn buffer_data_u16(
            target: GLEnum,
            data: *const c_void,
            data_length: usize,
            usage: GLEnum,
        );

        pub fn create_shader(shader_type: GLEnum) -> JSObject;
        pub fn shader_source(shader: JSObject, source: *const c_void, source_length: usize);
        pub fn compile_shader(shader: JSObject);
        pub fn create_program() -> JSObject;
        pub fn attach_shader(program: JSObject, shader: JSObject);
        pub fn link_program(program: JSObject);
        pub fn use_program(program: JSObject);
        pub fn get_attrib_location(
            program: JSObject,
            name: *const c_void,
            name_length: usize,
        ) -> GLint;
        pub fn vertex_attrib_pointer(
            index: GLUint,
            size: GLint,
            _type: GLEnum,
            normalized: bool,
            stride: GLsizei,
            pointer: GLintptr,
        );
        pub fn enable_vertex_attrib_array(index: GLUint);
        pub fn clear_color(r: f32, g: f32, b: f32, a: f32);
        pub fn clear(mask: GLEnum);
        pub fn draw_elements(mode: GLEnum, count: GLsizei, _type: GLEnum, offset: GLintptr);
    }

    // Sourced from here: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants
    #[repr(u32)]
    pub enum GLEnum {
        Triangles = 0x0004,
        ArrayBuffer = 0x8892,
        ElementArrayBuffer = 0x8893,
        VertexShader = 0x8B31,
        FragmentShader = 0x8B30,
        Byte = 0x1400,
        UnsignedByte = 0x1401,
        Short = 0x1402,
        UnsignedShort = 0x1403,
        Int = 0x1404,
        UnsignedInt = 0x1405,
        Float = 0x1406,
        StaticDraw = 0x88E4,
        DynamicDraw = 0x88E8,
        ColorBufferBit = 0x00004000,
    }

    // Define some types to make it easier to port directly from the spec.
    pub type GLUint = u32;
    pub type GLint = i32;
    pub type GLsizei = i32;

    // GLintptr should be an i64, but Wasm can't pass those so for now just use an i32.
    pub type GLintptr = i32;
}

pub mod glrs {
    use std::ffi::c_void;
    use crate::helpers::JSObject;
    use super::gl::{self, GLUint, GLEnum};
    
    pub fn shader_source(shader: JSObject, source: &str) {
        unsafe { gl::shader_source(shader, source as *const str as *const c_void, source.len()) }
    }
    
    pub fn get_attrib_location(program: JSObject, name: &str) -> Option<GLUint> {
        unsafe {
            // Returns -1 if attribute does not exist, to be more Rusty we'll return None instead.
            // A GLUint is returned instead because all APIs that consume this value expect a GLUint.
            // getAttribLocation only returns a signed value to use the -1 to indicate if the location
            // does not exist.
            let result =
                gl::get_attrib_location(program, name as *const str as *const c_void, name.len());
            if result == -1 {
                None
            } else {
                Some(result as u32)
            }
        }
    }
    
    pub fn bind_buffer(target: GLEnum, gl_object: Option<JSObject>) {
        unsafe { gl::bind_buffer(target, gl_object.unwrap_or(JSObject::null())) }
    }
    
    pub fn buffer_data_f32(target: GLEnum, data: &[f32], usage: GLEnum) {
        unsafe {
            gl::buffer_data_f32(
                target,
                data as *const [f32] as *const c_void,
                data.len(),
                usage,
            )
        }
    }
    
    pub fn buffer_data_u16(target: GLEnum, data: &[u16], usage: GLEnum) {
        unsafe {
            gl::buffer_data_u16(
                target,
                data as *const [u16] as *const c_void,
                data.len(),
                usage,
            )
        }
    } 
}