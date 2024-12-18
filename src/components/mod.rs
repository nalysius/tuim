/// The components module contains the UI components.

pub mod table;

/// The UIElement trait contains methods to be implemented by all
/// UI elements (e.g. Table)
pub trait UIElement {
	/// Returns the z index of the UI element.
	///
	/// Elements with a greater z index are printed after
	/// the the ones with a lower index. This way, it's possible for
	/// an element to be printed above the others (e.g. Popup).
	fn z_index(&self) -> u8 {
		0
	}

	/// Sets the z index of the element.
	fn set_z_index(&mut self, z_index: u8);

	/// Returns the title of the UI element.
	///
	/// By default the title is empty.
	fn title(&self) -> &str {
		""
	}

	/// Sets the title.
	fn set_title(&mut self, title: &str);
	/// Returns the width of the element.
	fn width(&self) -> Size;
	/// Sets the width.
	fn set_width(&mut self, width: Size);
	/// Returns the height of the element.
	fn height(&self) -> Size;
	/// Sets the height.
	fn set_height(&mut self, height: Size);
	/// Returns whether the element has been updated and needs to be redrawn.
	fn updated(&self) -> bool;
	/// Changes the update state of the element.
	fn set_updated(&mut self, updated: bool);
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

/// Position specifies the type of positioning used for an element.
pub enum Position {
	/// The element is placed relative to the screen.
	Absolute,
	/// The element is placed relative to the container element.
	Relative,
}

/// Size represents a size with a unit.
#[derive(Clone)]
pub enum Size {
	/// Compute the size automatically, based on the size of the siblings.
	/// The default is to share the space with siblings elements.
	Auto,
	/// A size, given in chars.
	Chars(u8),
	/// A size, given in percentage.
	Percents(u8),
}

/// Represents a layout.
///
/// A layout is a way to display the UI elements in a container.
#[derive(Clone)]
pub enum Layout {
	/// Horizontal
	/// Left to right
	Horizontal,
	/// Vertical
	/// Top to bottom.
	Vertical,
	/// Tabbed
	Tabbed,
}

/// A container for UI elements.
pub struct Container {
	/// The z index of the element.
	z_index: u8,
	/// The title of the container.
	title: String,
	/// The width of the container.
	width: Size,
	/// The height of the container.
	height: Size,
	/// Whether the container has been updated.
	updated: bool,
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
	/// The layout.
	///
	/// The elements will be displayed using this layout.
	pub layout: Layout,
	/// The UI elements to display.
	pub elements: Vec<Box<dyn UIElement>>,
}

impl Container {
	/// Creates a new Container.
	///
	/// # Parameters
	/// - elements: the UI elements to store. The value is moved, to avoid cloning.
	/// - layout: the layout of the container.
	pub fn new(elements: Vec<Box<dyn UIElement>>, layout: Layout) -> Self {
		Self {
			z_index: 0,
			title: "".to_string(),
			width: Size::Auto,
			height: Size::Auto,
			updated: true,
			border_vertical: ' ',
			border_horizontal: ' ',
			border_intersect: ' ',
			padding_vertical: 0,
			padding_horizontal: 0,
			elements: elements,
			layout: layout.clone()
		}
	}
}

impl UIElement for Container {
	/// Returns the z index.
	fn z_index(&self) -> u8 {
		self.z_index
	}

	/// Sets the z index.
	///
	/// TODO: set the z index on all elements?
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
