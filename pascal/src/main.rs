/*
 * I fixed this to pull values off pascal_triangle for constructing the next row.
 * However, now I'm unwrapping the Option instead of handling the None case, which is unsafe.
 * Also, it's super verbose to unwrap the last three times, and i'd rather alias it once, then
 * work with it inside the loop.
 */

fn main() {

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
