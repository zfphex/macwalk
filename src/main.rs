#![allow(unused)]
use macwalk::*;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    println!("Listing contents of directory: \"{}\"", path);
    recursive_walk(path);
}

fn recursive_walk(path: &str) {
    unsafe {
        let ns_path_string = NSString(path);
        let file_manager = NSFileManager::default();
        let enumerator = file_manager.enumerator(ns_path_string);
        assert!(!enumerator.is_null());

        let mut file_id = nextObject(enumerator);
        while !file_id.is_null() {
            let item = UTF8String(file_id);
            println!("- {}", item);
            file_id = nextObject(enumerator);
        }
    }
}

fn list_directory(path: &str) {
    unsafe {
        let ns_path_string = NSString(path);
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
