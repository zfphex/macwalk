#![allow(non_snake_case)]

use core::ffi::c_char;
use std::ffi::CString;

pub mod file_manager;
pub use file_manager::*;

#[link(name = "objc", kind = "dylib")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

pub type Id = *mut ObjcObject;
pub type Selector = *const std::os::raw::c_void;

// An opaque struct representing an Objective-C object.
#[repr(C)]
pub struct ObjcObject {
    _private: [u8; 0],
}

extern "C" {
    /// Gets a reference to a class by its name.
    fn objc_getClass(name: *const c_char) -> Id;

    /// Registers a method name and returns a selector.
    fn sel_registerName(name: *const c_char) -> Selector;

    /// The core function for sending messages (calling methods) on Objective-C objects.
    /// It's a variadic function, so we will cast its pointer to the correct
    /// function signature before calling it.
    fn objc_msgSend(receiver: Id, selector: Selector, ...);
}

//TODO: Any way to compile time add a \0 to these?
//Each parameter must end with a colon `par1:par2:...:`
//Empty parameters don't require a colon.
pub fn register_name(name: &str) -> Selector {
    let c_name = CString::new(name).unwrap();
    unsafe { sel_registerName(c_name.as_ptr() as *const c_char) }
}

pub fn get_class(name: &str) -> Id {
    let c_name = CString::new(name).unwrap();
    unsafe { objc_getClass(c_name.as_ptr() as *const c_char) }
}
