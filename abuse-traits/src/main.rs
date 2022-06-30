const iter: fn() -> Iter = || Iter;
impl IntoIterator for fn() -> Iter {}

struct Iter;
impl Iterator for Iter {}

fn main() {
    println!("Hello, world!");
}
