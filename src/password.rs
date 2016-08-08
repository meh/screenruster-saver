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

/// Represents the state of the password.
///
/// This is used to let the saver show a dialog or different animations based
/// on the input, without actually knowing what the input is.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Password {
	/// A character has been inserted.
	Insert,

	/// A character has been deleted.
	Delete,

	/// The field has been reset.
	Reset,

	/// The password is being checked.
	Check,

	/// The authentication was successful.
	Success,

	/// The authentication failed.
	Failure,
}
