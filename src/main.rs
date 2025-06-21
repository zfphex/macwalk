#![allow(non_snake_case)]
// Import necessary modules from the standard library.
use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

mod macros;
use macros::*;

mod class;
use class::*;

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

// Helper to fetch an Objective-C class by its Rust `&str` name.
#[allow(dead_code)]
fn get_class(name: &str) -> Id {
    let c_name = CString::new(name).expect("CString::new failed");
    unsafe { objc_getClass(c_name.as_ptr() as *const c_char) }
}

pub fn main() {
    // Get the directory path from command-line arguments, or use "." as a default.
    let args: Vec<String> = env::args().collect();
    let path_str = if args.len() > 1 { &args[1] } else { "." };

    println!("Listing contents of directory: \"{}\"", path_str);

    // All interactions with the Objective-C runtime are inherently unsafe.
    unsafe {
        // Create a C-style string for the path.
        let c_path = CString::new(path_str).expect("CString::new failed");
        let ns_path_string = NSString::new(c_path.as_ptr());

        // Get the default `NSFileManager` instance
        let file_manager = NSFileManager::default();

        // We pass `ptr::null_mut()` for the error pointer, as we are not handling it.
        let directory_contents: Id =
            file_manager.contentsOfDirectory(ns_path_string.0, ptr::null_mut());

        // --- 3. Iterate over the resulting `NSArray` and print the contents ---
        if !directory_contents.is_null() {
            // Get the number of items in the array.
            let len = count(directory_contents);

            // Loop through the array.
            for i in 0..len {
                // Get the object (an `NSString`) at the current index.
                let item = objectAtIndex(directory_contents, i);

                // Get the C string pointer from the `NSString`.
                let c_str_ptr = UTF8String(item);

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
