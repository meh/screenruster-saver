/// Represents pointer eents.
///
/// This can be used to make an interactive saver or show the dialog.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Pointer {
	/// The pointer has moved.
	Move {
		x: i32,
		y: i32,
	},

	/// A button has been pressed or released.
	Button {
		x: i32,
		y: i32,

		button: u8,
		press:  bool,
	}
}
