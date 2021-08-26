use gazebo::prelude::*;
use gazebo::variants::VariantName;

#[derive(VariantName)]
enum MyEnum {
    Foo,
    Bar,
    Baz,
}

fn main() {
    assert_eq!(MyEnum::Foo.variant_name(), "Foo");
    assert_eq!(MyEnum::Baz.variant_name(), "Baz");

    let x: &'static str = MyEnum::Bar.variant_name();
    dbg!(x);

    let xs: Vec<i32> = vec![1, -1, 5, -3, 0];
    let ys = xs.map(|x| x.abs());
    dbg!(ys);
}
