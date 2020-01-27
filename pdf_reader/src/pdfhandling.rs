pub mod pdf {
    extern crate lopdf;
    extern crate pdf_extract;

    use std::fs;
    use std::io;

    use lopdf::Document;
    use pdf_extract::*;
    use std::env;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path;
    use std::path::PathBuf;
    ///Reads all files from the designated pdf repository;
    pub fn find_all_files(path: String) -> Result<Vec<String>, std::io::Error> {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| String::from(e.path().to_str().unwrap())))
            .collect::<Result<Vec<_>, io::Error>>()?;

        // The order in which `read_dir` returns entries is not guaranteed. If reproducible
        // ordering is required the entries should be explicitly sorted.

        entries.sort();

        // The entries have now been sorted by their path.
        Ok(entries)
    }

    pub fn read_pdf_file(file: String) {
        //     let file = env::args().nth(1).unwrap();
        let output_kind = env::args().nth(2).unwrap_or_else(|| "txt".to_owned());
        println!("{}", file);
        let path = path::Path::new(&file);
        let filename = path.file_name().expect("expected a filename");
        let mut output_file = PathBuf::new();
        output_file.push("./text");
        output_file.push(filename);
        output_file.set_extension(&output_kind);
        let mut output_file =
            BufWriter::new(File::create(output_file).expect("could not create output"));
        let doc = Document::load(path).unwrap();

        print_metadata(&doc);

        let mut output: Box<dyn OutputDev> = match output_kind.as_ref() {
            "txt" => Box::new(PlainTextOutput::new(
                &mut output_file as &mut dyn std::io::Write,
            )),
            "html" => Box::new(HTMLOutput::new(&mut output_file)),
            "svg" => Box::new(SVGOutput::new(&mut output_file)),
            _ => panic!(),
        };

        output_doc(&doc, output.as_mut());
    }
}
