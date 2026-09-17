// Rust program as an incentive calculator

use std::io;

fn main() {
    //age
    let mut input = String::new();
    println!("\nEnter Age");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let age:f32 = input.trim().parse().expect("Not a valid number");

    //experience
    let mut experience_input = String::new();
    println!("\nDo you have experience? (Yes/No)");
    io::stdin().read_line(&mut experience_input).expect("Not a valid string");
    let experienced = experience_input.trim().to_lowercase();

    //Annual Incentive

    let input1 = 1_560_000.0;
    let input2 = 1_480_000.0;
    let input3 = 1_300_000.0;
    let input4 = 100_000.0;

    if age >= 40.0 && experienced == "yes"
    {
        println!("Annual incentive is: {}", input1);
    }
    else if age >=30.0 && age <=39.0 && experienced == "yes"
    {
        println!("Annual Incentive is: {}", input2);
    }
    else if age <28.0 && experienced == "yes"
    {
        println!("Annual Incentive is: {}", input3);
    }
    else if experienced == "no"
    {
        println!("Annual Incentive is: {}", input4);
    }
    else {
        println!("Please enter Yes or No for experience.");
    }
}
