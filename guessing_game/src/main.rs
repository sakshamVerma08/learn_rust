use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main(){
    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    println!("Please input your guess:");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: i32 = guess.trim().parse().expect("Please enter a number!");

    println!("you guessed {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Large"),
        Ordering::Equal=>println!("You Win !"),
    }


}
