// Unite Test - In rust unit test live in the same fale of the production code
// Integration Test - Live in a folder in a root directory called test

#[cfg(test)]
mod tests_first {
    // There a arguments you can use in cargo cli when running test this arguments. To cargo test cli arguments we can see in (cargo test --help).
    // To cargo test binary output (cargo test -- --help)

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[cfg(test)]
mod tests_output {
    //cargo test -- --show-output
    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {}", a);
        a
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(4, value)
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value)
    }
}

mod tests_subset {
    // cargo test add // run all tests with start with add
    // cargo test tests_subset // run all test in mod tests_subset

    pub fn add_tow(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_tow(2))
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_tow(3))
    }

    // cargo test one_hundred
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_tow(100))
    }

    // cargo test -- --ignore // run all tests ignored
    #[test]
    #[ignore]
    fn expensive_test() {}
}
