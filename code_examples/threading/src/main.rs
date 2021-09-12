fn main() {
    let message = "Hello thread".to_string();
    std::thread::spawn(move || {
        println!("{}", message);
    }).join().unwrap();
    // not allowed, since value has been moved
    // eprintln!("{}", message);
}
