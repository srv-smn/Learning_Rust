#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;
mod sh;

// global variable and constants
const MEANING_OF_LIFE: u8 = 42; // it does not have a fixed address, every where
                            // this variable used will be replaced by the value of 
                            // at compilation level
static x: i32 =123; // this can be used anywhere in the program, since it is declared outside of any scope

static mut z0:i32 = 234;
// this is mutable global variable 

fn main() {
    println!("inside main function");
    // data types
   // data_types();

    // operators
  //  operator();

  // scope and shadowing concepts
 // scope_and_shadowing();

 // global and constants

//  println!("Meaning of Life {}", MEANING_OF_LIFE);
//  println!("global variable x {}", x);

//  unsafe{
//     /*
//     since z is global variable and it is mutable,
//     so it can be modified by different thread and different places
//     which rust try to avoid.
//     So to use 'z0' we will have to use it in a unsafe block,
//     to let the compiler know that you know that you are doing some 
//     unsafe operation
//     */
//     z0 = 789;
//     println!("value of z0 {}",z0);
//  }

sh::stack_and_heap();
}

fn data_types() {
    let a = 123;
    println!("a = {}",a);

    // trying to mutate the value
   // a = 24; error as a is immutable

   let mut b=123;
   println!("b = {}",b);

   // mutable variables
   b = 25;
   println!("new value of b is {}",b);

   let c = 1234567;

   println!("value of c is {} and size of c is {}",c, mem::size_of_val(&c));
    // u8 i8 u16 i16 u32 i32 u64 i64

    let z: usize = 12345; // usize, isize. Takes size of the memory arch of the system running on
    println!("value of z is {} and size of z is {} bytes, running on {} bit os ",z, mem::size_of_val(&z),8*mem::size_of_val(&z));

    // float datatype
    let e = 3.25;
    println!("value of e is {} and size of e is {} bytes",e, mem::size_of_val(&e));
    
    let f = false;
    println!("value of f is {} and size of f is {} bytes",f, mem::size_of_val(&f));

}

fn operator() {
    let mut a = 2+3*4; // + - * % / 
    println!("{}",a);
    a = a+1; // inc and dec -- ++ operator is not supported
    a -= 2; // -= += *= /= %=

    println!("remainder of {}/{} = {}",a,3,(a%3));

    let a_cube = i32::pow(a,3);
    println!("{} cubed is {}",a,a_cube);

    let b = 2.5;
    let b_to_cube = f64::powi(b,3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b,b_to_cube,b, b_to_pi);

    // bitwise
    // bitwise is only available for 
    let c = 1|2; // | or , & and, ^ xor, !nor
    println!("1|2 = {}", c);
    let two_to_ten  = 1 << 10; //>> right/left shift
    println!("2^10 is {}",two_to_ten);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    // > < <= >=
    println!("Pi is less than 4, its {}", pi_less_4);
    
    
}

fn scope_and_shadowing() {
    let a = 123;

    //let a = 111; 
    //this declaration of 'a' variable will not raise any error
    // but it will re declare the a variable again with this new value

    
    {
        let b = 456;
        println!("inside b  = {}", b);

        // this inner declaration of a will overshadow the outer declaration
        // of a inside this inner declaration
        let a = 777;
        println!("inside a = {}",a);

    }

    // whenever we open a "{" it creates a scope
    println!("outside a = {}", a);
}

