pub fn receives_closure<T>(closure: T)
where
    T: Fn(i32, i32) -> i32,
{
    let result = closure(10, 19);
    println!("Result: {}", result);
}

pub fn run_1() {
    println!("[demystifying_closures_parte1] [run_1]");
    let sum = |x, y| x + y;
    receives_closure(sum);
    println!("");
}

pub fn returning_closure() -> impl Fn(i32) -> i32 {
    |x| x
}

pub fn run_2() {
    println!("[demystifying_closures_parte1] [run_2]");
    let closure = returning_closure();
    closure(3);
    println!("Result: {}", closure(3));
    println!("");
}

pub fn curry<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y)
}

pub fn run_3() {
    println!("[demystifying_closures_parte1] [run_3]");

    let curry_closure = |x, y| x + y;
    let result_closure = curry(curry_closure, 19);

    println!("Result: {}", result_closure(3));
    println!("");
}

pub fn generic_curry<F, X, Y, Z>(f: F, x: X) -> impl Fn(Y) -> Z
where
    F: Fn(X, Y) -> Z,
    X: Copy,
{
    move |y| f(x, y)
}

pub fn run_4() {
    println!("[demystifying_closures_parte1] [run_4]");

    let curry_closure = |x, y| x + y;
    let result_closure = generic_curry(curry_closure, 19);

    println!("Result: {}", result_closure(3));
    println!("");
}
