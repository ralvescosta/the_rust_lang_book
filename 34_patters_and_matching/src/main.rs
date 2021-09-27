fn main() {
    match_with_enum();
    match_with_if_let();
    match_with_while_let();
}

fn match_with_enum() {
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello, World!"),
        Language::Spanish => println!("Hola Mundo!"),
        _ => println!("Unsupported language"),
    }
}

fn match_with_if_let() {
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id = "34".parse::<u8>();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    }
}

fn match_with_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
