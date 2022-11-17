fn main() {
    // Variables and Mutability
    /*  Immutable variable
    let x = 5;
    println!("The value of x is: {x}");
    
    x = 6;  // cannot assign twice to immutable variable x
    println!("The value of x is: {x}");
    */

    /* Mutable variable */
    // let mut x = 5;
    // println!("The value of x is: {x}");

    // x = 6;
    // println!("The value of x is: {x}");

    /* Constants */
    // constants are values that are bound to a name and are not allowed to change
    
    // difference between constants and variables
    // allowed to use mut with constants
    // the type of the value must be annotated.
    // const can be any scope (let is cannot assigned in global scope)
    // that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /* Shadowing */
    // you can declare a new variable with the same name as a previous variable.
    // "first variable is shadowed by the second"
    // that the second variable is what the compiler will see when you use the name of the variable, in effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable by using the same variable's name and repeating the sue of the let keyword

    // let s = 5;
    // let s = s + 1;

    // {
    //     let s = s * 2;
    //     println!("The value of s in the inner scope is: {s}");  // 12
    // }

    // println!("The value of s is: {s}"); // 6

    // shadowing is different from marking a variable as mut.
    // by using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    /* DATA TYPES */

    // let guess: u32 = ~~~

    // Scalar Types
    // single value.
    // integers, floating-point numbers, Booleans, characters

    // integer
    // i8 (signed 8 bit) u8 (unsigned 8 bit)
    // arch (isize, usize)
    // depends on architecture of the computer you program is running on

    // floating
    // f64, f32 ...

    // boolean type
    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // character type
    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';

    /* Compound Type */
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");

    let t = ([1; 2], [3; 4]);
    let (a, _) = t;
    println!("{}", a[0] + t.1[0]); 
}
