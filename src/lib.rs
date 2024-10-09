#![doc = include_str!("../README.md")]

mod area;
mod model;
mod program;
pub mod widget;

pub use area::Area;
pub use model::{Command, Message, Model, Subroutine};
pub use program::Program;
