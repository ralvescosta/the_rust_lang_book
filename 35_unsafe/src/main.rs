use core::slice;

fn main() {
    dereferencing_a_raw_pointer();
    unsafe_functions_methods();
    creating_a_safe_abstraction();
    functions_to_call_external_code();
}

fn dereferencing_a_raw_pointer() {
    println!("DEREFERENCING A RAW POINTER!!");
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // println!("r1 is: {}", *r1); dont work
    // println!("r2 is: {}", *r2); dont work

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    println!("");
}

fn unsafe_functions_methods() {
    println!("UNSAFE FUNCTIONS AND METHODS!");

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    println!("");
}

fn creating_a_safe_abstraction() {
    println!("CREATING A SAFE ABSTRACTION!");

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("");
}

fn functions_to_call_external_code() {
    println!("CALL FOREIGN LANGUAGES!");
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }

    println!("")
}
