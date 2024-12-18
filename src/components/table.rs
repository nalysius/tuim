/// This module contains the definition of the Table UI element.

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
	/// Returns the current page of the table.
	fn current_page(&self) -> u32;
	/// Sets the current page of the table.
	fn set_current_page(&mut self, current_page: u32);
	/// Returns the number of items displayed by page.
	fn items_by_page(&self) -> u32;
	/// Sets the number of items displayed by page.
	fn set_items_by_page(&mut self, items_by_page: u32);
}

/// Represents the UI element Table.
pub struct Table {
	/// The z index.
	z_index: u8,
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
	/// Whether the table has been updated.
	updated: bool,
	/// The current page in the table.
	current_page: u32,
	/// The number of items displayed by page.
	items_by_page: u32
}

impl TableTrait for Table {
	/// Create a new Table.
	fn new(headers: &[String], data: Vec<Vec<String>>) -> Self {
		Self {
			z_index: 0,
			headers: headers.to_vec(),
			data: data.to_vec(),
			border_vertical: ' ',
			border_horizontal: ' ',
			border_intersect: ' ',
			padding_vertical: 1,
			padding_horizontal: 1,
			title: String::new(),
			width: Size::Auto,
			height: Size::Auto,
			updated: true,
			current_page: 0,
			items_by_page: 20,
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

	/// Returns the current page of the table.
	fn current_page(&self) -> u32 {
		self.current_page
	}

	/// Sets the current page of the table.
	fn set_current_page(&mut self, current_page: u32) {
		self.current_page = current_page
	}

	/// Returns the number of items displayed by page.
	fn items_by_page(&self) -> u32 {
		self.items_by_page
	}

	/// Sets the number of items displayed by page.
	fn set_items_by_page(&mut self, items_by_page: u32) {
		self.items_by_page = items_by_page
	}
}

impl UIElement for Table {
	/// Returns the z index.
	fn z_index(&self) -> u8 {
		self.z_index
	}

	/// Sets the z index.
	fn set_z_index(&mut self, z_index: u8) {
		self.z_index = z_index;
	}
	
	/// Returns the title of the table.
	fn title(&self) -> &str {
		self.title.as_str()
	}

	/// Sets the title.
	fn set_title(&mut self, title: &str) {
		self.title = title.to_string();
	}

	/// Returns the width of the table.
	fn width(&self) -> Size {
		self.width.clone()
	}

	/// Sets the width.
	fn set_width(&mut self, width: Size) {
		self.width = width.clone();
	}

	/// Returns the height of the table.
	fn height(&self) -> Size {
		self.height.clone()
	}

	/// Sets the height.
	fn set_height(&mut self, height: Size) {
		self.height = height.clone();
	}

	/// Returns whether the table has been updated and
	/// needs to be redrawn.
	fn updated(&self) -> bool {
		self.updated
	}

	/// Changes the state of the element.
	fn set_updated(&mut self, updated: bool) {
		self.updated = updated;
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
