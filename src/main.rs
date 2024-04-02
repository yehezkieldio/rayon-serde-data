mod process_data;
mod random_data;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data {
    id: u64,
    name: String,
    age: u8,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [random|process]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "random" => random_data::init(),
        "process" => process_data::init().unwrap(),
        _ => {
            eprintln!("Invalid option: {}", args[1]);
            std::process::exit(1);
        }
    }
}
