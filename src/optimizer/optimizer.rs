use pipeline::{
    self,
    Stage,
    RunPipeline
};

use crate::{
    error::Error,
    compiler::ir::Block,
    optimizer::pass::{Pass, StringPrintPass},
};

#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    None,
    All
}

impl From<&str> for OptimizationLevel {
    fn from(src: &str) -> Self {
        match src {
            "0" => OptimizationLevel::None,
            "" => OptimizationLevel::All,
            _ => panic!("Invalid optimization flag provided"),
        }
    }
}

pub struct Optimizer {
    level: OptimizationLevel
}

impl Optimizer {
    pub fn new(level: OptimizationLevel) -> Self {
        Self {
            level
        }
    }
}

impl Stage<Error> for Optimizer {
    type Input = Vec<Block>;
    type Output = Vec<Block>;

    fn run(self, input: Self::Input) -> Result<Self::Output, Error> {
        match self.level {
            OptimizationLevel::None => Ok(input),
            OptimizationLevel::All => Ok(pipeline
                ::pipeline(StringPrintPass::new(), |_| ())
                .run(input)
                .unwrap())
        }
    }
}
