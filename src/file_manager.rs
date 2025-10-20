use crate::*;
use core::mem::transmute;
use std::{
    borrow::Cow,
    ffi::{c_char, c_ulong, CStr},
};

// #[derive(Debug, Clone, Copy)]
// pub struct NSString(pub Id);

// impl NSString {
//     pub unsafe fn new(utf8: *const c_char) -> Self {
//         unsafe {
//             let sel = register_name("stringWithUTF8String:");
//             let cls = get_class("NSString");
//             let msg_send: unsafe extern "C" fn(Id, Selector, *const c_char) -> Id =
//                 transmute(objc_msgSend as *const ());
//             Self(msg_send(cls, sel, utf8))
//         }
//     }
// }

pub fn NSString(str: &str) -> Id {
    unsafe {
        let sel = register_name("stringWithUTF8String:");
        let cls = get_class("NSString");
        let c_str = CString::new(str).unwrap();
        let msg_send: unsafe extern "C" fn(Id, Selector, *const c_char) -> Id =
            transmute(objc_msgSend as *const ());
        msg_send(cls, sel, c_str.as_ptr() as *const c_char)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NSFileManager(pub Id);

impl NSFileManager {
    pub unsafe fn default() -> Self {
        unsafe {
            let sel = register_name("defaultManager");
            let class = get_class("NSFileManager");
            let msg_send: unsafe extern "C" fn(Id, Selector) -> Id =
                transmute(objc_msgSend as *const ());
            NSFileManager(msg_send(class, sel))
        }
    }

    pub unsafe fn contentsOfDirectory(&self, path: Id, error: *mut Id) -> Id {
        unsafe {
            let sel = register_name("contentsOfDirectoryAtPath:error:");
            let msg_send: unsafe extern "C" fn(Id, Selector, Id, *mut Id) -> Id =
                transmute(objc_msgSend as *const ());
            msg_send(self.0, sel, path, error)
        }
    }
}

pub unsafe fn count(array: Id) -> c_ulong {
    let sel = register_name("count");
    let msg_send: unsafe extern "C" fn(Id, Selector) -> c_ulong =
        transmute(objc_msgSend as *const ());
    msg_send(array, sel)
}

pub unsafe fn objectAtIndex(array: Id, idx: c_ulong) -> Id {
    let sel = register_name("objectAtIndex:");
    let msg_send: unsafe extern "C" fn(Id, Selector, c_ulong) -> Id =
        transmute(objc_msgSend as *const ());
    msg_send(array, sel, idx)
}

// pub unsafe fn UTF8String(nsstring: Id) -> *const c_char {
//     let sel = register_name("UTF8String");
//     let msg_send: unsafe extern "C" fn(Id, Selector) -> *const c_char =
//         transmute(objc_msgSend as *const ());
//     msg_send(nsstring, sel)
// }

pub unsafe fn UTF8String<'a>(nsstring: Id) -> Cow<'a, str> {
    let sel = register_name("UTF8String");
    let msg_send: unsafe extern "C" fn(Id, Selector) -> *const c_char =
        transmute(objc_msgSend as *const ());
    let c_str_ptr = msg_send(nsstring, sel);
    assert!(!c_str_ptr.is_null());
    CStr::from_ptr(c_str_ptr).to_string_lossy()
}
