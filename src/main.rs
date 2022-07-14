extern crate csv;
use csv::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use qrcode_generator::QrCodeEcc;

fn main() -> Result<(), Error> {
    let filename = "sample.csv";
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
        let directory = String::from("tmp/qr");
        let name = &record[0];
        let output = directory + name;
        let start = Instant::now();
        qrcode_generator::to_png_to_file(_qr_url, QrCodeEcc::Medium, 300, output).unwrap();
        let end = start.elapsed();
        println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
    }
    Ok(())
}
