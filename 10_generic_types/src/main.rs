fn main() {
    let int_list = vec![1, 2, 3, 4, 5];
    let largest = get_largest(int_list);
    println!("The Largest is: {}", largest);

    let char_list = vec!['a', 'v', 'c'];
    let largest = get_largest(char_list);
    println!("The Largest is: {}", largest);

    struct Point<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl<U> Point<f64, U> {
        fn y(&self) -> &f64 {
            &self.x
        }
    }
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}
