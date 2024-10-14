use crossterm::{cursor, event::MouseEvent, Command as _};

use super::Widget;
use crate::{Area, Message};

pub struct Border {
	inner: Box<dyn Widget>,

	top: String,
	bottom: String,
	left: String,
	right: String,
	top_left: String,
	top_right: String,
	bottom_left: String,
	bottom_right: String,

	forward: bool,
}

impl Border {
	pub fn new(
		inner: Box<dyn Widget>,

		top: String,
		bottom: String,
		left: String,
		right: String,
		top_left: String,
		top_right: String,
		bottom_left: String,
		bottom_right: String,
	) -> Box<Border> {
		Box::new(Border {
			inner,
			forward: false,

			top,
			bottom,
			left,
			right,
			top_left,
			top_right,
			bottom_left,
			bottom_right,
		})
	}

	pub fn forward(mut self: Box<Border>, yes: bool) -> Box<Border> {
		self.forward = yes;
		self
	}
}

impl Widget for Border {
	fn get_width_hint(&self) -> u16 {
		self.inner.get_width_hint()
			+ self.left.len() as u16
			+ self.right.len() as u16
	}

	fn get_height_hint(&self) -> u16 {
		self.inner.get_height_hint()
			+ self.top.len() as u16
			+ self.bottom.len() as u16
	}

	fn render(&self, area: Area) -> String {
		todo!();
	}

	fn process_mouse(&self, event: MouseEvent, area: Area) -> Message {
		if !area.contains(event) {
			return Message::empty();
		}

		// TODO: true length
		// TODO: area methods
		// XXX: overall, that's some ugly, ugly code.  Terminal length
		// should probably be implemented via a trait.
		let inner_area = Area {
			x: area.x + self.left.len() as u16,
			y: area.y + self.top.len() as u16,
			width: area.width
				- self.left.len() as u16 - self.right.len()
				as u16,
			height: area.height
				- self.top.len() as u16 - self.bottom.len()
				as u16,
		};

		if self.forward && area.contains(event) {
			// TODO: this won't work, because the child widget
			// checks if the event is inside the inner area.
			// Duplicating this check in every widget was a bad idea
			// anyways.  It should be done on program level.
			return self.inner.process_mouse(event, inner_area);
		}
		if !self.forward && inner_area.contains(event) {
			return self.inner.process_mouse(event, inner_area);
		}

		return Message::empty();
	}
}
