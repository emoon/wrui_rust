extern crate libloading;
mod ffi;

use libloading::{Library, Symbol};
use std::rc::Rc;

pub struct Wrui {
    lib: Rc<libloading::Library>,
    c_api: *const ffi::Wrui,
    wu_app: *const ffi::WUApplication,
}

pub struct Window {
    pub wu_window: *const ffi::WUWindow,
    pub c_api: *const ffi::Wrui,
}

#[macro_export]
macro_rules! window_set_paint_event {
    ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
        {
            extern "C" fn temp_paint_event(target: *mut std::os::raw::c_void) {
                unsafe {
                    let app = target as *mut $call_type;
                    $callback(&mut *app);
                }
            }

            unsafe {
                let window_funcs = (*(*$sender.c_api).window_funcs);
                window_funcs.set_paint_event.unwrap()($sender.wu_window, std::mem::transmute($data), temp_paint_event);
            }
        }
    }
}

impl Wrui {
    pub fn new(shared_lib: &str) -> Option<Wrui> {
        let lib = Rc::new(Library::new(shared_lib).unwrap());
        unsafe {
            let wrui_get: Symbol<unsafe extern fn() -> *const ffi::Wrui> =
                lib.get(b"wrui_get\0").unwrap();
    
            let c_api = wrui_get();

            Some(Wrui {
                lib: lib.clone(),
                c_api: wrui_get(),
                wu_app: (*c_api).application_create.unwrap()(),
            })
        }
    }

    pub fn create_window(&self) -> Window {
        let wu_window = unsafe { 
            (*self.c_api).window_create.unwrap()(std::ptr::null_mut()) 
        };

        Window {
            wu_window: wu_window,
            c_api: self.c_api,
        }
    }

    pub fn run(&self) {
        unsafe {
            (*(*self.c_api).application_funcs).run.unwrap()(self.wu_app);
        }
    }
}

