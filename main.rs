use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
	println!("Secret number is: {secret_number}");    
    
   loop {
	 println!("Input your guess...");
    let mut guess = String::new(); //the second half is a fucntion which returns a new instance of
                                   //a string
    //creates empty string that is mutable.
	
	io::stdin() //using the already imported io crate we call the function stdin().
        .read_line(&mut guess)
        .expect("Failed to read line.");
    //reads the line from user and references guess, mutating it to the users input.
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".cyan()),
        Ordering::Equal => { 
            println!("{}", "You win!".green());
            break;
        }
    }
    }
}
