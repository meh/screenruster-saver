#![feature(mpsc_select, macro_reexport, type_ascription, question_mark)]

extern crate libc;
extern crate x11;

#[macro_reexport(implement_vertex, program, uniform)]
pub extern crate glium as gl;
#[doc(hidden)]
pub use gl::{program, Version, Api, vertex, backend, uniforms};

pub extern crate image;

#[macro_use]
#[macro_reexport(object, array)]
pub extern crate json;

#[macro_use]
mod util;

mod error;
pub use error::{Result, Error};

mod state;
pub use state::State;

mod password;
pub use password::Password;

mod saver;
pub use saver::Saver;

mod channel;
pub use channel::Channel;

mod renderer;
pub use renderer::Renderer;

mod display;
pub use display::Display;

use std::io;

pub fn run<S: Saver + Send + 'static>(mut saver: S) -> error::Result<()> {
	let channel = Channel::new(io::stdin(), io::stdout());

	if let Ok(channel::Request::Config(config)) = channel.recv() {
		saver.config(config);
	}
	else {
		return Err(Error::Protocol);
	}

	let renderer = if let Ok(channel::Request::Target { display, screen, window }) = channel.recv() {
		Renderer::new(display, screen, window, saver)
	}
	else {
		return Err(Error::Protocol);
	};

	// select! is icky
	let c = channel.as_ref();
	let r = renderer.as_ref();

	loop {
		select! {
			message = c.recv() => {
				match message.unwrap() {
					_ => ()
				}
			},

			message = r.recv() => {
				match message.unwrap() {
					renderer::Response::Initialized => {
						channel.send(channel::Response::Initialized).unwrap();
					}

					renderer::Response::Started => {
						channel.send(channel::Response::Started).unwrap();
					}

					renderer::Response::Stopped => {
						break;
					}
				}
			}
		}
	}

	channel.send(channel::Response::Stopped).unwrap();

	Ok(())
}
