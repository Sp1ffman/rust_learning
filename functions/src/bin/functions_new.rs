use std::io::stdin;

fn second_function(){
    println!("second function");
}
fn main(){
    println!("Testing functions");
    let mut a = String::new();
    println!("Enter a number:");
    stdin().read_line(&mut a).expect("Failed to read line");
    let a =a.trim().parse().expect("Please only enter a number");  
    let mut b = String::new();
    println!("Enter 2nd number:");
    stdin().read_line(&mut b).expect("Failed to read line");
    let b =b.trim().parse().expect("Please only enter a number");  
    first_function();
    second_function();
    // param_function(-20);
    param_function(a);
    addition_function(a,b);
}
fn first_function(){
    println!("first function");
}
fn param_function(x:i32){
    println!("The value of x is {x}");
}
fn addition_function(a:i32,b:i32){
    println!("The sum of {a} and {b} is {}",a+b);
}