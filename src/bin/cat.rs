use std::{env, fs};

fn read_text_from_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}
fn main() {
    let args = env::args();
    for file in args.skip(1) {
        println!("Filename: {}", file);
        match read_text_from_file(file.as_str()) {
            Ok(text) => {
                for (i, line) in text.split('\n').enumerate() {
                    println!("{}: {}", i, line);
                }
            }
            Err(_) => {
                println!("Can't read text from {}", file);
                continue;
            }
        }
    }
}
