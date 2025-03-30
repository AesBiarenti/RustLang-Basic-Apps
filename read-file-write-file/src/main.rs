use std::fs::{self, File};

fn main() {
    fs::create_dir("files").expect("files adlı klasör oluşturuldu");
    File::create("files/deneme.txt").expect("Dosya Başarıyla Oluşturuldu");
    fs::write("files/deneme.txt", "Merhaba Kadir!").expect("Metin Başarıyla Eklendi");
    let conent = fs::read_to_string("files/deneme.txt").unwrap();
    println!("{}", conent);
}
