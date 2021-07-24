fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000_000;
    println!("The value of const is: {}", SUBSCRIBER_COUNT);

    let shadowing = 5;
    println!("Shadowing 1: {}", shadowing);
    let shadowing = "Ok";
    println!("Shadowing 2: {}", shadowing);

    //////////////////////////////////////

    // Scale Data Types
    //  - Integer: 8-bit, 16-bit, 32-bit, 64-bit, 128-bit, arch(isize, usize)
    let int = 98_111; // Decimal
    let int = 0xff; // Hex
    let int = 0o66; // Octal
    let int = 0b1111_0000; // Binary
    let int = b'a'; // Byte

    //  - Floating-point numbers
    let float = 2.0;

    //  - Booleans
    let boolean = true;

    //  - Character - UTF-8
    let char = '%';

    //////////////////////////////////////

    // Compound Types
    // - Tuples
    let tup = ("Ok", 100_000);

    // Arrays
    let array = [1, 2, 3, 6];

    // - Enums
    enum Enum {
        First,
        Second,
        Third,
    }

    // - Struct
    struct Struct {
        First: String,
        Second: String,
        Third: String,
    }

    // - Vec
    let vector = Vec::<String>::new();
}
