use std::io;
//Shape calculator

fn main() {
    println!("\tShape Calculator 26.0.1\t");
    println!("Enter required shape to calculate");
    println!("[T] Area of Trapezium");
    println!("[R] Area of Rhombus");
    println!("[P] Area of Parallelogram");
    println!("[C] Surface Area of a Cube");
    println!("[V] Volume of a Cylinder");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice: char = input.trim().parse().expect("Invalid input");

    match choice.to_ascii_uppercase() {
        'T' => println!("Area of Trapezium = {}",trapezium()),
        'R' => println!("Area of Rhombus = {}", rhombus()),
        'P' => println!("Area of Parallelogram = {}", parallelogram()),
        'C' => println!("Surface Area of a Cube = {}", cube()),
        'V' => println!("Volume of a Cylinder = {}", cylinder()),
         _  => println!("Invalid Choice. Pick T, R, P, C, or V."),
    }

}

fn read_number(prompt: &str) -> f64{
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().parse().expect("Invalid input");
}

fn trapezium()->f64 {
    let h = read_number("Enter height:");
    let a = read_number("Enter base a:");
    let b = read_number("Enter base b:");
    ((a + b) * h) / 2.0
    }
fn rhombus()->f64 {
    let d1 = read_number("Enter diagonal length 1:");
    let d2 = read_number("Enter diagonal length 2:");
    (d1 * d2) / 2.0
}
fn parallelogram()->f64 {
    let b = read_number("Enter length of base b:");
    let a = read_number("Enter altitude a:");
    b * a
}
fn cube()->f64 {
    let a = read_number("Enter side length:");
    6.0 * a * a
}
fn cylinder()->f64 {
    let pi: f64 = 22.0 / 7.0;
    let r = read_number("Enter radius of circular base:");
    let h = read_number("Enter height of Cylinder:");
    pi * r * r * h
}
