fn main() {
    // --- Ownership Rules ---

    // 1.Each value in Rust has a variable, that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scop, the value will be dropped

    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
    } // this scope is now over, s is no longer valid and rust drop s

    // Copy
    let c = 5;
    let a = c; // Copy
               // Rust implements Copy Trait for Integer, Boolean and Characters
    println!("c: {}, a: {}", c, a);
    println!("");

    // Move
    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)
    println!("s2: {}", s2);
    println!("");

    // Force the Copy
    let p1 = String::from("hello");
    let p2 = p1.clone();
    println!("o1: {}, p2: {}", p1, p2);
    println!("");

    // Function
    let t1 = String::from("hello");
    take_ownership(t1);
    println!("");

    let n1 = 1;
    make_copy(1);
    println!("n1: {}", n1);
    println!("");

    // Borrowing
    let s3 = String::from("Hello");
    let length = calculate_length(&s3);
    println!("The length of: '{}', is: {}", s3, length);
}

fn take_ownership(t1: String) {
    println!("t1: {}", t1)
}

fn make_copy(n1: i32) {}

fn calculate_length(s: &String) -> usize {
    s.len()
}
