unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;
    let p1 = &raw const num;
    let p2 = &raw mut num;
    // dereferencing has to be done in an unsafe block
    println!("{:?} {:?}", p1, p2);

    unsafe {
        println!("{:?} {:?}", *p1, *p2);
    }

    println!("{}", abs(-3));
}
