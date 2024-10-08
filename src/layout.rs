#![allow(unused)]

const EMPTY: Vec<Layout> = Vec::new();

#[derive(Clone, Copy, Debug)]
enum Direction {
	Vertical,
	Horizontal,
}

#[derive(Clone, Copy, Debug)]
enum Size {
	Auto,
	Fraction(usize),
	Length(usize),
}

fn auto() -> Layout {
	Layout::new(Direction::Vertical, Size::Auto, EMPTY)
}
fn length(len: usize) -> Layout {
	Layout::new(Direction::Vertical, Size::Length(len), EMPTY)
}
fn fraction(frac: usize) -> Layout {
	Layout::new(Direction::Vertical, Size::Fraction(frac), EMPTY)
}

impl From<Layout> for Size {
	fn from(value: Layout) -> Size {
		value.size
	}
}

#[derive(Clone, Debug)]
struct Layout {
	direction: Direction,
	size: Size,
	children: Vec<Layout>,
}

impl Layout {
	fn new(
		direction: Direction,
		size: Size,
		children: Vec<Layout>,
	) -> Self {
		let children = children.into_iter().map(|c| c.into()).collect();
		Self {
			direction,
			size,
			children,
		}
	}
}

impl From<Size> for Layout {
	fn from(value: Size) -> Self {
		Self {
			direction: Direction::Vertical,
			size: value,
			children: vec![],
		}
	}
}

macro_rules! vertical {
	($($c:expr),+ $(,)?) => {
		Layout::new(Direction::Vertical, Size::Auto, vec![$($c),+])
	};
	($s:expr; $($c:expr),+ $(,)?) => {
		Layout::new(Direction::Vertical, $s.into(), vec![$($c),+])
	};
}

macro_rules! horizontal {
	($($c:expr),+ $(,)?) => {
		Layout::new(Direction::Horizontal, Size::Auto, vec![$($c),+])
	};
	($s:expr; $($c:expr),+ $(,)?) => {
		Layout::new(Direction::Horizontal, $s.into(), vec![$($c),+])
	};
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	pub fn vertical() {
		let x: Layout = vertical! { length(100);
			auto(),
			fraction(1),
		};

		println!("{:#?}", x);
	}

	#[test]
	pub fn horizontal() {
		let x: Layout = horizontal! { fraction(1);
			length(20),
			fraction(1),
			length(20),
		};

		println!("{:#?}", x);
	}

	#[test]
	pub fn nested() {
		let x: Layout = horizontal! {
			auto(),
			vertical! {
				length(10), length(20),
			},
			horizontal! { fraction(1);
				length(30),
				vertical! {
					length(1), length(2),
				}
			}
		};

		println!("{:#?}", x);
	}
}
