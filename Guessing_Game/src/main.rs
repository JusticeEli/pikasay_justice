use rand::Rng;
use std::cmp::Ordering;
use colour::*;




fn main() {

    ///generates random number from 1 to 100
    let random_number = rand::thread_rng().gen_range(1, 101);
    ///prints the random number
    blue!("The generated random number is: ");
    yellow_ln!("{}", random_number.to_string());
   // println!("{} :{}", "The generated random number is".blue(), random_number.to_string().as_str().yellow());
    loop {
       // println!("{}", "Please guess a number in the range 1 -> 100".blue());
        blue_ln!("Please guess a number in the range 1 -> 100");
        let mut guess = String::new();
        let guess: u8 = match std::io::stdin().read_line(&mut guess) {
            Ok(_) => match guess.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    //user did not input a number
                   // eprintln!("Error:{}", e.to_string().as_str().red());
                    red_ln!("Error:{}",e.to_string());
                    continue;
                }
            }
            Err(_) => {
                //internal system error when input value to std input
                //eprintln!("{}", "Error:Trying to retrieve input from user".red());
                red_ln!("Error:Trying to retrieve input from user");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => {
               // eprintln!("{}", "You guess is less than the random number".red())
                red_ln!("You guess is less than the random number")
            }
            Ordering::Equal => {
               // eprintln!("{}", "You won !!!".green());
                green_ln!("You won !!!");
                break;
            }
            Ordering::Greater => {
               // eprintln!("{}", "You guess is greater than the random number".red())
                red_ln!("You guess is greater than the random number")
            }
        }
    }
}
