mod neural_net;
mod feat_map;


use std::env;
fn main() {
    // produce_numerical_inputs();
    let all_feats = feat_map::select_all_feats();
    let args: String = env::args()
        .skip(1)
        .map(|x| String::from(x).trim().to_lowercase().to_string())
        .collect();
    let split: Vec<String> = args
        .split(",")
        .map(String::from)
        .map(|x| {
            match all_feats.contains_key(&x){
                true => return all_feats[&x].to_string(),
                false => panic!("Feat {} is not found", &x)
            }
            
        })
        .collect();

    println!("{:?}", &split);
    neural_net::run_nn(split);
    // randomize_feat_order()
}

