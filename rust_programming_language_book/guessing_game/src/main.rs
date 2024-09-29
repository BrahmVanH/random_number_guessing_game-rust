use rand::Rng;
use std::io::{ self, BufRead, Error };
use std::cmp::Ordering;

fn main() -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(1..=100);
    let mut user_guess = String::new();
    println!(
        "i have chosen a random number between 1 and 100. can you guess what it is? enter your answer..."
    );
    println!("The random number is {}", &rand_num);

    let mut stdin = io::stdin().lock();
    loop {
        user_guess.clear();
        stdin.read_line(&mut user_guess)?;
        let user_guess_int = match user_guess.trim().parse::<i32>() {
            Ok(int) => int,
            Err(e) => {
                println!("guess again...");
                user_guess.clear();
                continue;
                // return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
            }
        };
        println!("user guess string: {}", &user_guess);
        println!("user guess parsed: {}", &user_guess_int);

        match user_guess_int.cmp(&rand_num) {
            Ordering::Equal => {
                println!("huzzah! you guessed correctly. the target number was {:?}", &rand_num);
                break;
            }
            Ordering::Greater => {
                println!("incorrect, the target number is lower than your guess. guess again...");
                continue;
            }
            Ordering::Less => {
                println!("incorrect, the target number is higher than your guess. guess again...");
                continue;
            }
        }

     
    }

    Ok(())
}
