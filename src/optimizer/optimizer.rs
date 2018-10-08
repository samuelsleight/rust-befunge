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

pub enum OptimizationLevel {
    None,
    All
}

pub struct Optimizer {
    level: OptimizationLevel
}

impl Optimizer {
    pub fn new(level: OptimizationLevel) -> Optimizer {
        Optimizer {
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
