extern crate rand;
use  std::io;
use  rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number=rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);    
    loop{
        println!("Please input your guess.");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).ok().expect("field to guess");
        println!("You guessed: {}", guess);
        let guess:u32=match guess.trim().parse(){
             Ok(num) => num,
             Err(_)=>continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("to less"),
            Ordering::Greater=>println!("to large"),
            Ordering::Equal=>{
              println!("equal!");
              return;
            }
        }    
    }    
}
