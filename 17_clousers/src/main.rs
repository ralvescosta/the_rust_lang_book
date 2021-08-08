use std::{thread, time::Duration};

fn main() {
    first_example_without_closure();
    second_example_with_simple_closure();
    third_example_with_closure_in_struct();
}

fn first_example_without_closure() {
    println!("first_example_without_closure");
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps!",
                simulated_expensive_calculation(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }

    let simulated_intensity = 0;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
    println!("");
}

fn second_example_with_simple_closure() {
    println!("second_example_with_simple_closure");
    fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |n: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            n
        };
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));
            println!("Next, do {} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_closure(intensity));
            }
        }
    }

    let simulated_intensity = 0;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
    println!("");
}

fn third_example_with_closure_in_struct() {
    println!("third_example_with_closure_in_struct");

    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Self {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut cached_result = Cacher::new(|n: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            n
        });
        if intensity < 25 {
            println!("Today, do {} pushups!", cached_result.value(intensity));
            println!("Next, do {} situps!", cached_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", cached_result.value(intensity));
            }
        }
    }

    let simulated_intensity = 0;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
    println!("");
}
