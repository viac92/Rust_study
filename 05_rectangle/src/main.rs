#[derive(Debug)]
struct Rectangle {
    height: u32,
    weight: u32,
}

fn main() {
    let my_rectangle = Rectangle {
        height: 10,
        weight: 20, 
    };
    println!("{}", area(&my_rectangle));
    dbg!(&my_rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.weight
}
