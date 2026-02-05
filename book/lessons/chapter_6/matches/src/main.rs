#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(s) => {
            println!("{:?}", s);
            25
        }
    }
}

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = Coin::Penny;
    let _c2 = Coin::Nickel;
    let _c3 = Coin::Dime;
    let _c4 = Coin::Quarter;
    let value = value_in_cents(coin);
    println!("{}", value);

    let _state = UsState::Alaska;
    let _state = UsState::Alabama;

    let x = Some(5);
    let xplus = plus_one(x);
    let none = plus_one(None);
    println!("{:?} | {:?} | {:?}", x, xplus, none);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Config is set to be max {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("Config is set to be max {}", max);
    }
}
