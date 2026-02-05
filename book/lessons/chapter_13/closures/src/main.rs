use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut count_blue = 0;
        let mut count_red = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Blue => count_blue += 1,
                ShirtColor::Red => count_red += 1,
            }
        }

        if count_blue > count_red {
            return ShirtColor::Blue;
        }
        ShirtColor::Red
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} wins {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} wins {:?}",
        user_pref2, giveaway2
    );

    let _expensive_closure = |num: i32| -> i32 {
        println!("Slowly calculating");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let example_closure = |x| x;
    println!("{}", example_closure(3));

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || list.push(4);
    borrows_mutably();
    println!("After borrowing mutably: {list:?}");

    let mut list = vec![
        Rectangle {
            width: 2,
            height: 3,
        },
        Rectangle {
            width: 4,
            height: 2,
        },
        Rectangle {
            width: 1,
            height: 10,
        },
        Rectangle {
            width: 7,
            height: 21,
        },
    ];
    println!("{:?}", list);
    list.sort_by_key(|r| r.width * r.height);
    println!("{:?}", list);

    let mut num_of_operations = 0;
    list.sort_by_key(|r| {
        num_of_operations += 1;
        r.width * r.height
    });
    println!("Sorted in {} operations", num_of_operations);
}
