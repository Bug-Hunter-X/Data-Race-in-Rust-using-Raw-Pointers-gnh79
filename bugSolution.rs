fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, use safe Rust methods
    v[0] = 10; //Modify the first element directly
    println!("The first element is: {}", v[0]);
}
