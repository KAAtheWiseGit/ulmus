use crossterm::{event::Event as CrosstermEvent, Command as CrosstermCommand};

use std::sync::mpsc;

pub type Subroutine<T> = Box<dyn FnOnce(mpsc::Sender<Msg<T>>) + Send>;

pub enum Msg<T: Send + 'static> {
	Term(CrosstermEvent),
	Custom(T),
}

pub enum Cmd<T: Send + 'static> {
	Term(TermCommand),
	Quit,
	Subroutine(Subroutine<T>),
}

pub trait Model: Sized {
	type CustomMsg: Sized + Send + 'static;

	fn init(&self) -> Vec<Cmd<Self::CustomMsg>>;

	fn update(
		&mut self,
		message: Msg<Self::CustomMsg>,
	) -> Vec<Cmd<Self::CustomMsg>>;

	fn view(&self) -> impl AsRef<str>;
}

pub struct TermCommand(String);

/// A seconday type.  It's a hacky workaround because `TermCommand` can't
/// implement `From<T: CrosstermCommand>` and `CrosstermCommand` at  the same
/// time, as it causes it to conflict with the `From<T> for T` implementation
/// from the standard library.
pub struct TermCommandImpl(String);

impl<T: CrosstermCommand> From<T> for TermCommand {
	fn from(value: T) -> Self {
		let mut buffer = String::new();
		if value.write_ansi(&mut buffer).is_err() {
			unreachable!("`String` is an infallible writer");
		}
		Self(buffer)
	}
}

impl Into<TermCommandImpl> for TermCommand {
	fn into(self) -> TermCommandImpl {
		TermCommandImpl(self.0)
	}
}

impl CrosstermCommand for TermCommandImpl {
	fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}
