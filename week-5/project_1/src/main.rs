use std::io;

fn main() {

    let mut p_portions:u32 = 0;
    let mut f_portions:u32 = 0;
    let mut a_portions:u32 = 0;
    let mut e_portions:u32 = 0;
    let mut w_portions:u32 = 0;

    let mut confirmed = false;
    while !confirmed{

    println!("Hello User, Welcome to Bukka Hot Pot.");
    println!("Today's Menu");
    println!("[P]\tPoundo Yam & Edinkaiko Soup\tN3,200");
    println!("[F]\tFried Rice & Chicken\tN3,000");
    println!("[A]\tAmala & Ewedu Soup\tN2,500");
    println!("[E]\tEba & Egusi Soup\tN2,000");
    println!("[W]\tWhite Rice & Stew\tN2,500");


    let mut adding_more = true;
while adding_more {

    //user is making selection
    let mut choice = String::new();
    println!("What would you like? Indicate by selecting [P,F,A,E or W]:");
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    println!("Item selected: {}", choice.trim());

    //asking user how many portions
    println!("How many Portions would like?");
    let mut portions_text = String::new();
    io::stdin().read_line(&mut portions_text).expect("Failed to read line");
    let portions:u32 = portions_text.trim().parse().expect("Please type a number");
    
    let letter = choice.trim().to_uppercase();

    if letter == "P" {
        println!("Poundo Yam & Edinkaiko Soup");
        p_portions = p_portions + portions;
    }
    else if letter == "F" {
        println!("Fried Rice & Chicken");
        f_portions = f_portions + portions;
    }
     else if letter == "A" {
        println!("Amala & Ewedu Soup");
        a_portions = a_portions + portions;
    }
     else if letter == "E" {
        println!("Eba & Egusi Soup");
        e_portions = e_portions + portions;
    }
     else if letter == "W" {
        println!("White Rice & Stew");
        w_portions = w_portions + portions;
    }
    else {
        println!("Invalid choice");
    }
    println!("Would you like to make another selection? (yes/no)");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");

    let answer = answer.trim().to_lowercase();
    if answer == "no" {
        adding_more = false;
    }
        }

println!("Poundo Yam portions: {}", p_portions);
println!("Fried Rice portions: {}", f_portions);
println!("Amala portions: {}", a_portions);
println!("Eba portions: {}", e_portions);
println!("White Rice portions: {}", w_portions);

println!("You have selected:");
    if p_portions > 0 {
        println!("Poundo Yam & Edinkaiko x{} = N{}", p_portions, p_portions * 3200);
    }
       if f_portions > 0 {
        println!("Fried Rice & Chicken x{} = N{}", f_portions, f_portions * 3000);
       } 
            if a_portions > 0 {
                println!("Amala & Ewedu Soup x{} = N{}", a_portions, a_portions * 2500);
            }
                if e_portions > 0 {
                    println!("Eba & Egusi Soup x{} = N{}", e_portions, e_portions * 2000);
                }
                    if w_portions > 0 {
                        println!("White Rice & Stew x{} = N{}", w_portions, w_portions * 2500);
                    }
            println!("Would you like to proceed (yes) or make new changes (no)?");
            let mut proceed_answer = String::new();
            io::stdin().read_line(&mut proceed_answer).expect("Failed to read line");

            let proceed_answer = proceed_answer.trim().to_lowercase();

        if proceed_answer == "yes" {
            confirmed = true;
        }
            else {
               println!("Would you like to edit your selection or clear it? (edit/clear)");
               let mut change_answer = String::new();
               io::stdin().read_line(&mut change_answer).expect("Failed to read line");

               let change_answer = change_answer.trim().to_lowercase();

               if change_answer == "clear" {
                p_portions = 0;
                f_portions = 0;
                a_portions = 0;
                e_portions = 0;
                w_portions = 0;
               }
            }
    }
    let total:u32 = (p_portions * 3200) + (f_portions * 3000) + (a_portions * 2500) + (e_portions * 2000) + (w_portions * 2500);
    println!("Your Total is: N{}", total);

    let mut amount_to_pay = total;

    if total > 10_000 {
        let discount = (total * 5) / 100;
        let discounted_total = total - discount;
        amount_to_pay = discounted_total;

        println!("With a 5% discount for our new Customer: N{}", discount);
        println!("Your discounted total is: N{}", discounted_total);
    }
    else{
        println!("No discount applied. To get our 5% discount on your purchases, get an order worth over N10,000 now!")
    }

    println!("Press ENTER to continue to payment, or press b to stop and start over.");
    let mut continue_key = String::new();
    io::stdin().read_line(&mut continue_key).expect("Failed to read input");
    let continue_key = continue_key.trim().to_lowercase();
    if continue_key == ""{
        println!("Processing payemnt...");
        println!("Pick Payment option: Bank Transfer(B) or Digital Wallet(D)");
        let mut pay_choice = String::new();
        io::stdin().read_line(&mut pay_choice).expect("Failed to read input");
        let pay_choice = pay_choice.trim().to_lowercase();

        if pay_choice == "b" {
            println!("Pay N{} to:", amount_to_pay);
            println!("Account number: 9157597309");
            println!("Bank: Opay or Moniepoint");
            println!("Type OK to confirm transaction");

            let mut confirm_ok = String::new();
            io::stdin().read_line(&mut confirm_ok).expect("Failed to read line");
        }
        else if pay_choice == "d" {
            println!("Pay with Apple Pay(A) or Google pay (G)?");

            let mut wallet_choice = String::new();
            io::stdin().read_line(&mut wallet_choice).expect("Failed to read line");
            println!("Enter OK to proceed");
            let mut wallet_ok = String::new();
            io::stdin().read_line(&mut wallet_ok).expect("Failed to read line");

        }
        else{
            println!("Invalid option");
        }
        println!("Payment received and confirmed! Thank you for patronizing Bukka Hot Pot.")
    }
    else if continue_key == "b" {
        println!("Starting over...");
    } 


}
