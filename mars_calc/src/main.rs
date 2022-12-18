use std::io;

fn main() {
    let mut input = String::new(); // owned by input variable (heap)

    io::stdin().read_line(&mut input);

    borrow_string(&input);
    own_string(input);

    // println!("input: {}", input);
    // let mars_weight: f32 = calculate_weight_on_mars(100.0);
    // println!("Weight on Mars: {}kg", mars_weight);
}

fn borrow_string(s: &String) {
    println!("{}", s);
}

fn own_string(s: String) {
    println!("{}", s);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
