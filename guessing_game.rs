use std::io;
pub fn guess(){
    println!("Guess the correct number");
    println!("Enter a number between 1 an 10");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error in reading input");
    println!("The number you guessed is - {}",number);

}