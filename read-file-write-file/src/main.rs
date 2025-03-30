use std::{fs::{self, File, OpenOptions}, io::Write};

fn main() {
    fs::create_dir("files").expect("files adlı klasör oluşturuldu");
    File::create("files/deneme.txt").expect("Dosya Başarıyla Oluşturuldu");
    fs::write("files/deneme.txt", "Merhaba Kadir!").expect("Metin Başarıyla Eklendi");
    let conent = fs::read_to_string("files/deneme.txt").unwrap();
    let file_path = "files/deneme2.txt";
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    //.truncate(true) // eğer dosya varsa içeriği siler baştan yazar
    .open(file_path).unwrap();
    file.write_all("Deneme 2 yazısı".as_bytes()).expect("");
    let content2 = fs::read_to_string("files/deneme2.txt").unwrap();
    println!("{}", conent);
    println!("{}",content2);
}
