#![feature(never_type)]

extern crate llvm_wrapper;
extern crate pipeline;

use std::path::PathBuf;

use structopt::StructOpt;

use pipeline::{Err, Pipeline, RunPipeline};

use crate::{
    inspector::Inspector,
    reader::FileReader,
    interpreter::Interpreter,
    compiler::Compiler,
    optimizer::{Optimizer, OptimizationLevel},
    translator::Translator,
};

mod error;
mod inspector;
mod reader;
mod interpreter;
mod compiler;
mod optimizer;
mod translator;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(required = true, parse(from_os_str))]
    filename: PathBuf,

    #[structopt(long = "interpret", short = "i")]
    interpret: bool,

    #[structopt(long = "debug-file")]
    debug_file: bool,

    #[structopt(long = "debug-unoptimized-ir")]
    debug_unoptimized_ir: bool,

    #[structopt(long = "debug-ir")]
    debug_ir: bool,

    #[structopt(long = "debug-llvm")]
    debug_llvm: bool,
}

fn main() {
    let options = Options::from_args();

    let pipe = pipeline
        ::pipeline(FileReader::new(), |_| ())
        .and_then(Inspector::new(options.debug_file), |_| ());

    let result = if options.interpret {
        pipe
            .and_then(Interpreter::stage(), |_| ())
            .run(options.filename)
    }
    else {
        pipe
            .and_then(Compiler::new(), |_| ())
            .and_then(Inspector::new(options.debug_unoptimized_ir), |_| ())
            .and_then(Optimizer::new(OptimizationLevel::All), |_| ())
            .and_then(Inspector::new(options.debug_ir), |_| ())
            .and_then(Translator::new(options.filename.clone()), |_| ())
            .and_then(Inspector::new(options.debug_llvm), |_| ())
            .run(options.filename)
            .map(|_| ())
    };

    if let Err(Err::Err(e)) = result {
        println!("{}", e)
    }
}