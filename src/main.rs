use std::io;

fn main() {
    println!("Hello, world!");
    println!("Devine le nombre!");

    println!("Entre un nombre à deviner");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    
    
}


