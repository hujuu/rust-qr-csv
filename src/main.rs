extern crate csv;
use csv::Error;
use std::fs::File;
use std::io::prelude::*;
use qrcode_generator::QrCodeEcc;

fn main() -> Result<(), Error> {
    qrcode_generator::to_png_to_file("Hello world!", QrCodeEcc::Medium, 1024, "tmp/file_output.png").unwrap();
    let filename = "sample.csv";
    // --snip--
    println!("In file {}", filename);
    // ファイルが見つかりませんでした
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");
    // テキストは\n{}です
    println!("With text:\n{}", contents);
    
    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    for record in reader.records() {
        let record = record?;
        let _qr_url = record[1].to_string();
        println!(
            &record[1],
        );
    }
    
    Ok(())
}

