//derive directive allows pretty-print to debug line
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//define object methods in impl block
impl Rectangle {
    //self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn grow(&mut self) {
        self.width = self.width + 1;
        self.height = self.height + 1;
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//basic struct for defining types
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

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    let user2 = User {
        email: user1.email,
        username: String::from("new Username"),
        ..user1
    };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let mut sq = Rectangle::square(3);

    sq.grow();
    println!("user2 is {}", user2.username);
    println!("rect1 is {:?}", rect1);
    println!("rect1 area {}", rect1.area());
    println!("rect1 can hold sq {}", rect1.can_hold(&sq));
}