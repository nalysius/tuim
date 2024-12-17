/// This module contains the definition of the Table UI element.
/// TODO: implement Cell? Line?

use super::{UIElement, Size};

/// This trait aims to make the Table struct replaceable by any struct which
/// implement it.
pub trait TableTrait {
	/// Creates a new instance.
	///
	/// # Parameters
	/// - headers: contains the line of headers. MUST have the same length
	///   than data.
	/// - data: contains the lines of data. MUST have the same length than headers.
	fn new(headers: &[String], data: Vec<Vec<String>>) -> Self;
	/// Returns the line of headers.
	fn headers(&self) -> Vec<String>;
	/// Returns the lines of data.
	fn data(&self) -> &Vec<Vec<String>>;
	/// Returns the vertical border character.
	fn border_vertical(&self) -> char;
	/// Returns the horizontal border character.
	fn border_horizontal(&self) -> char;
	/// Returns the intersection border character.
	fn border_intersect(&self) -> char;
	/// Returns the vertical padding.
	fn padding_vertical(&self) -> u8;
	/// Returns the horizontal padding.
	fn padding_horizontal(&self) -> u8;
}

/// Represents the UI element Table.
pub struct Table {
	/// The line of headers.
	headers: Vec<String>,
	/// The lines & columns contained in the table.
	/// Doesn't include the headers.
	data: Vec<Vec<String>>,
	/// The character to display in vertical border.
	/// Example: |
	border_vertical: char,
	/// The character to display in horizontal border.
	/// Example: -
	border_horizontal: char,
	/// The character to display in border intersection.
	/// Example: +
	border_intersect: char,
	/// The vertical space between the border and the text inside the table.
	padding_vertical: u8,
	/// The horizontal space between the border and the text inside the table.
	padding_horizontal: u8,
	/// The title of the table.
	title: String,
	/// The width of the table.
	width: Size,
	/// The height of the table.
	height: Size,
}

impl TableTrait for Table {
	/// Create a new Table.
	fn new(headers: &[String], data: Vec<Vec<String>>) -> Self {
		Self {
			headers: headers.to_vec(),
			data: data.to_vec(),
			border_vertical: '|',
			border_horizontal: '-',
			border_intersect: '+',
			padding_vertical: 1,
			padding_horizontal: 1,
			title: String::new(),
			width: Size::Auto,
			height: Size::Auto,
		}
	}

	/// Returns the line of headers.
	fn headers(&self) -> Vec<String> {
		self.headers.clone()
	}

	/// Returns the lines of data.
	fn data(&self) -> &Vec<Vec<String>> {
		&self.data
	}
	/// Returns the vertical border character.
	fn border_vertical(&self) -> char {
		self.border_vertical
	}
	/// Returns the horizontal border character.
	fn border_horizontal(&self) -> char {
		self.border_horizontal
	}
	/// Returns the intersection border character.
	fn border_intersect(&self) -> char {
		self.border_intersect
	}
	/// Returns the vertical padding.
	fn padding_vertical(&self) -> u8 {
		self.padding_vertical
	}
	/// Returns the horizontal padding.
	fn padding_horizontal(&self) -> u8 {
		self.padding_horizontal
	}
}

impl UIElement for Table {
	/// Returns the title of the table.
	fn title(&self) -> &str {
		self.title.as_str()
	}

	/// Returns the width of the table.
	fn width(&self) -> Size {
		self.width.clone()
	}

	/// Returns the height of the table.
	fn height(&self) -> Size {
		self.height.clone()
	}
}
