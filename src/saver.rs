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

use std::rc::Rc;

use gl;
use json;

use {State, Safety, Password, Pointer};

#[allow(unused_variables)]
pub trait Saver {
	/// Initialize configuration.
	fn config(&mut self, config: json::JsonValue) { }

	/// Initialize any graphics related stuff.
	fn initialize(&mut self, context: Rc<gl::backend::Context>) { }

	/// Resize the viewport.
	fn resize(&mut self, context: Rc<gl::backend::Context>) { }

	/// Whether to try and reduce power usage or not.
	fn throttle(&mut self, value: bool) { }

	/// Whether the screen has been blanked or unblanked.
	fn blank(&mut self, value: bool) { }

	/// Whether the screen is actually safe.
	fn safety(&mut self, value: Safety) { }

	/// The pointer moved or clicked.
	fn pointer(&mut self, value: Pointer) { }

	/// The password is being interacted with.
	fn password(&mut self, value: Password) { }

	/// The saver has been started, useful to implement a fade in or animation to
	/// only show at the beginning.
	fn start(&mut self);

	/// The screen has been locked.
	fn lock(&mut self) { }

	/// The saver has been stopped, useful to implement a fade out or animation
	/// to show at the end.
	fn stop(&mut self);

	/// Return the current saver state.
	fn state(&self) -> State;

	/// Called every 15 milliseconds.
	fn update(&mut self) { }

	/// Render the saver.
	fn render<S: gl::Surface>(&self, target: &mut S, screen: &gl::texture::Texture2d);
}
