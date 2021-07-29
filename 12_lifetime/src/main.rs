use std::fmt::Display;

fn main() {
    first_example();
    second_example();
    third_example();
    fourth_example();
    fifth_example();
}

fn first_example() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
fn second_example() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
}

fn third_example() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
fn fourth_example() {
    // Life time Roles

    // 1 - Each parameter that is a reference gets irs own lifetime parameter

    // 2 - if there is exactly one input lifetime parameter, that lifetime is
    // assigned to all output lifetime parameters

    // 3 - If there are multiple input lifetime parameters, but one of them is
    // &self or &mut self the lifetime of self is assigned to all output
    // lifetime parameters

    // OBS: static lifetime tell the reference could live as long the duration of our program
    fn first_world(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}

fn fifth_example() {
    fn longest_with_an_annuncement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
