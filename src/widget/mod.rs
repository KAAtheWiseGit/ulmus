use crossterm::event::MouseEvent;

mod flexbox;
mod text;

pub use flexbox::Flexbox;
pub use text::Text;

use crate::{Area, Message};

pub trait Widget {
	fn get_width_hint(&self) -> usize;

	fn get_height_hint(&self) -> usize;

	fn render(&self, area: Area) -> String;

	fn on_click(&self, area: Area, event: MouseEvent) -> Message;
}
