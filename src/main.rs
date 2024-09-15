use std::io;
use rand::Rng; // trait that defines methods that random number generators implement
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println! ("Please input your guess: ");

        let mut guess = String::new(); // creates a mutable variable that is bound to a new empty instance of a string

        io::stdin()
            .read_line(&mut guess) // calls the read_line method on the standard input handle to get input form the user
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue, // require user to give correct input
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // this allows us to break the loop
            },
        } // since the cmp method returs a variant whose type may be any of the three offered by the enums of Ordering, we ue a match expression to decide on what action to take next
    }
}
