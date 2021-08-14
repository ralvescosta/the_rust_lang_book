use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    first_example();
    second_example();
}

fn first_example() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    let b = Cons(1, Rc::clone(&a));
    let c = Cons(1, Rc::clone(&a));
}

fn second_example() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(4, Rc::clone(&a));
    println!("count after creating b {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!(
        "counter after c goes out of the scop = {}",
        Rc::strong_count(&a)
    );
}
