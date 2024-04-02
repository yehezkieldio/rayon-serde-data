mod random_data;

use serde::Serialize;

#[derive(Serialize)]
struct Data {
    id: u64,
    name: String,
    age: u8,
}

fn main() {
    random_data::init();
}
