fn main() {
    let mut owner: String = "Hello String".to_string();
    // immutable reference
    let borrow: &String = &owner;
    println!("owner: {}, borrow: {}", owner, borrow);
    // mutable reference
    let mut_borrow: &mut String = &mut owner;
    *mut_borrow += "!!";
    println!("modified owner: {}", owner);
}
