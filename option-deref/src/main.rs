// https://twitter.com/m_ou_se/status/1508509338482221078

struct Thing;

fn main() {
    let a: Option<Box<Thing>> = Some(Box::new(Thing));
    let _: Option<&Thing> = a.as_deref();

    let a: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    let b: Option<&[i32]> = a.as_deref();

    let a: Option<String> = Some(format!("{b:?}"));
    let _: Option<&str> = a.as_deref();
}
