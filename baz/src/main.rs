use static_assertions::assert_not_impl_any;
assert_not_impl_any!(bar::Bar: std::clone::Clone);

fn main() {
    println!("hello, i'm baz");
}
