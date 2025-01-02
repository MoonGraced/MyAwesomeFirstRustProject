fn main() {
    println!("Guess the number!");
    let secret: u32 = rand::random_range(1..=100);
    // println!("Number {secret}");
    loop {
        let mut guess_number: String = String::new();
        println!("Please input your guess!");
        std::io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_number.cmp(&secret) {
            std::cmp::Ordering::Less => {
                println!("{guess_number} is oo small!");
            },
            std::cmp::Ordering::Equal => {
                println!("Yay! Equal!");
                break;
            },
            std::cmp::Ordering::Greater => {
                println!("Yikes! {guess_number} is too big!");
            },
        }
    }
}
