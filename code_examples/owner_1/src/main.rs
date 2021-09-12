fn main() {
    let first_owner = "Hello Owner".to_string();
    // move ownership
    let second_owner = first_owner;
    println!("{}", second_owner);
    // string is dropped
}
