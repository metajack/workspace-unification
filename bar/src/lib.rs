pub fn bar_something() {
    println!("hello, i'm bar");
}

#[cfg(features = "fuzzing")]
pub fn bar_fuzzing() {
    println!("fuzz me");
}
