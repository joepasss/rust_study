use std::io;
use std::num::ParseFloatError;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter your weight! (kg): ");
        io::stdin().read_line(&mut input).unwrap();

        let weight: Result<f32, ParseFloatError> = input.trim().parse();

        match weight {
            Ok(weight) => {
                println!("Weight on mars: {}", calculate_weight_on_mars(weight));

                break;
            }
            Err(e) => {
                println!("Please input valid number");

                println!("ERR: {}", e);
                continue;
            }
        };
    }
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
