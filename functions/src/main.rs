fn main() {
    let res = another_function();
    println!("{res}");
}

fn another_function() -> i32 {
    let number = 3;

    if number < 5 {
        number
    } else {
        println!("condition was false");
        0
    }
}
