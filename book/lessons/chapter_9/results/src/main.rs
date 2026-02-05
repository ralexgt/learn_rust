use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn Error>> {
    // let file = match File::open("hello.txt") {
    // Ok(file) => file,
    // Err(error) => match error.kind() {
    // ErrorKind::NotFound => match File::create("hello.txt") {
    // Ok(file) => file,
    // Err(error) => panic!("Error creating the file: {}", error),
    // },
    // _ => panic!("Error openning the file: {}", error),
    // },
    // };
    //
    let file = File::open("hello.txt")?;

    Ok(())
}
