mod flexbox;
mod text;

pub use flexbox::Flexbox;
pub use text::Text;

use crate::{Reactive, View};

pub trait Widget: Reactive + View {
	fn get_width_hint(&self) -> usize;

	fn get_height_hint(&self) -> usize;
}
