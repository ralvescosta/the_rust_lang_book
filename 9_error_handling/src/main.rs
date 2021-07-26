use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    do_panic();
    println!("");
    let mut file = File::open("hello.txt")?;
    Ok(())
}
fn do_panic() {
    panic!("crash and burn")
}
fn result_enum_with_match() {
    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating the file {:?}", err),
            },
            other_err => panic!("Problem opening the file: {:?}", other_err),
        },
    };
}
fn result_enum_with_closure() {
    let file = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file {:?}", err)
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
