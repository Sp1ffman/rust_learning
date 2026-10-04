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
    first_function();
    second_function();
    // param_function(-20);
    param_function(a);
}
fn first_function(){
    println!("first function");
}
fn param_function(x:i32){
    println!("The value of x is {x}");
}