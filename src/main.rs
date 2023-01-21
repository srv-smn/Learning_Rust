#![allow(dead_code)]
#![allow(unused_variables)]

fn if_statement() {
    let temp =  25;
    if temp > 30 {
        println!("reaally hot outside");
    } else if  temp < 10{
        println!("really cold")
    }else {
        println!("temperature is ok")
    }
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("it is {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

}

fn while_and_loop(){
    let mut x =1;
    // while loop

    while x < 1000 {
        x *= 2;
        if x == 64 {continue;}
        println!("x = {}", x);
    }
    let mut y =1;
    // loop = while true
    // need to explicitly break to come out of loop

    loop {
        y *= 2;
        println!("y = {}",y);

        //2^10 = 1024
        if y == 1<<10 {break;}
    }
}
fn for_loop(){
    // here x is variable which will be iterated 
    // x value will start from 1 
    // and go till 10 (11 is exlusive)
    for x in 1..11 {
        // continue also work as expected in for
        if x == 3 {continue;}
        // break also work as expected in for
        if x == 8 {break;}

        println!("x = {}", x);
    }

    // if we want to start from any number
    // we also want the position of that 
    // number in the range
    // position will start from 0
    for (pos,y) in (30..41).enumerate()
    {
        println!("{} : {}",pos,y);
    }
}
fn main() {
    // if statement
   // if_statement();

   // while loop
   //while_and_loop();
   
   // for loop
   for_loop();
}
