use std::io; // std is the standard library, io is the input/output library, use is used to import modules into the current scope
use rand::Rng;  
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    
    let secret_number=rand::thread_rng().gen_range(1..=500);// generate a random number between 1 and 500

    loop{ //infinite loop
    
    println!("Please input your guess.");

    let mut guess = String::new();/*let is used to declare a variable, mut is used to make the variable mutable,
    new is used to create a new instance of a string*/

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    //let guess:u32=guess.trim().parse().expect("Please type a number!");
    let guess:u32=match guess.trim().parse(){// parse return a Result type, Ok(num) is the number, Err(_) is the error
        Ok(num) => num,//if the number is valid, return the number
        Err(_) => continue,//Err(_)-catch all errors, continue to the next iteration of the loop
    };

    println!("You guessed: {guess}");

     match guess.cmp(&secret_number){ // cmp is a method that compares two values and returns a Result type
        Ordering::Less => println!("Rushing!"), //Ordering is an enum with 3 variants: Less, Greater, Equal
        Ordering::Greater => println!("Dragging!"),
        Ordering::Equal =>{ println!("Little trouble there!");
        break;
    }
    }
}

   
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