#![doc = include_str!("../README.md")]

mod model;
mod program;
mod view;
pub mod widget;

pub use model::{Command, Message, Model, Subroutine};
pub use program::Program;
pub use view::{Area, View};
