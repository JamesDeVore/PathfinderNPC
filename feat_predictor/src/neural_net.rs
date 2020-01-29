extern crate rand;
extern crate rusty_machine;


pub mod nn {
    
   
    
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rusty_machine::learning::nnet::{BCECriterion, NeuralNet};
    use rusty_machine::learning::optim::grad_desc::StochasticGD;
    use rusty_machine::learning::toolkit::regularization::Regularization;
    use rusty_machine::learning::SupModel;
    use rusty_machine::linalg::Matrix;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn run_nn(feat_vec: Vec<String>) {
        //I think I might have to train over and over again, so
        //return a bunch of 1x49 matricies instead of what I am now
        //because 29+ layers doesn't really make sense
        let (inputs, targets) = produce_input_and_output(feat_vec.len());

        // Set the layer sizes - from input to output
        let mut layers = vec![1491];
        layers.resize(feat_vec.len()-2,10);
        layers.push(1491);

        // println!("layers: {:?}",layer_num );
        // Choose the BCE criterion with L2 regularization (`lambda=0.1`).
        let criterion = BCECriterion::new(Regularization::L2(0.3));

        // We will just use the default stochastic gradient descent.
        let mut model = NeuralNet::new(&layers, criterion, StochasticGD::default());

        // Train the model!

        for (index,(n_input, n_output)) in inputs.iter().zip(targets.iter()).enumerate() {
            for (i,(n_input1, n_output1)) in n_input.iter().zip(n_output.iter()).enumerate() {
                println!("Training level # {}/ {}, character # {} / {}", i + 1,n_input.len(),index + 1,inputs.len());
                model.train(&n_input1, &n_output1).expect("error training");
            }
        }

        //map the input strings to the id numbers
        let mut test_input : Vec<f64> = vec![];
        for id in feat_vec.iter(){
          let mut tester = vec![0.0; 1491];
          tester[id.parse::<usize>().unwrap()] = 0.5;
          test_input.extend(tester);
        }
        let test_inputs = Matrix::new(feat_vec.len(), 1491, test_input);

        // And predict new output from the test inputs
        let outputs = model.predict(&test_inputs).unwrap();
        let max = outputs.data().iter().cloned().fold(0. / 0., f64::max);
        let index = outputs.data().iter().position(|&x| x == max).unwrap();
        println!("Max Value = {:?}, index = {:?}", max, index);
    }
    fn produce_input_and_output(
        min_feats: usize,
    ) -> (Vec<Vec<Matrix<f64>>>, Vec<Vec<Matrix<f64>>>) {
        //first I need to read all the numerical inputs
        let file = File::open("./src/data/id_output.txt").unwrap();

        let reader = BufReader::new(file);
        let mut input: Vec<Vec<Matrix<f64>>> = Vec::new();
        let mut output: Vec<Vec<Matrix<f64>>> = Vec::new();
        let minimum_feats: Vec<String> = Vec::new();
        let mut total_i_layers = 0;
        let mut total_o_layers = 0;
        for (i, mut line) in reader.lines().enumerate() {
            let random_line = randomize_feat_order(line);
            match i {
                1..=229 => {
                    // if let Ok(l) = random_line {
                    let feats = random_line
                        .split(",")
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect::<Vec<f64>>();

                    let mut character: Vec<Matrix<f64>> = vec![];
                    if feats.len() == min_feats {
                        for f in feats.iter() {
                            let mut this_lvl = vec![0.0; 1491];
                            this_lvl[f.clone() as usize] = 0.5;
                            total_i_layers = total_i_layers + 1;
                            character.push(Matrix::new(1, 1491, this_lvl));
                        }
                        input.push(character);
                    }
                    // }
                }
                _ => {
                    // if let Ok(l) = random_line {
                    let feats = random_line
                        .split(",")
                        .map(|x| x.parse::<f64>().unwrap())
                        .collect::<Vec<f64>>();

                    let mut character: Vec<Matrix<f64>> = vec![];
                    if feats.len() == min_feats {
                        for f in feats.iter() {
                            let mut this_lvl = vec![0.0; 1491];
                            this_lvl[f.clone() as usize] = 0.5;
                            total_i_layers = total_i_layers + 1;
                            character.push(Matrix::new(1, 1491, this_lvl));
                        }
                        output.push(character);
                    }
                    // }
                }
            }
        }
        let mut total_layers = total_i_layers;
        if total_layers > total_o_layers {
            total_layers = total_o_layers;
        }
        input.resize(10,Vec::new());
        output.resize(10,Vec::new());
        println!("output len = {:?}", &output.len());
        return (input, output);
    }
    fn randomize_feat_order(input: std::result::Result<String, std::io::Error>) -> String {
        // // let all_feats = sql_lite::select_all_feats().unwrap();
        // // println!("{:?}",all_feats );
        // let file = File::open("./src/data/input.txt").unwrap();
        // let reader = BufReader::new(file);
        // let mut output_file = File::create("./src/random_input.txt").unwrap();
        // for line in reader.lines().take(3) {
        let mut collected: Vec<String> = input.unwrap().split(",").map(String::from).collect();
        let mut rng = thread_rng();
        collected.shuffle(&mut rng);

        collected.join(",")

        // }
    }
}
