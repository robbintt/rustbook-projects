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

    // can toss values in with default types per value
    //let tup = (500, 6.4, 1);
    // can specify types too
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tuple destructuring
    let (ax, ay, az) = tup;

    // destructure tuple - use a .i where i is the target index
    let bx = tup.0;
    let by = tup.1;
    let bz = tup.2;

    println!("The values of ax,ay,az are: {}, {}, {}", ax, ay, az);

    println!("The values of bx,by,bz are: {}, {}, {}", bx, by, bz);

    // array allocated on stack, not heap
    let array_a = [1, 2, 3, 4, 5];

    let first = array_a[0];
    let second = array_a[1];

    println!("First two elements of array_a: {}, {}", first, second);

    // if you aren't sure whether to use array or vector, use vector
    // vector is allowed to grow or shrink in size...

    // if you specify an out of bounds array index the code 
    // then you get a compile error
    // ''' error: index out of bounds: the len is 5 but the index is 10 '''
    // let tenth = array_a[10];

    // if you specify an out of bounds array index as a variable
    // then it will only be caught at runtime, it will compile fine...
    // ''' thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:56:17 ''' 
    //  so this is 'safe mode' and this memory cannot be accessed
    //  i assume if you run in unsafe mode then you can access this memory...
    let ten = 10;
    let tenth = array_a[ten];

    println!("Tenth element of array_a: {}", tenth);

}
