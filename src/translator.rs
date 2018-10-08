use pipeline::Stage;

use llvm_wrapper as llvm;

use std::path::PathBuf;

use crate::{
    error::Error,
    inspector::Inspectable,
    compiler::ir::{
        Block,
        Action,
        End
    },
};

impl Inspectable for llvm::Module {
    fn inspect(&self) {
        println!("{:?}", self);
    }
}

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
        let module = llvm::Module::new("test", self.source);
        let function = module.add_function::<_, fn() -> i32>("main");
        let block = function.add_block("entry");
        let putchar = module.add_function::<_, fn(i32)>("putchar");
        let puts = module.add_function::<_, fn(String)>("puts");
        let builder = llvm::Builder::new();
        builder.set_block(&block);

        for action in input[0].actions() {
            match action {
                Action::OutputChar(c) => builder.build_call(&putchar, (llvm::Value::constant(*c as u8 as i32),)),
                Action::OutputString(s) => builder.build_call(&puts, (module.add_string(s.clone()),)),
            }
        }

        match input[0].end() {
            End::End => builder.build_ret()
        }

        Ok(module)
    }
}