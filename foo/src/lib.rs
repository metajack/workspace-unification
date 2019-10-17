pub fn foo_something() {
    println!("hello, i'm foo");
}

#[cfg(features = "fuzzing")]
pub fn foo_fuzzing() {
    println!("fuzz me");
}
