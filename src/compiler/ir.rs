use crate::{
    interpreter::core::DynamicValue,
    inspector::Inspectable
};

#[derive(Debug, Clone)]
pub struct Block {
    actions: Vec<Action>,
    end: End
}

#[derive(Debug, Clone)]
pub enum ActionValue {
    Const(i32),
    Dynamic(DynamicValue),
}

#[derive(Debug, Clone)]
pub enum Action {
    OutputChar(ActionValue),
    OutputString(String),
    Input(usize)
}

#[derive(Debug, Clone)]
pub enum End {
    End,
    If(ActionValue, usize, usize)
}

impl Block {
    pub fn new(actions: Vec<Action>, end: End) -> Self {
        Self {
            actions,
            end
        }
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    pub fn end(&self) -> &End {
        &self.end
    }
}

impl Inspectable for Vec<Block> {
    fn inspect(&self) {
        for (idx, block) in self.iter().enumerate() {
            println!("Block {} {{", idx);

            for action in &block.actions {
                println!("\t{:?}", action);
            }

            println!("\t{:?}", block.end);
            println!("}}");
        }
    }
}
