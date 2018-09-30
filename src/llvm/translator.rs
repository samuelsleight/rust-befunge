use pipeline::Stage;

use std::path::PathBuf;

use crate::{
    error::Error,
    compiler::ir::Block,
    llvm::llvm
};

pub struct Translator {
    source: PathBuf
}

impl Translator {
    pub fn new(source: PathBuf) -> Translator {
        Translator {
            source
        }
    }
}

impl Stage<Error> for Translator {
    type Input = Vec<Block>;
    type Output = llvm::Module;

    fn run(self, input: Self::Input) -> Result<Self::Output, Error> {
        Ok(llvm::Module::new("test", self.source))
    }
}