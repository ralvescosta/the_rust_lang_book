fn main() {
    // Fundamentals
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let mut user1 = User {
        email: String::from("email@email.com"),
        username: String::from("ze"),
        sign_in_count: 1,
        active: true,
    };
    user1.username = String::from("Doidao");

    let user2 = build_user(String::from("email@email.com"), String::from("User"));
    let user3 = User {
        email: String::from("email@email.com.br"),
        username: String::from("Uhua"),
        ..user2
    };
    //////////////////////////////////////////
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        //associate functions (Without self) - Like a static method in a class
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }
    }

    let rect = Rectangle {
        width: 10,
        height: 10,
    };

    println!("Area: {}", rect.area());

    //#[derive()] allows to compile implement some Traits
}
