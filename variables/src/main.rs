fn main() {


    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    // shadowing variables
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    // shadowing with a new type is fine
    let spaces = "    ";
    let spaces = spaces.len();

    println!("There are {} spaces.", spaces);
}
