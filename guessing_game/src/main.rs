use std::io; // std is the standard library, io is the input/output library, use is used to import modules into the current scope

fn main(){
    println!("Guess the number!");
    
    println!("Please input your guess.");

    let mut guess = String::new();/*let is used to declare a variable, mut is used to make the variable mutable,
    new is used to create a new instance of a string*/

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {guess}");

    // user input integer
    // let mut x=String::new();
    // let mut y=String::new();
    // println!("Please enter the first number to add:");
    // io::stdin().read_line(&mut x).unwrap();
    // println!("Please enter the second number to add:");
    // io::stdin().read_line(&mut y).unwrap();    
    // let x1:i32=x.trim().parse().unwrap();    
    // let y1:i32=y.trim().parse().unwrap();
    // println!("The sum of {x1} and {y} is {}",x1+y1);
}   