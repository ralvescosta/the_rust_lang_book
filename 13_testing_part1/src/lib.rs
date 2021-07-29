#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add_two(n: i32) -> i32 {
    n + 2
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(
            larger.can_hold(&smaller),
            "Greeting did not contain name, value was `{}`",
            larger.height
        );
    }
    #[test]
    fn larger_cannot_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
