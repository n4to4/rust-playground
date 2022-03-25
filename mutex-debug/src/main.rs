// https://twitter.com/fasterthanlime/status/1507384399867912195

#![allow(dead_code)]
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Foo {
    foo: u64,
    bar: String,
}

#[derive(Debug)]
struct Outer {
    foo: Arc<Mutex<Foo>>,
}

fn main() {
    let o = Outer {
        foo: Arc::new(Mutex::new(Foo {
            foo: 42,
            bar: String::from("bar"),
        })),
    };
    let _locked = o.foo.lock().unwrap();
    println!("{:#?}", &o);

    //Outer {
    //    foo: Mutex {
    //        data: <locked>,
    //        poisoned: false,
    //        ..
    //    },
    //}

    //Outer {
    //    foo: Mutex {
    //        data: Foo {
    //            foo: 42,
    //            bar: "bar",
    //        },
    //        poisoned: false,
    //        ..
    //    },
    //}
}
