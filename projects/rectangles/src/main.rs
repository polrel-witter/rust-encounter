#[derive(Debug)]  // signals debugging info for the Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50,};
    let rect2 = Rectangle{width: 10, height: 40,};
    let rect3 = Rectangle{width: 60, height: 45,};

    println!("rect1 is {rect1:?}"); // only possible becuase we signaled the dervie(Debug) attribute for the Rectangle struct and formatted it as a string with `:?`

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}