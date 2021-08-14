fn main() {
    // - We use box when we have a type and the size cant be know in compile time
    // - When we have a mount of data and we want to transfer the ownership and garanti the data is fully copy
    // - When we have a data with implement a specific trait (Trait Object)

    fist_example();
    when_we_have_a_type_with_we_dont_know_the_size_in_compile_time();
}

fn fist_example() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn when_we_have_a_type_with_we_dont_know_the_size_in_compile_time() {
    // wrong way
    /*
        enum List {
            Cons(i32, List),
            Nil,
        }
    */
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
