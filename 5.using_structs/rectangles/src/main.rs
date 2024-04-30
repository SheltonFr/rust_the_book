#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {


    // &self is a shorthand for self: &Self, where Self is the type (and the caller) of the caller object.

    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30, 
        heigth: 50
    };

    let rect2 = Rectangle {
        width: 10, 
        heigth: 40
    };

    // println!("rect is {:#?}", rect);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

}

