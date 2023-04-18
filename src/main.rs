/*use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /*println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");*/

    /* 
    let mut a = String::new();
    let b = 12;
    let c = 23 + b;

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    println!("x = {a}, y = {b}, y+23={c}");
    */

    /* RAND
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Random number is {secret_number}");

     */

    //RAND 
    //
}*/


extern crate rand;
use rand::Rng;
use std::io; 
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
            
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                wining_handler();
                break;
            }    
        }
    }
}

fn wining_handler() -> i32
{
    return 1;
} 

// commit