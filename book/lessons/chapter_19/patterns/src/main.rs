struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("idk man"),
    };

    let x = Some(10);
    let _y = 20;

    match x {
        Some(2) => println!("1"),
        Some(y) => println!("{y}"),
        _ => println!("_"),
    };

    let p = Point { x: 1, y: 2 };
    let Point { x: _a, y: _b } = p;
}
