fn main() {
    println!("Simple Loop");
    simple_loop();

    println!("\n");
    println!("Nested Loops");
    nested_loops();

    println!("\n");
    println!("While Loop");
    while_loop();

    println!("\n");
    println!("List collection with while Loop");
    list_collection_with_while();

    println!("\n");
    println!("For collection");
    for_collection();

    println!("\n");
    println!("For loop range");
    for_loop_range();
}

fn simple_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if count == 2 {
                break 'counting_up;
            }
            if remaining == 9 {
                break;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn list_collection_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index <= 4 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_collection() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_loop_range() {
    for elem in 0..5 {
        println!("{elem}");
    }
}
