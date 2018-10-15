#![cfg_attr(feature = "cargo-clippy", warn(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(module_inception, stutter, cast_sign_loss, cast_possible_truncation))]

extern crate llvm_sys;

mod types;
mod value;
mod module;
mod function;
mod block;
mod builder;

pub use self::types::*;
pub use self::value::Value;
pub use self::module::Module;
pub use self::function::Function;
pub use self::block::Block;
pub use self::builder::Builder;
