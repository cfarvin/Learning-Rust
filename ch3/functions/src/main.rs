fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn blocks_are_expressions() {
    let y = {
        let x = 3;
        x + 1
        // [ NOTE ] No ; here. A ; would turn this into a statement.
        // Statements do not return values.
    };

    println!("The value of y is {}", y);
}

fn functions_with_return_values() -> i32 {
    5
    // [ NOTE ] Just to drive things home, notice how there's no ; here.
    // Additionally, rust defaults to returning the last-evaluated expression.
    // Returning like "return RETVAL" works, too.
}

fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    blocks_are_expressions();
    let z = functions_with_return_values();

    println!("The value of z is {}", z);
}
