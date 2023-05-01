#[derive(Debug)]
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
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Using Tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        areaForTuple(rect1)
    );

    // Using Structs
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        areaForStruct(&rect2)
    );
    println!("rect2 is {:?}", rect2); // output format - Debug (Output => rect2 is Rectangle { width: 30, height: 50 })
    println!("rect2 is {:#?}", rect2); // Pretty formatted output
    // Using dbg! to see data
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3); // To prevent debug macro from taking ownership
    // Method Syntax: Methods are similar to functions: we declare them with the fn keyword and a name
    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area()
    );

    // Methods with More Parameters
    let rect5 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect6 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect2 hold rect5? {}", rect1.can_hold(&rect5));
    println!("Can rect2 hold rect6? {}", rect1.can_hold(&rect6));

    // Associated Functions
    let sq = Rectangle::square(3);
    println!("Square Rectangle is {}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn areaForTuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn areaForStruct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}