/// The components module contains the UI components.

pub mod table;

/// The UIElement trait contains methods to be implemented by all
/// UI elements (e.g. Table)
pub trait UIElement {
	/// Returns the z index of the UI element.
	///
	/// By default, all elements have the same basic index.
	/// Elements with a greater z index are printed after
	/// the the ones with a lower index. This way, it's possible for
	/// an element to be printed above the others (e.g. Popup).
	fn z_index(&self) -> u8 {
		return 0;
	}
	/// Returns the title of the UI element.
	///
	/// By default the title is empty.
	fn title(&self) -> &str {
		""
	}
	/// Returns the width of the element.
	fn width(&self) -> Size;
	/// Returns the height of the element.
	fn height(&self) -> Size;
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
	/// The layout.
	///
	/// The elements will be displayed using this layout.
	pub layout: Layout,
	/// The UI elements to display.
	pub elements: Vec<Box<dyn UIElement>>,
}
