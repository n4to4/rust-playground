#![allow(unreachable_code)]

use iterate::iterate;

fn main() {
    run01();
    run02();
    run03();
}

fn run01() {
    let mut it = iterate![1, 2, 3];

    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
}

fn run02() {
    use std::cell::Cell;

    let cell = Cell::new(0);
    let cell = &cell;

    #[rustfmt::skip]
    let mut iterator = iterate![
        { cell.set(cell.get() + 1); 1 },
        { cell.set(cell.get() + 1); 2 },
        { cell.set(cell.get() + 1); 3 },
    ];

    assert_eq!(cell.get(), 0);
    assert_eq!(iterator.next(), Some(1));
    assert_eq!(cell.get(), 1);
    assert_eq!(iterator.next(), Some(2));
    assert_eq!(cell.get(), 2);
    assert_eq!(iterator.next(), Some(3));
    assert_eq!(cell.get(), 3);
    assert_eq!(iterator.next(), None);
}

fn run03() {
    let range = 0..5;
    let vec = vec![4, 1, 2, 3];
    let vec = &vec;

    #[rustfmt::skip]
    let iterator = iterate![
        1,
        ..range,
        ..vec.iter().copied(),
        10
    ];

    let result: Vec<i32> = iterator.collect();
    #[rustfmt::skip]
    assert_eq!(result, [
        1,
        0, 1, 2, 3, 4,
        4, 1, 2, 3,
        10,
    ]);
}
