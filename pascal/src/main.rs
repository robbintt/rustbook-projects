fn main() {

    let mut row = 1;
    let size = 10;

    let mut last_row: Vec<isize> = vec![1];
    let mut this_row: Vec<isize> = vec![1, 1];

    let mut pascal_triangle: Vec<Vec<isize>> = vec![];


    while row < size {

        pascal_triangle.push(last_row);
        last_row = this_row;
        this_row = Vec::new();
        // no matter what we need to initialize 1 vec per row of the triangle
        //
        // i think you can do this by adding the vector to the pascal_triangle
        // vector of vectors, then using drain from old to triangle and from
        // new to old....
        // 
        // the method i selected passes each Vec along instead
        // then initializes a new Vec at the tip of generation
        // i think this is the best method
        //
        // assuming we don't have to store the triangle, we then would
        // not need to initialize a vec...

        // this method used clone/clear which seems fine too
        // it still only creates one object.
        // i don't know how costly clone is compared to making
        // a new empty Vec
        //
        //last_row = this_row.clone();
        //this_row.clear();
        
        // initialize this_row
        this_row.push(1);

        // construct a new this_row from last_row's sums
        let mut i = 0;
        // if the vec has 5 elements, we want to grab addresses 3, 4 at max.
        while i+2 <= last_row.len() {
            this_row.push(&last_row[i]+&last_row[i+1]);
            i = i+1;
        }

        // finalize this_row
        this_row.push(1);

        row = row + 1;
    }

    // push the last objects
    pascal_triangle.push(last_row);
    pascal_triangle.push(this_row);

    println!("{:?}", pascal_triangle);

}
