fn main() {
    // Solve some exercises using primitives, functions and control flow
    // 1.
    let temp_in_f = 81.;
    let temp_in_c = 27.;

    println!(
        "{} fahrenheit equals {} celcius",
        temp_in_f,
        f_to_c(temp_in_f)
    );

    println!(
        "{} celcius equals {} hahrenheit",
        temp_in_c,
        c_to_f(temp_in_c)
    );

    // 2.
    for i in 0..9 {
        print!("{} ", nth_fibonacci(i));
    }
    println!();

    for i in 0..9 {
        print!("{} ", nth_fibonacci_recursive(i));
    }
    println!("\n");

    // 3.
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
    ];

    let mut current_strofe = String::from("");
    let mut strofe_counter = 0;
    for verse in lyrics {
        strofe_counter += 1;
        println!(
            "On the {} day of christmas, my true love sent to me:",
            strofe_counter
        );
        current_strofe = format!("{}\n{}", verse, current_strofe);
        println!("{}", current_strofe)
    }
}

fn f_to_c(temperature_in_f: f64) -> f64 {
    ((temperature_in_f - 32.) * 5. / 9.).round()
}

fn c_to_f(temperature_in_c: f64) -> f64 {
    (temperature_in_c * 9. / 5. + 32.).round()
}

fn nth_fibonacci(n: i8) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut _aux = 0;
    let mut first = 0;
    let mut second = 1;
    let mut counter = n;
    counter -= 2;
    while counter >= 0 {
        _aux = second;
        second += first;
        first = _aux;

        counter -= 1;
    }
    second
}

fn nth_fibonacci_recursive(n: i8) -> i32 {
    if n == 0 || n == 1 {
        return n as i32;
    }
    nth_fibonacci(n - 2) + nth_fibonacci(n - 1)
}
// fib 6
// fib 5 + fib 4
// fib 3 + fib 4 + fib 2 + fib 3
// 1 + fib 2 + fib 2 + fib 3 + 0 + 1 + 1 + fib 2
// 1 + 0 + 1 + 0 + 1 + 1 + fib 2 + 0 + 1 + 1 + 0 + 1
// 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
// 8
