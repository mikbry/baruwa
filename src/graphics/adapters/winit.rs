
pub mod winit {
    use std::{ffi::c_void, cell::RefCell};

    thread_local!(static MAIN_LOOP_CALLBACK: RefCell<Option<Box<dyn FnMut()>>>  = RefCell::new(None));

    extern "C" {
        pub fn startMainLoop();
        pub fn consoleLog(pointer: *const c_void, pointer_length: usize);
    }

    pub fn set_main_loop(callback: Box<dyn Fn() -> ()>) {
        MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = Some(Box::new(callback)); 
        });
        unsafe {
            startMainLoop();
        }
    }

    pub fn console_log(log: &str) {
        unsafe {
            consoleLog(log as *const str as *const c_void, log.len());
        }
    }

    #[no_mangle]
    pub extern "C" fn main_loop() {
        console_log("main_loop Rust");
        MAIN_LOOP_CALLBACK.with(|z| {
            if let Some(ref mut callback) = *z.borrow_mut() {
                callback();
            }
        });
    }
}