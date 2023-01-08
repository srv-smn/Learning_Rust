use std::mem;
fn main() {
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
