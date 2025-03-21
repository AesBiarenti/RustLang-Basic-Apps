use std::io;

fn main() {
    let mut first_input = input("İlk Değeri Gir: ");
    loop {
        let opt = select_operator("Operatör seç: +  -  *  / Çıkış yapmak için 'q' ya bas");
        if opt == "q" {
            println!("İşlem Bitirilmeden Sonlandırıldı. Sonuç: {}", first_input);
            break;
        }
        let second_input = input("İkinci Değeri Gir: ");
        first_input = calculate(first_input, second_input, &opt);
        println!("Güncellenmiş Sonuç: {}", first_input);
    }
}

fn input(prompt: &str) -> f64 {
    loop {
        let mut first_value = String::new();
        println!("{}", prompt);
        io::stdin()
            .read_line(&mut first_value)
            .expect("Okuma Hatası");
        match first_value.trim().parse::<f64>() {
            Ok(sum) => return sum,
            Err(_) => println!("Uygun Veri Gir. Mesela Sayı Gir"),
        }
    }
}

fn select_operator(prompt: &str) -> String {
    loop {
        let mut opt = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut opt).expect("Okuma hatası!");
        let opt_trim = opt.trim().to_string();
        if ["+", "-", "*", "/", "q"].contains(&opt_trim.as_str()) {
            return opt_trim;
        } else {
            println!("+ - * / q operatörlerinden birini seç");
        }
    }
}

fn calculate(a: f64, b: f64, op: &str) -> f64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b != 0.0 {
                a / b
            } else {
                println!("İkinci değerin 0 olmamalı çünkü payda 0 olursa bölme işlemi hatalı olur");
                a
            }
        }
        _ => a,
    }
}
