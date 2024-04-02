use std::{
    io::{self, BufRead},
    time::Instant,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::Data;

fn process(chunk: &str) -> Result<(), serde_json::Error> {
    let data: Vec<Data> = serde_json::from_str(chunk)?;

    for item in data {
        println!("ID: {}, Name: {}, Age: {}", item.id, item.name, item.age);
    }

    Ok(())
}

#[allow(dead_code)]
pub fn init() -> Result<(), io::Error> {
    let start_time = Instant::now();

    let file = std::fs::File::open("data.json")?;
    let reader = io::BufReader::new(file);

    let chunks: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let _ = chunks.par_iter().try_for_each(|chunk| process(chunk));

    let elapsed_time = start_time.elapsed();
    println!("Finished processing!");
    println!("Elapsed time: {:?}", elapsed_time);

    Ok(())
}
