use crossterm::event::MouseEvent;

mod border;
mod flexbox;
mod text;

pub use flexbox::{Flexbox, Size};
pub use text::Text;
pub use border::Border;

use crate::{Area, Message};

pub trait Widget {
	fn get_width_hint(&self) -> u16;

	fn get_height_hint(&self) -> u16;

	fn render(&self, area: Area) -> String;

	fn process_mouse(&self, event: MouseEvent, area: Area) -> Message;
}
