use std::io; 

fn main() {
    println!("\tERNST & YOUNG (EY) GLOBAL LIMITED LOGISTICS INTERVIEW PROGRAM\t");
    println!("Please Enter Your Name to Continue");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();

    let years = loop{
    println!("Hello {}, How many years of experience do you have to qualify for this job?", name);
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Failed to read input");
    match years.trim().parse::<u32>(){
        Ok(number) => break number,
        Err(_) => println!("Incorrect format. Try again but with numbers only."),
    }
    println!("{}, No of Years of Experience : {}", name, years);
        };

    let mut people = vec![("Ada", 4), ("Chidi", 9), ("Funke", 6), ("Emeka", 12),
     ("Tola", 3), ("Ngozi", 7), ("Ibrahim", 15), ("Zainab", 5),
     ("Kunle", 8), ("Amaka", 11), ("Segun", 2), ("Halima", 10),
     ("Obi", 13), ("Bisi", 1), ("Yusuf", 14), ("Chioma", 6),
     ("Tunde", 9), ("Fatima", 4), ("Uche", 7), ("Damilola", 5)
    ];
    
       let mut position = 1;
       for i in 0..people.len() {
        if people[i].1 > years  {
            position += 1;
        }
   }

   people.push((name, years));
   people.sort_by(|a, b| b.1.cmp(&a.1));            //used ai here to make my ranking position start from the highest to the lowest.
   println!("Highest Experience Ranks:");
   for i in 0..people.len() {
    println!("{}. {} - {} years", i + 1, people[i].0, people[i].1);
   }
        if position == 1{
            if people[0].0 == name {
                println!("You are: 1st position, with {} years!", years);
            } else {
                println!("Tie with {} for the highest experience ({} years)", people[0].0, people[0].1);
                }
            }
             else {
                println!("Person with the Highest Experience: {} with {} years.", people[0].0, people[0].1);
            }
            
   println!("You are: {}th ranking position with {} experience", position, years);
   println!("Thank you for participating. Hope to see you soon.");
}

