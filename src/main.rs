#![allow(dead_code)]
#![allow(unused_variables)]

struct Point {
    x: f64,
    y: f64
}
struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p1 = Point{x: 3.0, y: 4.0};
    let p2 = Point{x:5.0, y: 10.0};

    let myLine = Line{start: p1, end: p2};
    println!("line start at ({},{}) and ends at ({},{})", myLine.start.x,myLine.start.y, myLine.end.x, myLine.end.y);
}
fn main() {
    //struct
    structures();
}
