fn main() {

    another_function(5, 6);

    let y = {
        let x = 3;
        // note the missing semicolon. if you add it this becomes a statement and does not return
        // a value
        x + 1
    // there is a semicolon on the parent statement (statement setting y)
    };

    println!("{}", y);

    let z = five();
    let z1 = plus_one(five());
    println!("The value of z is: {}", z);
    println!("The value of z+1 is: {}", z1);
    

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


// return an expression
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1 // if we put a semicolon on this we would get an error (it would be come a statement)
}
