use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);  // 秘密の値を生成

    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        println!("Please input your guess:");

        /* 入力 */
        io::stdin()
            .read_line(&mut guess)          // 標準入力から読み取り
            .expect("Faild to read line");  // 失敗時のエラー時メッセージ
        
        /* guessをu32で再定義 */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);

        /* 比較 */
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }

    println!("Secret number: {}", secret_number);
}
