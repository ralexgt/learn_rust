const COMPARED_TO: u8 = 5;

fn main() {
    let x = 3;

    if x > COMPARED_TO {
        println!("{} higher than {}", x, COMPARED_TO);
    } else {
        println!("{} lower than {}", x, COMPARED_TO);
    }

    let some_condition = true;
    let x = if some_condition { 1 } else { 0 };
    println!("{}", x);

    let mut counter = 0;
    loop {
        if counter == 4 {
            counter += 1;
            continue;
        }
        if counter % 2 == 0 {
            println!("{}", counter);
        }
        if counter > 10 {
            break;
        }

        counter += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);

    let mut counter = 3;
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }
    println!("START!!!!!");

    // INFO: Panics, index out of bounds
    // let arr = [1, 2, 3, 4, 5];
    // let mut idx = 0;
    // while idx < 7 {
    // println!("{}", arr[idx]);
    // idx += 1;
    // }

    // use for to itterate over a collection. Avoid user errors
    let arr = ["apples", "oranges", "banannas"];
    for item in arr {
        println!("{}", item);
    }

    // can use for loops instead of while for counting
    for i in (1..4).rev() {
        print!("{} ", i);
    }
    println!();
}
