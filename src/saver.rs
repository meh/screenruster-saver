use std::rc::Rc;

use gl;
use json;

use {State, Password};

#[allow(unused_variables)]
pub trait Saver {
	/// The update step for the render loop.
	fn step() -> f64 { 0.015 }

	/// Initialize configuration.
	fn config(&mut self, config: json::JsonValue) { }

	/// Initialize any graphics related stuff.
	fn initialize(&mut self, context: Rc<gl::backend::Context>) { }

	/// Resize the viewport.
	fn resize(&mut self, context: Rc<gl::backend::Context>) { }

	/// The dialog is now `active` or not.
	fn dialog(&mut self, active: bool) { }

	/// The password is being interacted with.
	fn password(&mut self, value: Password) { }

	/// The saver has been started, useful to implement a fade in or animation to
	/// only show at the beginning.
	fn begin(&mut self);

	/// The saver has been stopped, useful to implement a fade out or animation
	/// to show at the end.
	fn end(&mut self);

	/// Return the current saver state.
	fn state(&self) -> State;

	/// Called each `step` milliseconds.
	fn update(&mut self) { }

	/// Render the saver.
	fn render<S: gl::Surface>(&self, target: &mut S, screen: &gl::texture::Texture2d);
}
