use pipeline::Stage;

use llvm_wrapper as llvm;

use std::{
    collections::HashMap,
    path::PathBuf
};

use crate::{
    error::Error,
    inspector::Inspectable,
    interpreter::core::{
        StackValue,
        DynamicValue,
    },
    compiler::ir::{
        Block,
        Action,
        ActionValue,
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

pub struct Values {
    values: HashMap<usize, llvm::Value<i32>>
}

impl Values {
    fn new() -> Values {
        Values {
            values: HashMap::new()
        }
    }

    fn put(&mut self, idx: usize, value: llvm::Value<i32>) {
        self.values.insert(idx, value);
    }

    fn get_value(&mut self, value: &StackValue, builder: &llvm::Builder) -> llvm::Value<i32> {
        match value {
            &StackValue::Const(i) => llvm::Value::constant(i),
            &StackValue::Dynamic(ref value) => self.get(value, builder)
        }
    }

    fn get(&mut self, value: &DynamicValue, builder: &llvm::Builder) -> llvm::Value<i32> {
        match value {
            &DynamicValue::Tagged(idx) => self.values.get(&idx).expect("Invalid tagged value").clone(),

            &DynamicValue::Add(ref lhs, ref rhs) => {
                let lhs = self.get_value(&*lhs, builder);
                let rhs = self.get_value(&*rhs, builder);
                builder.build_add(lhs, rhs)
            },

            _ => unimplemented!("Unimplemented dynamic value type: {:?}", value)
        }
    }
}

impl Stage<Error> for Translator {
    type Input = Vec<Block>;
    type Output = llvm::Module;

    fn run(self, input: Self::Input) -> Result<Self::Output, Error> {
        let module = llvm::Module::new("test", self.source);

        let mut values = Values::new();

        let getchar = module.add_function::<_, fn() -> i32>("getchar");
        let putchar = module.add_function::<_, fn(i32)>("putchar");
        let puts = module.add_function::<_, fn(String)>("puts");

        let function = module.add_function::<_, fn() -> i32>("main");
        let block = function.add_block("entry");

        let builder = llvm::Builder::new();
        builder.set_block(&block);

        for action in input[0].actions() {
            match action {
                Action::Input(idx) => values.put(*idx, builder.build_call(&getchar, ())),
                Action::OutputChar(ActionValue::Const(i)) => builder.build_call(&putchar, (llvm::Value::constant(*i),)),
                Action::OutputChar(ActionValue::Dynamic(value)) => builder.build_call(&putchar, (values.get(value, &builder),)),
                Action::OutputString(s) => builder.build_call(&puts, (module.add_string(s.clone()),)),

                _ => unimplemented!("Translator hit unimplemented action: {:?}", action)
            }
        }

        match input[0].end() {
            End::End => builder.build_ret()
        }

        Ok(module)
    }
}