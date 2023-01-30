
// What follows are types defined to help communicate with Javascript code.

#[derive(Clone, Copy)]
#[repr(C)]
pub struct JSObject(u32);

impl JSObject {
    pub const fn null() -> Self {
        JSObject(0)
    }
}