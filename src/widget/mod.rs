mod text;

pub use text::Text;

use crate::Reactive;

/// A trait which describes a composable widget.
pub trait Widget: Reactive + crossterm::Command {
	/// Sets the display width of the widget to exactly `width`.  If the
	/// widget is smaller than that, it should pad or stretch.
	///
	/// If `width` is `None`, the widget is allowed to determine its width
	/// by itself.
	fn set_width(&mut self, width: Option<usize>);

	/// Analogous to [`set_width`].
	///
	/// [`set_width`]: Widget#tymethod.set_width
	fn set_height(&mut self, height: Option<usize>);

	/// Returns the display width of the widget.  If wigth was previosly set
	/// `set_width`, this method must return the same value.  If `set_width`
	/// wasn't called or called with `None`, it must return the display
	/// width of lines which will be returned by `lines`.
	fn width(&self) -> usize;

	/// Analogous to [`height`].
	///
	/// [`height`]: Widget#tymethod.height
	fn height(&self) -> usize;
}
