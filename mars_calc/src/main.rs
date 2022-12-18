use std::io;

fn main() {
    let input = String::new(); // owned by input variable (heap)
    let mut s = input; // input is no longer valid because string is complex type that value is cannot be copy it only can move.

    /*
    let a = 5;
    let b = a;  // a's value is copied
    println!("{}", b);
    */

    some_fn(&s);
    some_mut_fn(&mut s);

    io::stdin().read_line(&mut s);
    let mars_weight: f32 = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// OWNERSHIP

// 1. Each value in Rust is owned by a variable.

// 2. When the owner goes out of scope, the value will be deallocated.

// 3. There can only be ONE owner at a time.

fn some_fn(s: &String) {} // function expect reference   (borrow)
                          // reference is immutable for default
                          // it can be used attaching "mut" keyword

fn some_mut_fn(s: &mut String) {}
