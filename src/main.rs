#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;
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

// option
/*
Type Option represents an optional value: every Option is either Some and contains a value, or None
It can have multiple uses like:
Return values for functions that are not defined over their entire input range (partial functions)
Return value for otherwise reporting simple errors, where None is returned on error

A option type will have either "Some" value or "None" value
*/

fn option() {
    let x = 3.0;
    let y = 2.0;

    // if valid value then Some else None

    let result: Option<f64> = 
        if y != 0.0 {Some(x/y)} else {None};

    println!("{:?}", result);

    // Some value can be utilised in multiple ways like

    match result {
        Some(z) => println!("{}/{} = {}",x,y,z),
        None => println!("can not divide {} by {}",x,y)
    }

    // if let / while let
    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(z) = result {println!("z = {}",z);}

}

// arrays
// we can not re-size an array
// array can not be dynamic sized
// the size of the index in array or vector "var_name[index]" is usize

fn arrays() {
    // [132;5] => [dataType: size]
    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} element, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}",a);

    if a == [321,2,3,4,5]{
        println!("match");
    }

    let b = [1; 10];
    // b is array of length 10 and all element is 1
    // by default the datatype that b stores is i32 to in total b takes 40 bytes as len is 10
    // but we can specify the datatype as let b:[i16: 10] = [1:10] or let b = [1i16:10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    // 2-D arrays
     // let var_name = [[column_dataType;no_of_col], no_of_row]
     let mtx:[[f32;3];2] = 
        [
            [1.0, 0.0, 0.0],
            [0.0, 2.0, 0.0]
        ];
        println!("{:?}", mtx);

        // iterating over loop
         for i in 0..mtx.len()
         {
            for j in 0..mtx[0].len()
            {
                if i==j 
                {
                    println!("mtx[{}][{}] = {}",i,j,mtx[i][j]);
                }
            }
         }

}
/*
Declare dynamic data(change length at runtime).
index of any data structure in rust is usize
*/
fn vectors() {
    let mut a = Vec::new(); // making vector mutable to edit in future
    a.push(1);
    a.push(2); // adding value in vector
    a.push(3);

    println!("a = {:?}",a);
    a.push(44);
    println!("a = {:?}",a);

    let mut idx:usize = 0;
    a[idx] = 312;
    println!("reading value at a[idx] = {}", a[idx]);

    idx = 10;
    // a[idx] = 312; this will throw error as idx is > total element
    // so safe way to access value is get()
    //get() returns either some or none

    // option
    match a.get(3)
    {
        Some(x) => println!("a[3] = {}",x),
        None => println!("error, no such elemnt")
    }

    // iterating over vector
    for x in &a {println!("{}",x);}

    a.push(77);
    println!("{:?}",a);

    // pop() returns Option
    let last_ele = a.pop(); // Some(77)
    println!("last element is {:?}, a = {:?}",last_ele, a);

    // printing vector in reverse
    while let Some(x) = a.pop()
    {
        println!("{}",x);
    }


}

fn use_slice(slice: &mut[i32])
{   
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices(){
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]); // data[1..4] = 2,3,4
    //use_slice(&mut data);
    println!("{:?}", data);
}

fn strings()
{
    // utf-8
    let s: &'static str = "hello there";
    /*
    Here '&str' is string slice
    static means the memory is allocated at compile time
    */

    // s = "abc"; assignment to this variable will give error
    // let h = s[0]; array like manipulation is also not possible

    // iterating over each character in string
    for c in s.chars() // to access the character in reverse we can use s.chars().rev()
    {
        println!("{}",c); // we can print each character of the word
    }

    // accessing any particular character
    if let Some(firsr_char) = s.chars().nth(0)
    {
        println!("first letter is {}", firsr_char);
    }

    // heap
    // string

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a<= ('z' as u8)
        {
            letters.push(a as char);// pass char
            letters.push_str(","); // pass string
            a +=1;
        }
        println!("{}", letters);

        // some function might need String as input and
        // some takes &str as input
        // so we might need to convert &str to String and vice versa

        // &str <> String
        let u :&str= &letters;

        // concatination
        // there could be many combination of concatination
        // String + str
        //let z = letters + &letters;
        
        // converting str to string
        let mut abc = "hello world".to_string();
        abc.remove(0);
        abc.push_str("!!!");
        println!("{}",abc.replace("ello", "goodbye"));

}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{   // when the value will be returned it will be in the form of tupple
    // values in tupples can be hetrogenous
    (x+y, x*y)
}
fn tuples()
    {
        let x =3;
        let y = 4;
        let sp = sum_and_product(x, y);
        println!("sp = {:?}",sp);
        println!("{0}+{1} = {2}, {0}*{1} = {3}",x,y,sp.0, sp.1);

        // destructuring
        let (a,b) =sp;
        println!("a={}, b={}",a,b);

        let sp2 = sum_and_product(4,7);
        let combined = (sp,sp2);
        println!("{:?}",combined);

        // making tuple of single element
       // let m = (20) this will create a integer var
       let m = (42,);
       println!("{:?}",m);

    }

    fn how_many(x: i32) -> &'static str
    {
        match x
            {
                0 => "no",
                1|2 => "one or two",
                12 => "a dozen",
                z @ 9..=11 => "lots of",
                _ if (x%2 ==0) => "some",
                _ => "a few"
            }
    }

    fn pattern_matching()
        {
            for x in 0..13
                {
                    println!("{}: I have {} oranges", x, how_many(x));
                }
            let point = (0,7);

            match point
            {
                (0,0) => println!("origin"),
                (0,y) => println!("x axis, y = {}", y),
                (x,0) => println!("y axis, x = {}",x) ,
                // if we want to paas the argument as reference and modifiy
                // (ref mut x,0) => println!("y axis, x = {}",x) ,
                (_, y) => println!("(?, {})", y) 
            }
        }


fn main() {
    //struct
   // structures();

   //enum
   //enums();

   //union
   //unions();

   // option
   //option();

   // arrays
   //arrays();

   // vectors
  // vectors();

  // slices
  //slices();

  // string
  //strings();

  // tuples
  //tuples();

  // pattern matching
  pattern_matching();
}
