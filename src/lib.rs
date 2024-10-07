#![doc = include_str!("../README.md")]

mod interface;
mod program;
mod reactive;
mod view;
pub mod widget;

pub use interface::{Cmd, Model, Msg, Subroutine, TermCommand};
pub use program::Program;
