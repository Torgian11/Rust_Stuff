struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30
    };

    let rect2 = Rectangle {
        width: 40,
        height: 20
    };

    let rect3 = Rectangle {
        width: 55,
        height: 30
    };

    let square1 = Rectangle::square(10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("What about rect3? {}", rect1.can_hold(&rect3));
    println!("Square {}", square1.area());
}
