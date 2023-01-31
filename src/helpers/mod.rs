
// What follows are types defined to help communicate with Javascript code.
pub mod js {
    extern "C" {

    }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct JSObject(pub u32);

impl JSObject {
    pub const fn null() -> Self {
        JSObject(0)
    }
}
