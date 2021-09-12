fn main() {
    let first_owner = "Hello Owner".to_string();
    // move ownership
    let second_owner = first_owner;
    // usage of moved value
    println!("{}", first_owner);
    // string is dropped
}
