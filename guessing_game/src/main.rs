use std::io; // std is the standard library, io is the input/output library, use is used to import modules into the current scope

fn main(){
    println!("Guess the number!");
    
    println!("Please input your guess.");

    let mut guess = String::new();/*let is used to declare a variable, mut is used to make the variable mutable,
    new is used to create a new instance of a string*/

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
}