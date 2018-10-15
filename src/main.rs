#![feature(never_type)]

#![cfg_attr(feature = "cargo-clippy", warn(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(module_inception, stutter, cast_sign_loss))]

extern crate llvm_wrapper;
extern crate pipeline;

use std::path::PathBuf;

use structopt::StructOpt;

use pipeline::{Err, Pipeline, RunPipeline};

use crate::{
    inspector::Inspector,
    reader::FileReader,
    interpreter::{Interpreter, Debugger},
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
struct SharedOptions {
    #[structopt(flatten)]
    debug: Debug,

    #[structopt(required = true, parse(from_os_str))]
    filename: PathBuf,
}

#[derive(Debug, StructOpt)]
struct Debug {
    #[structopt(long = "debug-file")]
    file: bool,

    #[structopt(long = "debug-unoptimized-ir")]
    unoptimized_ir: bool,

    #[structopt(long = "debug-ir")]
    ir: bool,

    #[structopt(long = "debug-llvm")]
    llvm: bool
}

#[derive(Debug, StructOpt)]
enum Options {
    #[structopt(name = "c")]
    Compiler {
        #[structopt(long = "optimize", short = "O", default_value="", parse(from_str))]
        optimize: OptimizationLevel,

        #[structopt(flatten)]
        options: SharedOptions
    },

    #[structopt(name = "i")]
    Interpreter {
        #[structopt(flatten)]
        options: SharedOptions
    },

    #[structopt(name = "d")]
    Debugger {
        #[structopt(long = "trace", short = "t")]
        trace: bool,

        #[structopt(long = "continue", short = "c")]
        cont: bool,

        #[structopt(flatten)]
        options: SharedOptions
    },
}

impl Options {
    fn options(&self) -> &SharedOptions {
        match self {
            Options::Compiler { ref options, .. } |
            Options::Interpreter { ref options } |
            Options::Debugger { ref options, .. } => options,
        }
    }
}

fn main() {
    let command = Options::from_args();
    let options = command.options();

    let pipe = pipeline
        ::pipeline(FileReader::new(), |_| ())
        .and_then(Inspector::new(options.debug.file), |_| ());

    let result = match command {
        Options::Compiler { optimize, .. } => pipe
            .and_then(Compiler::new(), |_| ())
            .and_then(Inspector::new(options.debug.unoptimized_ir), |_| ())
            .and_then(Optimizer::new(optimize), |_| ())
            .and_then(Inspector::new(options.debug.ir), |_| ())
            .and_then(Translator::new(options.filename.clone()), |_| ())
            .and_then(Inspector::new(options.debug.llvm), |_| ())
            .run(options.filename.clone())
            .map(|_| ()),

        Options::Interpreter { .. } => pipe
            .and_then(Interpreter::stage(), |_| ())
            .run(options.filename.clone()),

        Options::Debugger { trace, cont, .. } => pipe
            .and_then(Debugger::stage(trace, cont), |_| ())
            .run(options.filename.clone()),
    };

    if let Err(Err::Err(e)) = result {
        println!("{}", e)
    }
}
