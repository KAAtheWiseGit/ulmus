use crossterm::style::Print;

pub trait View {
	fn view(&self) -> Print<String>;
}
