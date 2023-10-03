fn main() {
    // Need explicit typing

    // Not working => error E0282
    // let _guess = "42".parse().expect("Not a number!");
    
    // Can by typed at declaration or operation

    // Declaration
    let _guess: i32 = "42".parse().expect("Not a number!");
    let _x: Vec<char> = "hello".chars().rev().collect();
    
    // During operation
    let _guess = "42"
        .parse::<i32>()
        .expect("Not a number!");
    let _x = "hello"
        .chars()
        .rev()
        .collect::<Vec<char>>();

    // int separator char _ is ignore and only here to enhance human redability
    println!("{}", 5_1_6_5_4);
}