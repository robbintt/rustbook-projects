fn main() {

    let mut row = 1;
    let size = 10;

    let mut last_row: Vec<isize> = vec![1];
    let mut this_row: Vec<isize> = vec![1, 1];

    let mut pascal_triangle: Vec<Vec<isize>> = vec![];

    // never exists as this_row in this implementation
    pascal_triangle.push(last_row.clone());

    while row < size {

        //println!("last_row: {:?}", last_row);

        // how can i pass it instead of clone/clear
        last_row = this_row.clone();
        this_row.clear();
        
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

        // i need to clone here since i pass this_row to last_row
        pascal_triangle.push(this_row.clone());

        row = row + 1;
        //println!("row number: {}", row);

    }

    //println!("this_row: {:?}", this_row);
    println!("{:?}", pascal_triangle);

}
