use std::io::{self, BufRead};

use crate::interpreter::{
    core::{DebugInspectable, DebuggerCallback, InterpreterCore},
    Interpreter,
};

#[derive(Clone, Copy)]
enum DebugMode {
    Step,
    Continue,
}

pub struct Debugger {
    mode: DebugMode,
    trace: bool,
}

impl Debugger {
    pub fn stage(trace: bool, cont: bool) -> InterpreterCore<Interpreter, Self> {
        let debugger = Self {
            mode: if cont {
                DebugMode::Continue
            } else {
                DebugMode::Step
            },

            trace,
        };

        InterpreterCore::new(Interpreter, debugger)
    }

    pub fn trace<I: DebugInspectable>(&self, inspectable: &I) {
        println!();
        println!("Stack: {:?}", inspectable.inspect_stack());
        println!(
            "Next: {:?}: {}",
            inspectable.inspect_pos(),
            inspectable.inspect_next()
        );
    }
}

impl<I: DebugInspectable> DebuggerCallback<I> for Debugger {
    fn debug_step(&mut self, inspectable: &I) {
        match self.mode {
            DebugMode::Step => {
                self.trace(inspectable);

                let input = io::stdin().lock().lines().next().unwrap().unwrap();
                match &input as &str {
                    "c" => self.mode = DebugMode::Continue,
                    "t" => self.trace = !self.trace,
                    _ => (),
                }
            }

            DebugMode::Continue => {
                if self.trace {
                    self.trace(inspectable);
                }
            }
        }
    }
}
