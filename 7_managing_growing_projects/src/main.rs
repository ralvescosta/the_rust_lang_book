mod front_of_house;
use front_of_house::{hosting, serving};

fn main() {
    // Roles
    // 1- A package must have one create
    // 2- A package could have 0 or 1 library creates
    // 3- A package could have any number binary creates

    // The 'use' keyword bring cretes to current scop
    do_something();
}

fn do_something() {
    hosting::add_to_wait_list();
    serving::serve_order();
}
