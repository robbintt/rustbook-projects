fn main() {

    // expression returning to a statement
    let expression_result = {
        // statement wrapped in an expression
        println!("is this wrapped in an expression? yes!");
        let local_variable = 5;
        // return the variable from its expression's scope
        local_variable
    };
    println!("An expression inside of an assignment statement: {}", expression_result);

}

