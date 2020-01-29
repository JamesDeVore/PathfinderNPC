extern crate rusty_machine;

pub mod nn {
    use rusty_machine::learning::nnet::{BCECriterion, NeuralNet};
    use rusty_machine::learning::optim::grad_desc::StochasticGD;
    use rusty_machine::learning::toolkit::regularization::Regularization;
    use rusty_machine::learning::SupModel;
    use rusty_machine::linalg::Matrix;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn run_nn(num_feats: usize) {
      //I think I might have to train over and over again, so 
      //return a bunch of 1x49 matricies instead of what I am now
      //because 29+ layers doesn't really make sense
        let (inputs, targets) = produce_input_and_output(5);
        
        // Set the layer sizes - from input to output
        let layers = &vec![1491;3];
        // println!("layers: {:?}",layer_num );
        // Choose the BCE criterion with L2 regularization (`lambda=0.1`).
        let criterion = BCECriterion::new(Regularization::L2(0.1));
        
        // We will just use the default stochastic gradient descent.
        let mut model = NeuralNet::new(layers, criterion, StochasticGD::default());

        // Train the model!

        for (n_input, n_output) in inputs.iter().zip(targets.iter()){
          println!("Training");
          model.train(&n_input, &n_output).expect("error training");
        }
        
        let mut tester = vec![0.0;1491];
        tester[10] = 1.0;
        tester[15] = 1.0;
        tester[43] = 1.0;
        let test_inputs = Matrix::new(
            1,
            1491,
            tester,
        );
        
        // And predict new output from the test inputs
        let outputs = model.predict(&test_inputs).unwrap();
        let max = outputs.data().iter().cloned().fold(0./0., f64::max);
        let index = outputs.data().iter().position(|&x| x == max);
        println!("Max Value = {:?}, index = {:?}", max, index );
    }
    fn produce_input_and_output(min_feats: usize) -> (Vec<Matrix<f64>>, Vec<Matrix<f64>>) {
        //first I need to read all the numerical inputs
        let file = File::open("./src/data/id_output.txt").unwrap();

        let reader = BufReader::new(file);
        let mut input: Vec<Matrix<f64>> = Vec::new();
        let mut output: Vec<Matrix<f64>> = Vec::new();
        let minimum_feats: Vec<String> = Vec::new();
        let mut total_i_layers = 0;
        let mut total_o_layers = 0;
        for (i, line) in reader.lines().enumerate() {
            match i {
                1..=229 => {
                    if let Ok(l) = line {
                        let feats = l
                            .split(",")
                            .map(|x| x.parse::<f64>().unwrap())
                            .collect::<Vec<f64>>();
                        let mut this_char = vec![0.0; 1491];
                        
                        if feats.len() == min_feats {
                          for f in feats.iter() {
                            this_char[f.clone() as usize] = 1.0;
                        }
                          total_i_layers = total_i_layers + 1;
                            input.push(Matrix::new(1,1491,this_char));
                        }
                    }
                }
                _ => {
                    if let Ok(l) = line {
                        let feats = l
                            .split(",")
                            .map(|x| x.parse::<f64>().unwrap())
                            .collect::<Vec<f64>>();
                        let mut this_char = vec![0.0; 1491];
                        
                        if feats.len() == min_feats {
                          for f in feats.iter() {
                            this_char[f.clone() as usize] = 1.0;
                        }
                          total_o_layers = total_o_layers + 1;
                            output.push(Matrix::new(1,1491,this_char));
                        }
                    }
                }
            }
        }
        let mut total_layers = total_i_layers;
        if total_layers > total_o_layers {
          total_layers = total_o_layers;
        }
        // input.resize(total_layers,vec![]);
        // output.resize(total_layers,vec![]);
        // let flatten_input: Vec<f64> = input
        //                 .iter()
        //                 .flat_map(|array| array.iter())
        //                 .cloned()
        //                 .collect();
        // let flatten_output: Vec<f64> = input
        //                 .iter()
        //                 .flat_map(|array| array.iter())
        //                 .cloned()
        //                 .collect();
        // println!("{:?}",&flatten_input );

        
        // let inputs = Matrix::new(
        //     total_layers, //layer num
        //     1491,      //num per layer //should be the number of feats total
        //     flatten_input,
        // );
        // let targets = Matrix::new(
        //     total_layers,
        //     1491,
        //     flatten_output,
        // );
        
        
        
        println!("output len = {:?}",&output.len() );
        return (input, output);
    }
}
