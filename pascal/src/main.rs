/*
 * I fixed this to pull values off pascal_triangle for constructing the next row.
 * However, now I'm unwrapping the Option instead of handling the None case, which is unsafe.
 * Also, it's super verbose to unwrap the last three times, and i'd rather alias it once, then
 * work with it inside the loop.
 */

fn main() {

    let mut row = 1;
    let size = 10;

    let mut pascal_triangle: Vec<Vec<isize>> = Vec::new();


    pascal_triangle.push(vec![1]);

    while row <= size {

        let mut this_row = Vec::new();

        // initialize this_row
        // ideally i would toss this right on pascal_triangle and be able to directly access
        // the second to last element of pascal_triangle as well.
        this_row.push(1);

        // construct a new this_row from last_row's sums
        let mut i = 0;
        // if the vec has 5 elements, we want to grab up to indexes (3, 4) 
        while i+2 <= pascal_triangle.last().unwrap().len() {
            this_row.push(&pascal_triangle.last().unwrap()[i]+&pascal_triangle.last().unwrap()[i+1]);
            i = i+1;
        }

        // finalize this_row
        this_row.push(1);

        pascal_triangle.push(this_row);
        row = row + 1;
    }

    // display the triangle
    for row in pascal_triangle {
        println!("{:?}", row);
    }
    //println!("{:?}", pascal_triangle);

}
