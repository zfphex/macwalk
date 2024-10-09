use objc2::{runtime::*, *};
use std::{
    ffi::{c_char, CStr, CString},
    path::{Path, PathBuf},
};

fn nsstring(string: &str) -> *mut AnyObject {
    let nsstring: *mut AnyObject = unsafe {
        let cstr = CString::new(string).unwrap();
        let nsstring: *mut AnyObject =
            msg_send![class!(NSString), stringWithUTF8String: cstr.as_ptr()];
        nsstring
    };
    nsstring
}

#[derive(Debug)]
pub struct DirEntry {
    pub name: &'static str,
    pub path: PathBuf,
}

fn walk(directory: &str) -> Result<Vec<DirEntry>, &'static str> {
    let root = Path::new(directory).canonicalize().unwrap();

    unsafe {
        let file_manager: *mut AnyObject = msg_send![class!(NSFileManager), defaultManager];
        let ns_path = nsstring(directory);
        //https://developer.apple.com/documentation/foundation/nsfilemanager/1414584-contentsofdirectoryatpath
        let contents: *mut AnyObject = msg_send![file_manager, contentsOfDirectoryAtPath: ns_path error: std::ptr::null::<*mut AnyObject>()];

        if contents.is_null() {
            return Err("Failed to get directory contents.");
        }

        let count: usize = msg_send![contents, count];

        Ok((0..count)
            .map(|i| {
                let item: *mut AnyObject = msg_send![contents, objectAtIndex: i];
                let c_str: *const c_char = msg_send![item, UTF8String];
                let item_str = CStr::from_ptr(c_str).to_str().unwrap();
                DirEntry {
                    name: item_str,
                    path: root.join(item_str),
                }
            })
            .collect())
    }
}

fn main() {
    let files = walk("../").unwrap();
    dbg!(files);
}
