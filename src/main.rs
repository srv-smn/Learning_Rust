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
fn main() {
    // if statement
   // if_statement();

   // while loop
   while_and_loop();
}
