enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn some_func() {}
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };
    //////////////////////////////

    // Option Enum
    let some_number = Some(4);
    let some_string = Some(String::from(""));

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(6);

    let sum = x + y.unwrap_or(0);

    ////////////////////////////////////

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            _ => None,
        }
    }
    let five = Some(5);
    let six = plus_one(five);

    ///////////////////////////////
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("Ulalala");
    };
}
