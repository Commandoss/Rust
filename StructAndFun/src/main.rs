#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.heigth > rectangle.heigth && self.width > rectangle.width
    }
    fn square(size: u32) -> Rectangle {
        size,
        size,
    }
}

fn main() {
    let width = 30;
    let heigth = 50;
    //просто функция
    println!("The area of the rectangle is {} square pixels (fun).", area(width, heigth));
    //коржи
    let rectangle = (35, 55);
    println!("The area of the rectangle is {} square pixels (corge).", area1(rectangle));

    let rectangle = Rectangle {
        width: 25,
        heigth: 35,
    };
    println!("The area of the rectangle if {} square pixels (struct).", area2(&rectangle));

    // let rectangle = Rectangle {
    //     width: 10,
    //     heigth: 20,
    // };
    // println!("rectangle is {:?}", rectangle);

    let rectangle = Rectangle {
        width: 10,
        heigth: 20,
    };
    println!("The area of the rectangle if {} square pixels (method).", rectangle.area());

    let rectagle = Rectangle {
        width: 5,
        heigth: 10,
    };

    let rectangle1 = Rectangle {
        width: 15,
        heigth: 20,
    };

    println!("Can rect1 hold rect2? {}", rectangle.can_hold(&rectangle1));

    let rectangle = Rectangle::square(32);
}

fn area(width: u32, heigth: u32) -> u32 {
    width * heigth
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectagle: &Rectangle) -> u32 {
    rectagle.width * rectagle.heigth
}