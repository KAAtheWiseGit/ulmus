#![doc = include_str!("../README.md")]

mod interface;
mod program;
pub mod widget;

pub use interface::{Cmd, Model, Msg, Subroutine, TermCommand};
pub use program::Program;
