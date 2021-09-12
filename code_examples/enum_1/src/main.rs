/*enum Option<T> {
    Some(T),
    None
}*/

fn maybe_sqrt(x: f64) -> Option<f64> {
    if x >= 0.0 {
        Some(x.sqrt())
    } else {
        None
    }
}

fn main() {
    match maybe_sqrt(9.0) {
        Some(x) => println!("{}", x),
        None => panic!(),
    }
}