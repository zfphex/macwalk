#![allow(unused)]
use macwalk::*;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_str = if args.len() > 1 { &args[1] } else { "." };

    println!("Listing contents of directory: \"{}\"", path_str);

    unsafe {
        recursive_walk(path_str, 0);
    }

    return;

    unsafe {
        let ns_path_string = NSString(path_str);
        let file_manager = NSFileManager::default();

        //Why tf does macos return a null pointer when the directory doesn't exist?
        let directory = file_manager.contentsOfDirectory(ns_path_string, core::ptr::null_mut());
        assert!(!directory.is_null());

        let len = count(directory);
        for i in 0..len {
            let id = objectAtIndex(directory, i);
            let item = UTF8String(id);
            println!("- {}", item);
        }
    }
}

pub unsafe fn recursive_walk(path: &str, depth: usize) {
    let ns_path_string = NSString(path);
    let file_manager = NSFileManager::default();

    let directory = file_manager.contentsOfDirectory(ns_path_string, core::ptr::null_mut());
    if directory.is_null() {
        return;
    }

    let len = count(directory);
    for i in 0..len {
        let id = objectAtIndex(directory, i);
        let item = UTF8String(id);

        // Print with indentation based on depth
        for _ in 0..depth {
            print!("  ");
        }
        println!("- {}", item);

        // Build the full path and recursively traverse if it's a directory
        let full_path = format!("{}/{}", path, item);

        // You may want to add a check to see if it's actually a directory
        // to avoid trying to traverse files
        if is_directory(&full_path) {
            recursive_walk(&full_path, depth + 1);
        }
    }
}

// Helper function to check if a path is a directory
pub unsafe fn is_directory(path: &str) -> bool {
    let ns_path = NSString(path);
    let file_manager = NSFileManager::default();

    // You'd need to add a method to check file attributes
    // For now, you could try calling contentsOfDirectory and checking if it's null
    let contents = file_manager.contentsOfDirectory(ns_path, core::ptr::null_mut());
    !contents.is_null()
}
