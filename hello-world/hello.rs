/// This is a docimentation comment.
/// it can be a multi line comment
fn main() {
    // This is a comment
    /*
     * This is another comment
     */
    println!("Hello world!");
    println!("I am a Rustacian now.");
    println!("This is how to print variable values {}", 23232u32);
    println!(
        "This is how you can print multiple values in different order {0},{1},{1},{0}",
        0b10001010, "hi"
    );
}
