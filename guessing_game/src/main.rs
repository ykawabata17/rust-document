use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faile to read line");              // 行の読み込みに失敗しました

    println!("You guess {}", guess);
}