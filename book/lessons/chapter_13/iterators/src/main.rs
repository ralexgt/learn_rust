fn main() {
    let v1 = [3, 2, 7, 5, 19, 10];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got value: {}", val);
    }
}
