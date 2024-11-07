use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    println!("Number guessing game");
    let secret_nuber=rand::thread_rng().gen_range(1..=100);
    // println!("secret_nuber={}",secret_nuber);

    println!("enter the number");
    let mut  guess=String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("failed");

    if guess!=secret_nuber.to_string()
    {
        if guess>secret_nuber.to_string(){
            println!("your guess is {guess}, which is greater then secret number ");
        }
        if guess<secret_nuber.to_string(){
            println!("your guess is {guess}, which is smallar then secret number ");
        }
        
    }else{
        println!(
            "your guess={guess} is correct, secret number is {secret_nuber} "
        );
    }
    println!("your guessed number is {}",guess);
}
