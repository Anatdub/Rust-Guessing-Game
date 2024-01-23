use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct {correct}!");
    println!("Guess the number 1..10!");
    
    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(e)=>{
                println!("Error parcing-try again {e}");
                continue;
             }
        };
        
        /* let  message: &str =  if correct < guess {
            // String::from("You guess to high")
            "You guess to high"
        }else if correct > guess{
            // String ::from("You guess to low")
            "You guess to low"
        } else {
            // String::from("You guess: correct")
            "You guess: correct"
        }; */
        let  message =  match guess.cmp(&correct) {
            Ordering::Greater =>"You guess to high",
            Ordering::Less =>"You guess to low",
            Ordering::Equal =>{
                println!("You guess: correct");
                break;
            }
        }; 
        println!("{message}");
    }
}

