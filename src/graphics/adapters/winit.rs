
pub mod winit {
    use std::ffi::c_void;

    extern "C" {
        pub fn startMainLoop(callback: *const c_void);
        pub fn consoleLog(pointer: *const c_void, pointer_length: usize);
    }

    pub fn set_main_loop(callback: &dyn Fn() -> ()) {
        unsafe {
            let ptr = Box::into_raw(Box::new(callback)) as *const c_void;
            startMainLoop(ptr);
        }
    }

    pub fn console_log(log: &str) {
        unsafe {
            consoleLog(log as *const str as *const c_void, log.len());
        }
    }

    #[no_mangle]
    pub extern "C" fn main_loop(ptr: *const c_void) -> *const c_void {
        let callback = unsafe { Box::from_raw(ptr as *mut Box<dyn Fn() -> ()>) };
        let mut callback = *callback;
        let callback = &mut *callback;
        callback();
        console_log("main_loop Rust");
        Box::into_raw(Box::new(callback)) as *const c_void
    }
}