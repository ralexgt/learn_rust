mod front_of_house {
    fn test_in_front() {
        crate::front_of_house::serving::take_order();
        let meal = super::back_of_house::Breakfast::summer("tra");
    }
    mod hosting {
        fn add_to_waitlist() {}
        pub fn seat_at_table() {
            crate::front_of_house::test_in_front();
        }
    }

    mod serving {
        use crate::testrr as blabla;
        pub fn take_order() {}
        fn serve_order() {
            blabla();
        }
        fn take_payment() {
            crate::front_of_house::test_in_front();
            crate::front_of_house::hosting::seat_at_table();
            serve_order();
        }
    }
}

fn testrr() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}

use rand::Rng;
use rand::thread_rng;
pub fn lala() {
    let secret_number = thread_rng().gen_range(1.3..=100.);
    println!("{}", secret_number);
    println!("{}", restaurant::trei());
}
