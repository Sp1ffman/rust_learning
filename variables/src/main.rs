use std::io;
//use std::io::Write;
fn main() {
    println!("Please Enter a number: ");
    /*print!("Please Enter a number: "); 
    /* print!() writes text without \n so it just sits in the buffer and waits for a trigger to flush it(go from temp holding aread(buffer) to its real destination(here terminal))*/
    /*    
    io::stdout().flush().expect("Failed to flush stdout");
    /* So this line is required to manually flush it and print the statement in the termianl */    
    */
    */
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let mut x: u32 = x.trim().parse().expect("Enter a number! ");
    println!("The value of x is: {x}");
    x=6;
    println!("The value of x is: {x}");
}
