use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    let mut how_many = String::new();
    println!("How many random number do you want yo Guess?");
    io::stdin()
    .read_line(&mut how_many)
    .expect("Error reading input");

    let num_guesses:u8 = how_many.trim().parse().expect("Error reading input");
    
    let mut correct=Vec::new();
    for _ in 0..num_guesses{
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    
    println!("correct {correct:?}!");
    
    let mut guesses_made=0;
    while guesses_made<num_guesses {
        println!("Guess the number 1..10!");    
        
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
        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater =>println!("You guess to high"),
            Ordering::Less =>println!("You guess to low"),
            Ordering::Equal =>{
                println!("You guess: correct");
                guesses_made += 1;
                if guesses_made<num_guesses{
                    println!("One more play");
                }
            }
        }; 
    }
    println!("thanks for playing. The correct answers were:");
    for item in correct{
        println!("{item}");
    }
}

