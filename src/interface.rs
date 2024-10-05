use crossterm::event::Event as CrosstermEvent;

pub enum Msg<T> {
	Term(CrosstermEvent),
	Custom(T),
}

pub trait Model: Sized {
	type CustomMsg: Sized;

	fn init() -> Self;

	fn update(&mut self, message: Msg<Self::CustomMsg>);

	fn view(&self) -> impl AsRef<str>;
}
