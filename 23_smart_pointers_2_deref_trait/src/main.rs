use std::ops::Deref;
fn main() {
    first_example_wrong_assert();
    second_example_with_box();
    third_example_ref_coercion();
}

fn first_example_wrong_assert() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    //assert_eq(5, y);
}

fn second_example_with_box() {
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    };

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    };

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));
    assert_eq!(5, *y);
}

fn third_example_ref_coercion() {
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    };

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    };

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}
