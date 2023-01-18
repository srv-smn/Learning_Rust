#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn Origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = Origin(); // memory is allocated on to heap
    let p2 = Box::new(Origin()); // memory is allocated to heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // op 16 bytes || as x and y takes 8+8 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // op 8 bytes || as p2 is storing reference of heap

    let p3 = * p2; // auto boxing
    println!("p3 takes up {} bytes", mem::size_of_val(&p3)); 
    
}