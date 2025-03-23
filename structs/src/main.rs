
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}



fn main() {
    let mut user1 = User {
        username: String::from("someusername"),
        email: String::from("some123.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("some1234.com");
    println!("User1: {}", user1.username);
    println!("User1: {}", user1.email);
    println!("User1: {}", user1.active);
    println!("User1: {}", user1.sign_in_count);

    let user2 = build_user(
        String::from("some1234.com"), 
        String::from("someusername")
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
    println!("The area of the rectangle is: {}", area(&rect1));
    println!("The area of the rectangle is: {}", rect1.area());
    println!("The width of the rectangle is: {}", rect1.width());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

