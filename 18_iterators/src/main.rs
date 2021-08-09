fn main() {
    first_example();
    iterator_sum();
    own_iterator();
}

fn first_example() {
    println!("FIRST EXAMPLE");

    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value)
    }

    println!("");
}

// In  rust iterator are lazy, the iterator do nothing util we consume him.

// Iterator has two type os methods:
//  - Adaptors: take a iterator, do something and return other iterator
//  - Consumers: take a iterator, do something and return other type

// into_iter take the ownership

fn iterator_sum() {
    println!("iterator_sum");
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    println!("{}", total);
    println!("");
}

fn own_iterator() {
    println!("own_iterator");

    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Self {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b): (u32, u32)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("");
}
