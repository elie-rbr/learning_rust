use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("nombre secret : {}", secret_number);

    println!("rentrez une suggestion:");

    let mut guess= String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {}", guess);
}
