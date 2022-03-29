#[derive(Debug)]
struct Rectangle {
    height: u32,
    weight: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.weight
    }
    fn can_hold(&self, second_rectangle: &Rectangle) -> bool {
        if self.area() >= second_rectangle.area() {
            true
        } else {
            false
        }
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            height: size,
             weight: size 
        }
    }
}

fn main() {
    let my_rectangle = Rectangle {
        height: 10,
        weight: 20,
    };

    let little_rectangle = Rectangle {
        height: 5,
        weight: 5,
    };

    let big_rectangle = Rectangle {
        height: 100,
        weight: 200,
    };

    let square = Rectangle::square(10);

    println!("Area - {}", my_rectangle.area());
    println!("Can big rectangle hold little one? - {}", big_rectangle.can_hold(&little_rectangle));
    println!("Can a small rectangle hold big one? - {}", little_rectangle.can_hold(&big_rectangle));
    println!("Squre area: {}", square.area());
}
