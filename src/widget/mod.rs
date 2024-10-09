use crossterm::event::MouseEvent;

mod flexbox;
mod text;

pub use flexbox::Flexbox;
pub use text::Text;

use crate::{Message, View};

pub trait Widget: View {
	fn get_width_hint(&self) -> usize;

	fn get_height_hint(&self) -> usize;

	fn on_click(&self, event: MouseEvent) -> Message;
}
