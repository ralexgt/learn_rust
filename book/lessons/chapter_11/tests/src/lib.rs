pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub struct Guess {
    _i: i32,
}

impl Guess {
    pub fn new(i: i32) -> Guess {
        if !(1..100).contains(&i) {
            panic!("Value should be between 1 and 100");
        }
        Guess { _i: i }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        let result = add(2, 3);
        assert!(result == 5);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 6,
            height: 8,
        };
        let smaller = Rectangle {
            width: 3,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 6,
            height: 8,
        };
        let smaller = Rectangle {
            width: 3,
            height: 4,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain the name, value was \"{}\"",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Value should be between 1 and 100")]
    fn guess_outside_of_bounds() {
        Guess::new(200);
    }

    #[test]
    fn addition() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("Crazy"))
        }
    }
}
