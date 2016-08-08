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
