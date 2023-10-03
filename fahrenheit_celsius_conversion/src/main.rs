fn main() {
    let temp = 41;
    let f_to_c = farhenheit_to_celsuis(temp);
    let c_to_f = celsuis_to_farhenheit(temp);

    println!("F to C => {temp} to {f_to_c}");
    println!("C to F => {temp} to {c_to_f}");
}

fn farhenheit_to_celsuis(ferhenheit: i32) -> f64 {
    (ferhenheit - 32) as f64 * 1.8
}

fn celsuis_to_farhenheit(celsuis: i32) -> f64 {
    celsuis as f64 * 1.8 + 32 as f64
}
