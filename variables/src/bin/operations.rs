fn main(){
    //addition
    let sum =-5+10;
    println!("the sum is {sum}");

    //subtraction

    let diff = -5 - (-5);
    println!("The difference is {diff}");

    //multiplication

    let mult = 5 * (-25);
    println!("The product is {mult}");

    //division

    let quotient = -5/-5;
    println!("The quotient is {quotient}");

    let remainder = 5%3;
    println!("The remainder is {remainder}");

    let truncated = -5/3; //result = -1 -> rounds towards 0 and not negative infinity
    println!("The value from negative division is {truncated}");

}