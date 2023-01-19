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
fn main() {
    if_statement();
}
