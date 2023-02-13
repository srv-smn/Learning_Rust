#![allow(dead_code)]
#![allow(unused_variables)]

fn print_value(x: i32)
{
    println!("value = {}",x);
}

fn increase(x: &mut i32)
{
    *x +=1;
}
fn product(x:i32, y:i32) -> i32{
    x*y // to return a value dont put ";"
}
fn functions() 
{
    print_value(33);

    let mut z= 1;
    increase(&mut z);
    println!("z = {}",z);

    let a =3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}",a,b,p);
}

struct Point {
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.start.x -  self.end.x;
        let dy = self.start.y -  self.end.y;
        (dx*dx + dy*dy).sqrt()

    }
}

fn methods()
{
    // methods are the behaviou of a struct 
    // eg: len

    let p1 = Point{x:3.0, y:4.0};
    let p2: Point = Point{x: 5.0, y:10.0};
    let myLine = Line{start:p1, end: p2};
    println!("length = {}", myLine.len());
}
fn main() {
    //functions();

    methods();
}
