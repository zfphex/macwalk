use macwalk::*;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_str = if args.len() > 1 { &args[1] } else { "." };

    println!("Listing contents of directory: \"{}\"", path_str);

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
