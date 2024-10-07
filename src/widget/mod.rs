mod text;

pub use text::Text;

use crate::{Cmd, Msg};

/// A trait which describes a composable widget.
pub trait Widget {
	type WidgetMsg: Sized + Send + 'static;

	/// Updates the widget state.  This method
	fn update(
		&mut self,
		message: Msg<Self::WidgetMsg>,
	) -> Vec<Cmd<Self::WidgetMsg>>;

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

	/// Returns an iterator over the textual lines of a widget.
	///
	/// The widget **must** guarantee that the iterator has exactly the
	/// number of rows set in [`set_height`] and each line, after being
	/// displayed, takes up exactly the number of columns set in
	/// [`set_width`].
	///
	/// [`set_width`]: Widget#tymethod.set_width
	/// [`set_height`]: Widget#tymethod.set_height
	fn lines(&self) -> impl Iterator<Item = &str>;

	/// Returns the view of the widget as a whole.  Has a default
	/// implementation, which uses `lines`, which should be sufficient.
	///
	/// It's a utility method, which can be used to pass a widget to
	/// [`Model.view`].
	///
	/// [`Model.view`]: crate::Model#tymethod.view
	fn as_string_view(&self) -> impl AsRef<str> {
		// +1 for the newline character
		let capacity = (self.width() + 1) * self.height();
		let mut out = String::with_capacity(capacity);

		for line in self.lines() {
			out.push_str(line.as_ref());
		}

		out
	}
}
