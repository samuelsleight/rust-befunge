use pipeline::Stage;

use dragon_tamer as llvm;

use std::{collections::HashMap, path::PathBuf};

use crate::{
    compiler::ir::{Action, ActionValue, Block, End},
    error::Error,
    inspector::Inspectable,
    interpreter::core::{DynamicValue, StackValue},
};

impl Inspectable for llvm::Module {
    fn inspect(&self) {
        println!("{:?}", self);
    }
}

pub struct Translator {
    source: PathBuf,
}

impl Translator {
    pub fn new(source: PathBuf) -> Self {
        Self { source }
    }
}

pub struct Values {
    values: HashMap<usize, llvm::Value<i32>>,
}

impl Values {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn put(&mut self, idx: usize, value: llvm::Value<i32>) {
        self.values.insert(idx, value);
    }

    fn get_value(&mut self, value: &StackValue, builder: &llvm::Builder) -> llvm::Value<i32> {
        match value {
            StackValue::Const(i) => llvm::Value::constant(*i),
            StackValue::Dynamic(ref value) => self.get(value, builder),
        }
    }

    fn get(&mut self, value: &DynamicValue, builder: &llvm::Builder) -> llvm::Value<i32> {
        match value {
            DynamicValue::Tagged(idx) => {
                self.values.get(&idx).expect("Invalid tagged value").clone()
            }

            DynamicValue::Add(ref lhs, ref rhs) => {
                let lhs = self.get_value(&*lhs, builder);
                let rhs = self.get_value(&*rhs, builder);
                builder.build_add(&lhs, &rhs)
            }

            DynamicValue::Sub(ref lhs, ref rhs) => {
                let lhs = self.get_value(&*lhs, builder);
                let rhs = self.get_value(&*rhs, builder);
                builder.build_sub(&lhs, &rhs)
            }

            _ => unimplemented!("Unimplemented dynamic value type: {:?}", value),
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

        let builder = llvm::Builder::new();

        let blocks = input
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                if idx == 0 {
                    function.add_block("entry")
                } else {
                    function.add_block(idx.to_string())
                }
            })
            .collect::<Vec<llvm::Block>>();

        for (idx, block) in input.iter().enumerate() {
            builder.set_block(&blocks[idx]);

            for action in block.actions() {
                match action {
                    Action::Input(idx) => values.put(*idx, builder.build_call(&getchar, ())),
                    Action::OutputChar(ActionValue::Const(i)) => {
                        builder.build_call(&putchar, (llvm::Value::constant(*i),))
                    }
                    Action::OutputChar(ActionValue::Dynamic(value)) => {
                        builder.build_call(&putchar, (values.get(value, &builder),))
                    }
                    Action::OutputString(s) => {
                        builder.build_call(&puts, (module.add_string(s.clone()),))
                    }
                    Action::Tag(idx, value) => {
                        let value = values.get(value, &builder);
                        values.put(*idx, value)
                    }
                }
            }

            match block.end() {
                End::End => builder.build_ret(),
                End::If(ActionValue::Const(i), t, f) => builder.build_conditional_jump(
                    &llvm::Value::constant(*i),
                    &blocks[*t],
                    &blocks[*f],
                ),
                End::If(ActionValue::Dynamic(value), t, f) => builder.build_conditional_jump(
                    &values.get(value, &builder),
                    &blocks[*t],
                    &blocks[*f],
                ),
            }
        }

        Ok(module)
    }
}
