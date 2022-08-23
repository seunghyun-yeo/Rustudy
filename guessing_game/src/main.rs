use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the nubmer!");
    // generate random number in between 1 ~ 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {secret_number}");
    loop{
        println!("Please input your guess.");
        
        // create mutable variable named guess with string type
        let mut guess = String::new();

        // read line from console and store it to guess, if it fails print string passed to .expect()
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        // use shadow variable for guess and set it as unsigned 32bit inteager
        // make string guess empty and change it into different type of var ie. inteager
        // match will check the return of guess.trim().parse()
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
        
        // compare guess and secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
