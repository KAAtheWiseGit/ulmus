#![doc = include_str!("../README.md")]

mod model;
mod program;
mod reactive;
mod view;
pub mod widget;

pub use model::Model;
pub use program::Program;
pub use reactive::{Command, Message, Reactive, Subroutine};
pub use view::{Area, View};
