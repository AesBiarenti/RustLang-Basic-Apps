use std::io;

use rand::Rng;
fn main() {
    println!("0 ile 5 arası bir sayı tahmin et");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Okuma Hatası");
        let value: i32 = match input.trim().parse() {
            Ok(num) if (0..=5).contains(&num) => num,
            _ => {
                println!("0 ile 5 arası bir değer girmen gerekiyor. Yeniden Dene");
                return;
            }
        };
        predict_number(value);
    }
}

fn predict_number(value: i32) {
    let mut attempt = 1;
    loop {
        let random_number = rand::thread_rng().gen_range(0..=5);
        if value == random_number {
            println!(
                "Correct prediction {} {} in {}. attempt",
                value, random_number, attempt
            );
            break;
        } else {
            println!("{} {}", value, random_number);
            attempt += 1;
            continue;
        }
    }
}
