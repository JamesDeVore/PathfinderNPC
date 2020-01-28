mod sql_lite;
use std::fs;

fn main() {

    // sql_lite::create().unwrap();
    // sql_lite::select().unwrap();
    let all_feats = sql_lite::select_all_feats().unwrap();
    println!("{:?}", all_feats["Monkey Moves"])
}
