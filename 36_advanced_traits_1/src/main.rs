use std::ops::Add;

fn main() {
    associated_types();
    default_generic_type_parameters();
    calling_methods_with_the_same_name();
}

fn associated_types() {
    println!("ASSOCIATED TYPES");

    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter;
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(0)
        }
    }

    println!("");
}

fn default_generic_type_parameters() {
    println!("DEFAULT GENERIC TYPE PARAMETERS");

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 1 } + Point { x: 1, y: 1 },
        Point { x: 2, y: 2 },
    );

    println!("")
}

fn calling_methods_with_the_same_name() {
    println!("CALLING METHODS WITH THE SAME NAME");
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly();
    }

    struct Human;
    impl Human {
        fn fly(&self) {
            println!("Waving arms furiously")
        }
    }

    impl Pilot for Human {
        fn fly(&self) {
            println!("Pilot flying");
        }
    }
    impl Wizard for Human {
        fn fly() {
            println!("Up");
        }
    }

    let person = Human {};
    person.fly();
    Pilot::fly(&person);
    <Human as Wizard>::fly();

    println!("");
}
