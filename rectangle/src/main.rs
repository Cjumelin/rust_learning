#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect = Rectangle {
        width: 43,
        height: 34,
    };
    dbg!(&rect);
    println!("Rectangle area: {:#?}", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
