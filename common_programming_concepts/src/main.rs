fn main() {
    // Variables and Mutability
    /*  Immutable variable
    let x = 5;
    println!("The value of x is: {x}");
    
    x = 6;  // cannot assign twice to immutable variable x
    println!("The value of x is: {x}");
    */

    /* Mutable variable */
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

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

    let s = 5;
    let s = s + 1;

    {
        let s = s * 2;
        println!("The value of s in the inner scope is: {s}");  // 12
    }

    println!("The value of s is: {s}"); // 6

    // shadowing is different from marking a variable as mut.
    // by using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
}
