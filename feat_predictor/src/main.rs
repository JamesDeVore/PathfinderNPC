mod neural_net;
mod sql_lite;

// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufReader;
use std::env;
fn main() {
    // sql_lite::create().unwrap();
    // sql_lite::select().unwrap();
    // produce_numerical_inputs();
    let all_feats = sql_lite::select_all_feats().unwrap();
    let args: String = env::args()
        .skip(1)
        .map(|x| String::from(x).trim().to_lowercase().to_string())
        //
        .collect();
    let split: Vec<String> = args
        .split(",")
        //
        .map(String::from)
        .map(|x| {
            match all_feats.contains_key(&x){
                true => return all_feats[&x].to_string(),
                false => panic!("Feat {} is not found", &x)
            }
            
        })
        .collect();

    // println!("{:?}", &split);
    neural_net::run_nn(split);
    // randomize_feat_order()
}

// fn produce_numerical_inputs() {
//     let all_feats = sql_lite::select_all_feats().unwrap();
//     // println!("{:?}",all_feats );
//     let file = File::open("./src/data/input.txt").unwrap();
//     let reader = BufReader::new(file);
//     let mut output_file = File::create("id_output.txt").unwrap();
//     for line in reader.lines() {
//         for word in line.unwrap().split(",") {
//             if !all_feats.contains_key(&word.trim().to_lowercase()) {
//                 // println!("Missing {}", &word.to_lowercase().trim())
//             } else {
//                 output_file
//                     .write_all(all_feats[&word.trim().to_lowercase()].as_bytes())
//                     .expect("Error writing");
//                 output_file
//                     .write_all(",".as_bytes())
//                     .expect("Error writing");
//                 println!(
//                     "Name: {}, ID: {}",
//                     word,
//                     all_feats[&word.trim().to_lowercase()]
//                 );
//             }
//         }
//         output_file
//             .write_all("\n".as_bytes())
//             .expect("newline broke it");
//     }
// }

//         // for word in line.unwrap().split(",") {
//         // }
//     }
// }
