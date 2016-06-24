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
