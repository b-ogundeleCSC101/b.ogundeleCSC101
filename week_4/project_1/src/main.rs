//Rust program to calculate the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d = b * b - 4.0 * a * c;
    println!("The discriminant is: {}", d);
    if d > 0.0 {
        // two distinct roots
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b + d.sqrt()) / (2.0 * a);

        println!("The first root is: {}", x1);
        println!("The second root is: {}", x2);
    } 
    else if d == 0.0 {
        // one real root
        let x = -b / (2.0 * a);
        println!("There is one real root.");
        println!("The root is: {}", x);
    }
    else {
        // no real roots
        println!("There are no real roots.");
    }
}
