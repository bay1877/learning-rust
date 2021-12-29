use std::io;

fn main() {    
    println!("Enter your name to say hello: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Hello {} nice to meet you!", guess.trim_end());
}