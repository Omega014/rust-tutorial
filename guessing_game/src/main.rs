extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // ここで標準出力を求める
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // guessの定義を新しいもので隠す(シャドーイング)
        // string型からu32の数値型へ定義を上書きしている
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // 数値比較
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