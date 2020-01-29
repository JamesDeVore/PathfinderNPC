mod neural_net;
mod sql_lite;
use neural_net::nn;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // sql_lite::create().unwrap();
    // sql_lite::select().unwrap();
    // produce_numerical_inputs()
    nn::run_nn(3);
}

fn produce_numerical_inputs() {
    let all_feats = sql_lite::select_all_feats().unwrap();
    // println!("{:?}",all_feats );
    let file = File::open("./src/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut output_file = File::create("id_output.txt").unwrap();
    for line in reader.lines() {
        for word in line.unwrap().split(",") {
            if !all_feats.contains_key(&word.trim().to_lowercase()) {
                // println!("Missing {}", &word.to_lowercase().trim())
            } else {
                output_file
                    .write_all(all_feats[&word.trim().to_lowercase()].as_bytes())
                    .expect("Error writing");
                output_file
                    .write_all(",".as_bytes())
                    .expect("Error writing");
                println!(
                    "Name: {}, ID: {}",
                    word,
                    all_feats[&word.trim().to_lowercase()]
                );
            }
        }
        output_file
            .write_all("\n".as_bytes())
            .expect("newline broke it");
    }
}
