/*
 * I fixed this to pull values off pascal_triangle for constructing the next row.
 * However, now I'm unwrapping the Option instead of handling the None case, which is unsafe.
 * Also, it's super verbose to unwrap the last three times, and i'd rather alias it once, then
 * work with it inside the loop.
 *
 * Some other ideas:
 *  - write a recursive function to deliver the nth row of pascal's triangle
 *
 *
 *  trent: i think i will write a write a function that calculates the nth row of pascal's 
 *  triangle recursively and use that with map, and maybe use the actual formula to calculate 
 *  the nth row non-recursively which is sort of trivial
 */

extern crate num_bigint;
extern crate ramp;

use ramp::Int;

use std::thread;
use std::time::Duration;

fn main() {

    // generate a full triangle and keep it all around
    // full_triangle();
    
    // see biggest of each type from: https://www.reddit.com/r/rust/comments/6gxvs2/big_numbers_in_rust/
    // println!("{}", std::u64::MAX);
    // println!("{}", std::u128::MAX);

    // int works up to 67
    // bigint works up to 132
    // ramp should work forever but is not a static type so need to use borrow
    // gets slow around 2000 or so depending on your definition
    let size = 10000; 

    // generate a bunch of rows in place - don't keep the previous row
    let mut current_triangle_row = Vec::new();
    while current_triangle_row.len() < size {
        current_triangle_row = build_next_row_pascal_triangle(current_triangle_row);
        //thread::sleep(Duration::from_millis(100));
        //println!("{:?}", current_triangle_row);
        println!("{}", current_triangle_row.len());
    }

}

/*
fn get_nth_pascal_triangle_row (n: u64) -> Vec<u64> {

    /* unravel the full_triangle function to have a recursive component
     * atomic function will generate the next row from a given row
     */

}
*/


fn build_next_row_pascal_triangle (base_row: Vec<Int>) -> Vec<Int> {

    /* needs bigint for triangles over 66 deep */

    let mut next_row = Vec::new();


    // if you want to use Int in the match statement you must instantiate them first
    // it would be nice if match accepted an expression like this: { Int::from(0) }
    // i no longer use them here though
    let Int_1 = Int::from(1);

    match base_row.len() {
        0 => { 
            next_row.push(Int::one());
        }
        1 => { 
            next_row.push(Int::one());
            next_row.push(Int::one());
        },
        // put the row builder here
        _ => {
            // left element of row is 1 
            next_row.push(Int::one());

            // construct a new this_row from last_row's sums
            let mut i = 0;
            // if the vec has 5 elements, we want to grab up to indexes (3, 4) 
            while i+2 <= base_row.len() {
                // I had to move these to 2 lines because i wasn't able to access pascal_triangle as
                // mutable inside of accessing pascal_triangle as immutable
                // or maybe it was immutable inside mutable, i forget
                // 20180908 is this true when these vecs are not in the same parent vec?
                // println!("{}", base_row[i]);
                let nextval = &base_row[i]+&base_row[i+1];
                next_row.push(nextval);
                i = i+1;
            }

            // right element of row is 1 
            next_row.push(Int::one());
        }
    }

    next_row

}

fn full_triangle () {

    let mut row = 1;
    let size = 10;

    // instead of ::new() lets try ::with_capacity() - since we know our expected capacity up front
    let mut pascal_triangle: Vec<Vec<isize>> = Vec::with_capacity(size);

    pascal_triangle.push(vec![1]);

    while row <= size {

        pascal_triangle.push(Vec::new());

        // store len to generate indeces for last and this Vec objects in pascal_triangle
        let pt_len = pascal_triangle.len();
        // improve readability by calculating the actual values for last and this indeces.
        let last = pt_len - 2;
        let this = pt_len - 1;

        // left element of row is 1 
        &pascal_triangle[this].push(1);

        // construct a new this_row from last_row's sums
        let mut i = 0;
        // if the vec has 5 elements, we want to grab up to indexes (3, 4) 
        while i+2 <= pascal_triangle[last].len() {
            // I had to move these to 2 lines because i wasn't able to access pascal_triangle as
            // mutable inside of accessing pascal_triangle as immutable
            // or maybe it was immutable inside mutable, i forget
            let nextval = &pascal_triangle[last][i]+&pascal_triangle[last][i+1];
            &pascal_triangle[this].push(nextval);
            i = i+1;
        }

        // right element of row is 1 
        &pascal_triangle[this].push(1);

        row = row + 1;
    }

    // display the triangle
    for row in pascal_triangle {
        println!("{:?}", row);
    }

}


