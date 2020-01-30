extern crate rand;
extern crate rusty_machine;

use crate::feat_map::select_all_feats_id;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusty_machine::learning::nnet::{BCECriterion, NeuralNet};
use rusty_machine::learning::optim::grad_desc::StochasticGD;
use rusty_machine::learning::toolkit::regularization::Regularization;
use rusty_machine::learning::SupModel;
use rusty_machine::linalg::Matrix;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run_nn(feat_vec: Vec<String>) {
    //I think I might have to train over and over again, so
    //return a bunch of 1x49 matricies instead of what I am now
    //because 29+ layers doesn't really make sense
    let feat_map = select_all_feats_id();
        // println!("{:?}", &index);
        let mut results : HashMap<String,String> = HashMap::new();
        let (inputs, targets) = produce_input_and_output(feat_vec.len());
        
    for _i in 1..100 {
        

        // Set the layer sizes - from input to output
        let mut layers = vec![1491];
        layers.resize(feat_vec.len() - 2, 15);
        layers.push(1491);

        // println!("layers: {:?}",layer_num );
        // Choose the BCE criterion with L2 regularization (`lambda=0.1`).
        let criterion = BCECriterion::new(Regularization::L2(0.1));

        // We will just use the default stochastic gradient descent.
        let mut model = NeuralNet::new(&layers, criterion, StochasticGD::default());

        // Train the model!

        for (_index, (n_input, n_output)) in inputs.iter().zip(targets.iter()).enumerate() {
            // for (i,(n_input1, n_output1)) in n_input.iter().zip(n_output.iter()).enumerate() {
            // println!("Training level # {}/ {}, character # {} / {}", i + 1,n_input.len(),index + 1,inputs.len());
            model.train(&n_input, &n_output).expect("error training");
            // }
        }

        //map the input strings to the id numbers
        let mut tester = vec![0.0; 1491];
        for id in feat_vec.iter() {
            tester[id.parse::<usize>().unwrap()] = 1.0;
        }
        let test_inputs = Matrix::new(1, 1491, tester);

        // And predict new output from the test inputs
        let outputs = model.predict(&test_inputs).unwrap();
        let max = outputs.data().iter().cloned().fold(0. / 0., f64::max);
        let index = outputs.data().iter().position(|&x| x == max).unwrap();
        
        match feat_map.contains_key(&index.to_string()) {
            true => {
                let name = feat_map.get(&index.to_string()).unwrap();
                println!("Max Value = {:?}, index = {:?}, name = {:?}",max,index,&name);
                match results.contains_key(&name.to_string()){
                    true => {
                        let current_max = results[&name.to_string()].parse::<f64>().unwrap();
                        let new_avg = current_max + max;
                        results.insert(String::from(name),new_avg.to_string());
                    }
                    false => {
                        results.insert(String::from(name),max.to_string());
                    }
                }
            }
            false => println!("Misread"),
        }
        
    }
    
    let top_char = results.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("Top result: {:?}",top_char);

}
fn produce_input_and_output(min_feats: usize) -> (Vec<Matrix<f64>>, Vec<Matrix<f64>>) {
    //first I need to read all the numerical inputs
    let file = File::open("./src/data/id_output.txt").unwrap();

    let reader = BufReader::new(file);
    let mut input: Vec<Matrix<f64>> = Vec::new();
    let mut output: Vec<Matrix<f64>> = Vec::new();
    let _minimum_feats: Vec<String> = Vec::new();
    let mut total_i_layers = 0;
    let mut total_o_layers = 0;
    for (i, line) in reader.lines().enumerate() {
        let random_line = randomize_feat_order(line);
        match i {
            1..=229 => {
                // if let Ok(l) = random_line {
                let feats = random_line
                    .split(",")
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>();

                // let mut character: Vec<Matrix<f64>> = vec![];
                if feats.len() == min_feats {
                    let mut this_char = vec![0.0; 1491];
                    for f in feats.iter() {
                        this_char[f.clone() as usize] = 1.0;
                        total_i_layers = total_i_layers + 1;
                    }
                    // character.push(Matrix::new(1, 1491, this_char));
                    input.push(Matrix::new(1, 1491, this_char));
                }
                // }
            }
            _ => {
                // if let Ok(l) = random_line {
                let feats = random_line
                    .split(",")
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>();

                // let mut character: Vec<Matrix<f64>> = vec![];
                if feats.len() == min_feats {
                    let mut this_char = vec![0.0; 1491];
                    for f in feats.iter() {
                        this_char[f.clone() as usize] = 1.0;
                        total_o_layers = total_o_layers + 1;
                    }
                    // character.push(Matrix::new(1, 1491, this_char));
                    output.push(Matrix::new(1, 1491, this_char));
                }
                // }
            }
        }
    }

    return (input, output);
}
fn randomize_feat_order(input: std::result::Result<String, std::io::Error>) -> String {

    let mut collected: Vec<String> = input.unwrap().split(",").map(String::from).collect();
    let mut rng = thread_rng();
    collected.shuffle(&mut rng);

    collected.join(",")


}
