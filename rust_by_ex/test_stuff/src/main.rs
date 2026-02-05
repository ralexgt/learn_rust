fn main() {
    if cfg!(target_os = "linux") {
        println!("You are running linux!");
    } else {
        println!("You are not running linux :(");
    }
}
