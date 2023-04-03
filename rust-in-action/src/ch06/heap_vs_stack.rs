#![allow(dead_code)]
use std::mem::drop;

pub fn run() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    drop(a);
    let d = Box::new(2);
    let result2 = *b + *c + *d;

    println!("result1: {}", result1);
    println!("result2: {}", result2);
}
