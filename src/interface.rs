use crossterm::{event::Event as CrosstermEvent, Command as CrosstermCommand};

use std::sync::mpsc;

type Subroutine<T> = Box<dyn FnOnce(mpsc::Sender<Msg<T>>) + Send>;

pub enum Msg<T: Send + 'static> {
	Term(CrosstermEvent),
	Custom(T),
}

pub enum Cmd<T: Send + 'static> {
	// TODO implement an opaque type, which can supports From for crossterm
	// commands
	Term,
	Quit,
	Subroutine(Subroutine<T>),
}

pub trait Model: Sized {
	type CustomMsg: Sized + Send + 'static;

	fn init() -> Self;

	fn update(
		&mut self,
		message: Msg<Self::CustomMsg>,
	) -> Cmd<Self::CustomMsg>;

	fn view(&self) -> impl AsRef<str>;
}
