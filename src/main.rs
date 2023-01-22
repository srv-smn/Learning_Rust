#![allow(dead_code)]
#![allow(unused_variables)]

// struct
struct Point {
    x: f64,
    y: f64,
}
struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };

    let my_line = Line { start: p1, end: p2 };
    println!(
        "line start at ({},{}) and ends at ({},{})",
        my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y
    );
}

// enum
// It can have normal vaules like RED
// It can also have tupple or struct
// it is very powerfull
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black: u8},
}
fn enums(){
    let c = Color::CmykColor { cyan: 0, magenta: 128, yellow: 0, black: 255 };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::CmykColor { cyan:_, magenta:_, yellow:_, black:255} => println!("black"), //all value can be anything but black should be 255
        Color::RgbColor(r, g, b) => println!("rgb {},{},{}",r,g,b),
        _ => ()
    }
}

// union
/*
Union is a concept taken from c/c++ which can contain value 
of different type, but only any one of its property can have
value.
since union can have value for any of the property, so it is 
not sure which data we will get while reading 
so this read are unsafe
 */
union IntOrFloat {
    i:i32,
    f:f32
}

fn process_value(iof: IntOrFloat){
    unsafe{
        match iof {
            IntOrFloat {i:42 } => {println!("meaning of life");}
            IntOrFloat{f} => {println!("f32 = {}",f);}
        }
    }
}

fn unions(){
    let mut iof = IntOrFloat{i:123};
     iof.i = 42;
    let value = unsafe {iof.i};
    process_value(iof);
    process_value(IntOrFloat{f: 1.23});


}
fn main() {
    //struct
   // structures();

   //enum
   //enums();

   //union
   unions();
}
