use std::io;
fn main() {
    println!("\tAPS LEVER CHECKER AGOs. Public Service Server 1.23.10");
    println!("Welcome to APS Public Service level checker");
    println!("Please enter your name to continue");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name = name.trim();

    let dept_name = loop { 
    println!("Enter Department");
    println!("[O] OFFICE ADMIN");
    println!("[A] ACADEMIC");
    println!("[L] LAWYER");
    println!("[T] TEACHER");

    let mut dept = String::new();
    io::stdin().read_line(&mut dept).expect("Failed to read input");
    let dept = dept.trim().to_uppercase();
   
    match dept.as_str() {
        "O" | "OFFICE ADMIN" => break "OFFICE ADMIN",
        "A" | "ACADEMIC" => break "ACADEMIC",
        "L" | "LAWYER" => break "LAWYER",
        "T" | "TEACHER" => break "TEACHER",
        _ => println!("Sorry, incorrect format. Please try again."),
    }
};  
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    if dept_name == "OFFICE ADMIN"{ 
    let office = vec!["INTERN", "ADMINISTRATOR", "SENIOR ADMINISTRATOR", "OFFICE MANAGER", "DIRECTOR", "CEO"];
    let position = loop {
        println!("Select Role {}", dept_name);
        println!("[I] INTERN");
        println!("[A] ADMINISTRATOR");
        println!("[S] SENIOR ADMINISTRATOR");
        println!("[O] OFFICE MANAGER");
        println!("[D] DIRECTOR");
        println!("[C] CEO");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice = choice.trim().to_uppercase();

        match choice.as_str() {
            "I" => break 0,
            "A" => break 1,
            "S" => break 2,
            "O" => break 3,
            "D" => break 4,
            "C" => break 5,
            _=> println!("Sorry, Incorrect format. Please try again."),

        };
            };
    println!("Name: {}", name);
    println!("Department: {}", dept_name);
    println!("Role: {}", office[position]);
    println!("APS Level: {}", levels[position]);

    };

     if dept_name == "LAWYER" {
    let lawyer = vec!["PARALEGAL", "JUNIOR ASSOCIATE", "ASSOCIATE", "SENIOR ASSOCIATE 1-2", "SENIOR ASSOCIATE 3-4", "PARTNER"];
    let position1 = loop {
        println!("Select Role {}", dept_name);
        println!("[P] PARALEGAL");
        println!("[J] JUNIOR ASSOCIATE");
        println!("[A] ASSOCIATE");
        println!("[S] SENIOR ASSOCIATE 1-2");
        println!("[S1] SENIOR ASSOCIATE 3-4");
        println!("[PA] PARTNER");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().to_uppercase();

        match choice.as_str() {
            "P" => break 0,
            "J" => break 1,
            "A" => break 2,
            "S" => break 3,
            "S1" => break 4,
            "PA" => break 5,
            _ => println!("Sorry Incorrect format. Please try again"),
        };
    };
    println!("Name: {}", name);
    println!("Department: {}", dept_name);
    println!("Role: {}", lawyer[position1]);
    println!("APS Level: {}", levels[position1]);
        }

    if dept_name == "ACADEMIC"{
    let academic = vec!["-", "RESEARCH ASSISTANT", "PhD CANDIDATE", "POST-DOC RESEARCHER", "SENIOR LECTURER", "DEAN"];
    let position2 = loop {
        println!("Select Role {}", dept_name);
        println!("[R] RESEARCH ASSISTANT");
        println!("[P] PhD CANDIDATE");
        println!("[PD] POST-DOC RESEARCHER");
        println!("[S] SENIOR LECTURER");
        println!("[D] DEAN");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().to_uppercase();

        match choice.as_str() {
            "R" => break 0,
            "P" => break 1,
            "PD" => break 2,
            "S" => break 3,
            "D" => break 4,
            _=> println!("Sorry Incorrect format. Please try again"),
        };
    };
    println!("Name: {}", name);
    println!("Department: {}", dept_name);
    println!("Role: {}", academic[position2]);
    println!("APS Level: {}", levels[position2]);
    }

    if dept_name == "TEACHER"{
    let teacher = vec!["PLACEMENT", "CLASSROOM TEACHER", "SENIOR TEACHER", "LEADING TEACHER", "DEPUTY PRINCIPAL", "PRINCIPAL"];
    let position3 = loop {
        println!("Select Role {}", dept_name);
        println!("[P] PLACEMENT");
        println!("[C] CLASSROOM TEACHER");
        println!("[S] SENIOR TEACHER");
        println!("[L] LEADING TEACER");
        println!("[D] DEPUTY PRINCIPAL");
        println!("[PI] PRINCIPAL");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().to_uppercase();

        match choice.as_str() {
            "P"=> break 0,
            "C" => break 1,
            "S" => break 2,
            "L" => break 3,
            "D" => break 4,
            "PI" => break 5,
            _=> println!("Sorry Incorrect format. Please try again"),
        };
    };
    println!("Name: {}", name);
    println!("Department: {}", dept_name);
    println!("Role: {}", teacher[position3]);
    println!("APS Level: {}", levels[position3]);
    }
}