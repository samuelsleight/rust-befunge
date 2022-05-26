#![cfg_attr(feature = "cargo-clippy", warn(clippy, clippy_pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(module_inception, stutter, cast_sign_loss, cast_possible_truncation)
)]

extern crate llvm_sys;

mod block;
mod builder;
mod function;
mod module;
mod types;
mod value;

pub use self::block::Block;
pub use self::builder::Builder;
pub use self::function::Function;
pub use self::module::Module;
pub use self::types::*;
pub use self::value::Value;
