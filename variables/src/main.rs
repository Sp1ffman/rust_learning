use std::io;
use std::io::Write;
fn main() {
    print!("Please Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let mut x: u32 = x.trim().parse().expect("Enter a number! ");
    println!("The value of x is: {x}");
    x=6;
    println!("The value of x is: {x}");
}
