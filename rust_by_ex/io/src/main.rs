use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    let path = Path::new("hello.txt");
    let path_display = path.display();

    let mut file_read = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Couldnt get: {}", e),
    };

    let mut s = String::new();
    match file_read.read_to_string(&mut s) {
        Ok(_) => println!("Read\n{}into s", s),
        Err(e) => panic!("Couldn't read {}: {}", path_display, e),
    }

    let mut file_write = match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("Couldnt get: {}", e),
    };

    match file_write.write_all(LOREM_IPSUM.as_bytes()) {
        Ok(_) => println!("Successfully wrote to {}", path_display),
        Err(e) => panic!("Can't write to {}: {}", path_display, e),
    }

    match file_write.read_to_string(&mut s) {
        Ok(_) => println!("Read\n{}into s", s),
        Err(e) => panic!("Couldn't read {}: {}", path_display, e),
    }
}
