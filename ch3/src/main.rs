fn main() {

    //
    // IMMUTABILITY DEFAULTS
    //
    let x = 5;
    println!("Since we have not used the 'mut' keyword, x is bound to a single value. Attempts to re-assign this value willl produce a compilation error. The value of x is: {}\n", x);

    let mut a = "The value bound to the variable 'a' within this scope must always be a string. Since we have used the 'mut' keyword, the type of this binding cannot change.";
    println!("Unused variables produce a compiler warning, so we will use 'a': {}",  a);
    a = "We are changing the value of 'a' here to avoid the compiler warning that 'a' would not otherwise need to be mutable.";
    println!("{}", a);

    //
    // CONST KEYWORD
    //
    const Y: u8 = 255;
    println!("The 'const' keyword requires a type, and cannot be used in tandem with the 'mut' keyword. Furthermore, the Rust compiler will warn on non-capitol const variable names, as to encourage the standard. The value of y is {}\n", Y);

    //
    // SHADOWING
    //
    let x = x + 1;
    let x = x * 2;
    println!("We may 'shadow' a variable that has already been declared with the 'let' keyword. This creates a new variable in memory bound to the same name, allowing the developer to change the value and type bound to this variable name. Rust is smart enough to be able to reference previous values bound to the name until the assignment is final. The value of x is: {}", x);

    let z = "    ";
    let z = z.len();
    println!("We have changed the type of 'z' from a string to an int. The original string had {} spaces.", z);

    //
    // DATA TYPES
    // Two kinds: scalar && compound
    //

    // SCALAR DATA TYPES
    //   Represent a single value. Types include: int, fpoint, bool, char.
    //   8bit, 16bit, 32bit, 64bit, arch (isize && usize)
    {
        let x = 2.0;       // f64 (default)
        let y: f32  = 3.0; // f32
        
    }
    
    // COMPOUND DATA TYPES
    //   Two primitive compound data types: tuples && arrays
    {
        let tuple: (i32, f64, u8) = (500, 6.4, 1); // 
        let tuple2 = (2.0, 2, "two");              // type specification is optional
        let (x, y, z) = tuple;
        println!("The value of y is: {}", y);
        println!("The value of the third index of tuple2 is: {}", tuple2.2);
    }
    
}
