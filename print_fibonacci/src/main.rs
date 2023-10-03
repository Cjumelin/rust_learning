fn main() {
    let mut previous = [0, 1];
    for i in 0..45 {
        println!("n: {}, xn: {}", i, previous[0]);
        let swp = previous[1];
        previous[1] = previous[0] + previous[1];
        previous[0] = swp;
    }
}
