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

fn say_hello() { println!("hello");}

fn closures()
{   
    // we can assign a variable 
    // this concepts will be used for closure
    let sh = say_hello;
    sh();

    // declaring function and assign it to variables
    // this function will have a limited scopes

    let plus_one = |x:i32| -> i32 {x+1};
    let a =6;

    println!("{}+1 = {}",a, plus_one(a));

    let mut two = 2;
    { // everything declared under this scope will get destroyed 
        // after the closing of the bracket, this is done to avoid 
        // borrow issue
        let plus_two = |x|
        {
            let mut z =x;
            z +=two;
            z 
        };
        println!("{} + 2 = {}",3,plus_two(3));
    }
    let borrow_two = &mut two;
    /**
     * ways to call a function
     * T: by value
     * T& by reference
     * &mut by mut reference
    */

    let plus_three = |x: &mut i32| *x += 3;

    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}",f);
}
fn main() {
    //functions();

   // methods();

    // closure
    closures();
}
