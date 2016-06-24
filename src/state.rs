/// Represents the state of the saver.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum State {
	/// The saver is inactive.
	None,

	/// The saver is beginning.
	Begin,

	/// The saver is running.
	Running,

	/// The saver is ending.
	End,
}

impl Default for State {
	fn default() -> State {
		State::None
	}
}
