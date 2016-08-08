//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

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
