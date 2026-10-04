use std::io::stdin;

fn main(){

    //tuples
    let x:(i32,u32,f64)= (-9,10,279.57);
    // println!("The values in tuple are {x}");
     /* 
     doesn't work -> {The problem is println!("{x}") — tuples don't implement Display (the trait behind {}/{x} formatting), only Debug ({:?}). Tuples don't have one canonical "human-readable" string form, so Rust doesn't provide Display for them automatically.} */
    println!("The values in tuple are {x:?}");/* {x:?} reads as: "format the variable x, using its Debug implementation */
    println!("The idividual values are x.0={}, x.1={}, x.2={}",x.0,x.1,x.2);

    let tup = (1,2,String::from("A"));
    // let mut tup = (1,2,String::from("A"));
    println!("The original tuple is {tup:?}");
    /*
    tup.0=105;
    tup.1=200;
    tup.2=0b1111;
    */
    let (x,y,z)=tup;
    println!("The first value is {x}");
    println!("The second value is {y}");
    println!("The third value is {z}");

    /*
    --------------------------------------------------------------------------------

    */

    //arrays

    let a=[1,2,3,4,5];
    println!("{a:?}");
    for x in a{
        println!("{x}");
    }
    for (i,x) in a.iter().enumerate(){
        println!("The number at index {i} : {x}");
    }
    println!("Please enter an array index");
    let mut index=String::new();
    stdin().read_line(&mut index).expect("Failed to read line");
    let index:usize = index.trim().parse().expect("Enter a number!");
    let element =a[index];
    println!("The value of the element at {index} is {element}");


}