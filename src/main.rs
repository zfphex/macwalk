#![allow(unused, non_snake_case)]
// Import necessary modules from the standard library.
use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_ulong};
use std::ptr;

mod macros;
use macros::*;

// Link to the Objective-C runtime and the Foundation framework.
#[link(name = "objc", kind = "dylib")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

// Define aliases for core Objective-C types for clarity.
// `Id` is a pointer to any Objective-C object.
type Id = *mut ObjcObject;
// `SEL` is a pointer to the name of a method.
type Selector = *const std::os::raw::c_void;

// An opaque struct representing an Objective-C object.
#[repr(C)]
struct ObjcObject {
    _private: [u8; 0],
}

// FFI declarations for the Objective-C runtime functions we will use.
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

pub fn main() {
    // Get the directory path from command-line arguments, or use "." as a default.
    let args: Vec<String> = env::args().collect();
    let path_str = if args.len() > 1 { &args[1] } else { "." };

    println!("Listing contents of directory: \"{}\"", path_str);

    // All interactions with the Objective-C runtime are inherently unsafe.
    unsafe {
        // --- 1. Create an `NSString` from our Rust string path ---

        // Get the `NSString` class.
        let nsstring_class = objc_getClass(b"NSString\0".as_ptr() as *const c_char);

        // Create a C-style string for the path.
        let c_path = CString::new(path_str).expect("CString::new failed");

        // Get the selector for the `stringWithUTF8String:` class method.
        let sel_with_utf8 = sel_registerName(b"stringWithUTF8String:\0".as_ptr() as *const c_char);

        // Cast `objc_msgSend` to the correct function signature for this call.
        let msg_send_fn: unsafe extern "C" fn(Id, Selector, *const c_char) -> Id =
            std::mem::transmute(objc_msgSend as *const ());

        // Send the message to create the `NSString` object.
        let ns_path_string: Id = msg_send_fn(nsstring_class, sel_with_utf8, c_path.as_ptr());

        // --- 2. Get the default `NSFileManager` instance ---

        // Get the `NSFileManager` class.
        let nsfilemanager_class = objc_getClass(b"NSFileManager\0".as_ptr() as *const c_char);

        // Get the selector for the `defaultManager` class method.
        let sel_default_manager = sel_registerName(b"defaultManager\0".as_ptr() as *const c_char);

        // Cast `objc_msgSend` for this call.
        let msg_send_fn: unsafe extern "C" fn(Id, Selector) -> Id =
            std::mem::transmute(objc_msgSend as *const ());

        // Send the message to get the singleton file manager instance.
        let file_manager: Id = msg_send_fn(nsfilemanager_class, sel_default_manager);

        // --- 3. Call `contentsOfDirectoryAtPath:error:` ---

        // Get the selector for `contentsOfDirectoryAtPath:error:`.
        let sel_contents_at_path =
            sel_registerName(b"contentsOfDirectoryAtPath:error:\0".as_ptr() as *const c_char);

        // Cast `objc_msgSend` for this call.
        // The return is an `NSArray`, which is also an `Id`.
        let msg_send_fn: unsafe extern "C" fn(Id, Selector, Id, Id) -> Id =
            std::mem::transmute(objc_msgSend as *const ());

        // We pass `ptr::null_mut()` for the error pointer, as we are not handling it.
        let directory_contents: Id = msg_send_fn(
            file_manager,
            sel_contents_at_path,
            ns_path_string,
            ptr::null_mut(),
        );

        // --- 4. Iterate over the resulting `NSArray` and print the contents ---

        if !directory_contents.is_null() {
            // Get the number of items in the array.
            let sel_count = sel_registerName(b"count\0".as_ptr() as *const c_char);
            let msg_send_fn: unsafe extern "C" fn(Id, Selector) -> c_ulong =
                std::mem::transmute(objc_msgSend as *const ());
            let count = msg_send_fn(directory_contents, sel_count);

            // Get selectors for `objectAtIndex:` and `UTF8String`.
            let sel_object_at_index =
                sel_registerName(b"objectAtIndex:\0".as_ptr() as *const c_char);
            let sel_utf8_string = sel_registerName(b"UTF8String\0".as_ptr() as *const c_char);

            // Loop through the array.
            for i in 0..count {
                // Get the object (an `NSString`) at the current index.
                let msg_send_fn: unsafe extern "C" fn(Id, Selector, c_ulong) -> Id =
                    std::mem::transmute(objc_msgSend as *const ());
                let obj_at_index = msg_send_fn(directory_contents, sel_object_at_index, i);

                // Get the C string pointer from the `NSString`.
                let msg_send_fn: unsafe extern "C" fn(Id, Selector) -> *const c_char =
                    std::mem::transmute(objc_msgSend as *const ());
                let c_str_ptr = msg_send_fn(obj_at_index, sel_utf8_string);

                // Convert the C string to a Rust string slice and print it.
                if !c_str_ptr.is_null() {
                    let rust_str = CStr::from_ptr(c_str_ptr).to_string_lossy();
                    println!("- {}", rust_str);
                }
            }
        } else {
            eprintln!("Failed to get directory contents. The path may be invalid or you may not have permissions.");
        }
    }
    // NOTE: Memory management is handled by Objective-C's Automatic Reference Counting (ARC).
    // The objects returned by the Foundation methods are autoreleased, so we don't need
    // to manually release them in this simple, short-lived program. In a more complex
    // or long-running application, managing autorelease pools would be necessary.
}
