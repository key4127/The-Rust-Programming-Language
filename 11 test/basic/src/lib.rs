pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("less than 1.");
        } else if value > 100 {
            panic!("more than 100.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test add
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // assert_eq!(4, result) will get the same output
        assert_ne!(result, 5); // these two functions acquire the PartialEq and Debug traits
    }

    // test can_hold
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // test greeting
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // test guess
    // if it did not panic, the test will fail
    #[test]
    #[should_panic(expected = "more than 100.")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // test add use Result<T,E>
    #[test]
    fn another_it_works() -> Result<(), String>{
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
