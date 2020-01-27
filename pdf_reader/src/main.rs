mod pdfhandling;
mod text_parser;

use pdfhandling::pdf;
use text_parser::return_feats;
fn main() {
    //don't need to read the files anymore
    // let files =
    //     pdf::find_all_files(String::from("./CharacterSheets")).expect("error reading names");
    // for file_name in files.iter() {
    //     println!("Reading file: {}", &file_name.trim());
    //     // pdf::read_pdf_file(String::from(file_name));
    // }

    //next step is to begin pulling out the list of feats per character

    let all_feats = return_feats(pdf::find_all_files(String::from("./text")).unwrap());
    println!("{:?}",all_feats);
}
