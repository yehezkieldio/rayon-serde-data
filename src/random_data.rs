use std::time::Instant;

use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Data;

fn generate() -> Data {
    let mut rng = thread_rng();
    let id = rng.gen::<u64>();
    let name = format!("User {}", rng.gen_range(0..1000));
    let age = rng.gen_range(0..100);

    Data { id, name, age }
}

#[allow(dead_code)]
pub fn init() {
    let start_time = Instant::now();

    let data_vec: Vec<Data> = (0..100_000).into_par_iter().map(|_| generate()).collect();

    let json_string = serde_json::to_string(&data_vec).unwrap();
    println!("{}", json_string);

    let elapsed_time = start_time.elapsed();
    println!("Finished generating!");
    println!("Elapsed time: {:?}", elapsed_time);

    std::fs::write("data.json", json_string).unwrap();
}
