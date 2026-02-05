struct Rectangle {
    width: u32,
    height: u32,
}

// v4
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // v1
    let width = 3;
    let height = 5;
    println!(
        "The area of the rectangle with width: {} and height: {} is {}",
        width,
        height,
        area_v1(width, height)
    );

    // v2
    let rect = (30, 50);
    println!(
        "The area of the rectangle with width: {} and height: {} is {}",
        rect.0,
        rect.1,
        area_v2(rect)
    );

    // v3
    let rect = Rectangle {
        width: 10,
        height: 15,
    };
    println!(
        "The area of the rectangle with width: {} and height: {} is {}",
        rect.width,
        rect.height,
        area_v3(&rect)
    );

    // v4
    let rect = Rectangle {
        width: 12,
        height: 12,
    };
    println!(
        "The area of the rectangle with width: {} and height: {} is {}",
        rect.width,
        rect.height,
        rect.area()
    );
}

fn area_v1(x: u32, y: u32) -> u32 {
    x * y
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_v3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
