use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number game");

    let rand_number:i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a number.");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Something went wrong with reading");
        let input: i32 = match input.trim().parse(){
            Ok(val) => val,
            Err(_) => continue
        };
        println!("the guess is {}", input);

        match rand_number.cmp(&input) {
            Ordering::Equal => {
                println!("Congratulations you won!");
                break;
            },
            Ordering::Greater => println!("Too small!"),
            Ordering::Less => println!("Too big!")
        }
    }
}
