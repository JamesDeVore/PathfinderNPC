extern crate regex;

use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;

pub fn return_feats(files: Vec<String>) -> Vec<Vec<String>> {
  //compile all the regs ahead of time
    let newline_regexp = Regex::new(r"\n").unwrap();
    let skills_regexp = Regex::new(r"([sS]kills)").unwrap();
    let feats_regexp = Regex::new(r"([F]eats.+)").unwrap();
    let b_regexp = Regex::new(r"B$").unwrap();
    let parentheses = Regex::new(r"\(.+\)").unwrap();
    let mut final_feats : Vec<Vec<String>> = Vec::new();
    for file_name in files.iter() {
        let contents = fs::read_to_string(file_name).expect("reading text file went wrong");
        let feats = return_only_feats_from_string(
            &contents,
            &newline_regexp,
            &feats_regexp,
            &skills_regexp,
            &b_regexp,
            &parentheses,
        );
        match feats {
            Some(val) => final_feats.push(val),
            None => println!("Error in {}", file_name),
        }
        //println!("{:?}",feats.unwrap() );
    }
    return final_feats;

    
}

fn return_only_feats_from_string(
    input: &String,
    new_line_reg: &Regex,
    feat_reg: &Regex,
    skill_reg: &Regex,
    b_reg: &Regex,
    parentheses: &Regex
) -> Option<Vec<String>> {
    //first remove all newlines to make it easier to grab the feats
    let without_newlines = new_line_reg.replace_all(&input, " ");
    let skill_to_newline = skill_reg.replace_all(&without_newlines, "\n");
    let feats_with_skills = match feat_reg.captures(&skill_to_newline) {
        Some(val) => match val.get(0) {
            Some(c) => c.as_str(),
            None => "",
        },
        None => "",
    };
    //now I need to remove all the Bs at the end of the string
    let mut just_feats = feats_with_skills.replace("Feats", "");
    just_feats = parentheses.replace_all(&just_feats,"").to_string();

    let mut feat_vec: Vec<String> = Vec::new();
    for word in just_feats.split(",") {
        let no_b = b_reg.replace_all(word, "");
        feat_vec.push(no_b.trim().to_string());
    }
    match feat_vec.len() {
        20..=999 => None,
        _ => Some(feat_vec),
    }
}
