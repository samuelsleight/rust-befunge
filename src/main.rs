use std::path::PathBuf;

use structopt::StructOpt;

use pipeline::{Err, Pipeline, RunPipeline};

use crate::{
    inspector::Inspector,
    reader::FileReader,
    interpreter::Interpreter,
};

mod error;
mod inspector;
mod reader;
mod interpreter;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(required = true, parse(from_os_str))]
    filename: PathBuf,

    #[structopt(long = "debug-file")]
    debug_file: bool,
}

fn main() {
    let options = Options::from_args();

    let result = pipeline
        ::pipeline(FileReader::new(), |_| ())
        .and_then(Inspector::new(options.debug_file), |_| ())
        .and_then(Interpreter::new(), |_| ())
        .run(options.filename);

    if let Err(Err::Err(e)) = result {
        println!("{}", e)
    }
}