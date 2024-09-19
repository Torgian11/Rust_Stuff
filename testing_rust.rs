pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
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
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        let result = add(2,3);
        assert_eq!(result, 5);
    }

    #[test]
    fn rectangle_can_hold_smaller_rectangle() {
        let larger = Rectangle {
            width: 10,
            height: 15,
        };

        let smaller = Rectangle {
            width: 9,
            height: 10,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_rect_cannot_hold_larger_rect() {
        let larger = Rectangle {
            width: 10,
            height: 15,
        };

        let smaller = Rectangle {
            width: 9,
            height: 10,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn not_equal_two() {
        let result = add(1, 2);
        assert_ne!(result, 2);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting didn't contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn greater_than_1() {
        Guess::new(0);
    }

    #[test]
    fn it_works_two() -> Result<(), String> {
        let result = add(2,2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
