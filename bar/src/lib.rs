#[cfg_attr(feature = "fuzzing", derive(Clone))]
pub struct Bar(u32);

pub fn bar_something() {
    println!("hello, i'm bar");
}

#[cfg(features = "fuzzing")]
pub fn bar_fuzzing() {
    println!("fuzz me");
}
