#[derive(Debug)] // Allows us to pretty print structs without implementing `Debug` trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is much better
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // We can also give a method the same name as a field
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can also add associated functions, which are functions associated with the type (like
    // static or class methods in other languages)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let another_rect = Rectangle {
        width: 10,
        height: 40,
    };
    let square = Rectangle::square(50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect.width);
    }

    println!("Can rect1 hold rect2? {}", rect.can_hold(&another_rect));

    dbg!(&square);
}

// We should instead turn this into a method
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
