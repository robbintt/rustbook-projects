fn main() {
    let number = 3;
    //let number = 7;

    // blocks of code inside if expressions or match expressions are sometimes called `arms`
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // condition must be bool but this `if` expression evaluates to 3 - (error: mismatched types)
    if number != 0 {
        println!("number was something other than 0");
    }

    // better to use match for many conditions, this is too verbose
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    // rust only executes the first true condition, so this is skipped
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // use any expression inside a let statement to assign a variable, e.g. an if-else block.
    let number = if condition {
        5
    } else {
        6
        // "six" doesn't work because the types aren't consistent/compatible in the blocks/arms of the
        // expression
        //"six"
    };
    println!("The value of the number is: {}", number);

    loop {
        println!("again!");
        break; // without this, you need to break or ctrl-c to get out of a `loop`
    }

}
