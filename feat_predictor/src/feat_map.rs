use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::error::Error;


#[derive(Debug)]
struct Feat {
    id: String,
    feat_name: String,
}

pub fn select_all_feats() -> HashMap<String, String> {
      let mut all_feats = HashMap::new();

    let file = File::open("./src/data/feats.txt").unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines(){
      // println!("{:?}",&line );
      let split = line.unwrap().split(",").map(String::from).collect::<Vec<String>>();
      
      all_feats.insert(split[1].to_string().trim().to_lowercase(),split[0].to_string());
    }
    // println!("{:?}",&all_feats );
    all_feats
}

pub fn select_all_feats_id() -> HashMap<String, String> {    
  let mut all_feats = HashMap::new();

    let file = File::open("./src/data/feats.txt").unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines(){
      
      let split = line.unwrap().split(",").map(String::from).collect::<Vec<String>>();
      // println!("{:?}",&split );
      all_feats.insert(split[0].to_string(),split[1].to_string().trim().to_lowercase());
    }


    all_feats
}
    
