const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    let mut x = 6;
    println!("{}", x);
    x = 5;
    println!("{}", x);
    println!("constant is: {}", THREE_HOURS_IN_SECONDS);

    println!("shadowing part");
    let x = 33;
    println!("x now is {}", x);
    let x = x / 11;
    {
        let x = x * 4;
        println!("inner scope x is {}", x);
    }
    println!("final x is {}", x);

    let tup: (i32, f64, char) = (3, 6.2, '@');
    println!("first el of tuple: {}", tup.0);
    let (_x, y, _z) = tup;
    println!("{}", y);

    let arr = [1, 2, 4, 3];
    println!("{}", arr[1]);

    print_function(6, 'a');

    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y);

    println!("{}", plus_one(7));
}

fn print_function(x: i32, c: char) {
    println!("Printed this: {{ {} }} and that: '{}'", x, c);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
