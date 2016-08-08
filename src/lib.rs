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

#![feature(macro_reexport, type_ascription, question_mark)]
#![cfg_attr(feature = "renderer", feature(mpsc_select))]

#[macro_use]
#[macro_reexport(debug, error, info, log, log_enabled, trace, warn)]
pub extern crate log;
#[doc(hidden)]
pub use log::{LogLocation, LogLevel, __static_max_level, max_log_level, __log, __enabled};
extern crate env_logger;

#[macro_use]
#[macro_reexport(object, array)]
pub extern crate json;
#[doc(hidden)]
pub use json::{JsonValue};

extern crate libc;

#[cfg(feature = "renderer")]
extern crate x11;

#[cfg_attr(feature = "renderer", macro_reexport(implement_vertex, program, uniform))]
#[cfg(feature = "renderer")]
pub extern crate glium as gl;
#[cfg_attr(feature = "renderer", doc(hidden))]
#[cfg(feature = "renderer")]
pub use gl::{program, Version, Api, vertex, backend, uniforms};

#[cfg(feature = "renderer")]
pub extern crate image;

#[macro_use]
mod util;

mod error;
pub use error::{Result, Error};

mod state;
pub use state::State;

mod password;
pub use password::Password;

pub mod pointer;
pub use pointer::Pointer;

mod channel;
pub use channel::{Request, Response, Channel};

#[cfg(feature = "renderer")]
mod saver;
#[cfg(feature = "renderer")]
pub use saver::Saver;

#[cfg(feature = "renderer")]
mod renderer;
#[cfg(feature = "renderer")]
pub use renderer::Renderer;

#[cfg(feature = "renderer")]
mod display;
#[cfg(feature = "renderer")]
pub use display::Display;

use std::io;
use std::env;

/// Initialize the saver.
pub fn init() -> Result<Channel> {
	// Initialize logger.
	{
		let mut builder = env_logger::LogBuilder::new();
		let     pid     = unsafe { libc::getpid() };

		builder.format(move |record| {
			format!("{}:{}:{}: {}", record.level(), pid, record.location().module_path(), record.args())
		});

		if env::var("RUST_LOG").is_ok() {
			builder.parse(env::var("RUST_LOG")?.as_ref());
		}

		builder.init()?;
	}

	Channel::open(io::stdin(), io::stdout())
}

/// Run the saver.
#[cfg(feature = "renderer")]
pub fn run<S: Saver + Send + 'static>(mut saver: S) -> Result<()> {
	macro_rules! exit {
		($body:expr) => (
			if let Ok(value) = $body {
				value
			}
			else {
				break;
			}
		);
	}

	let channel = init()?;

	if let Ok(Request::Config(config)) = channel.recv() {
		saver.config(config);
	}
	else {
		return Err(Error::Protocol);
	}

	let renderer = if let Ok(Request::Target { display, screen, window }) = channel.recv() {
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
				match exit!(message) {
					channel::Request::Target { .. } | channel::Request::Config(..) => {
						unreachable!();
					}

					channel::Request::Throttle(value) => {
						renderer.throttle(value).unwrap();
					}

					channel::Request::Blank(value) => {
						renderer.blank(value).unwrap();
					}

					channel::Request::Resize { width, height } => {
						renderer.resize(width, height).unwrap();
					}

					channel::Request::Start => {
						renderer.start().unwrap();
					}

					channel::Request::Pointer(pointer) => {
						renderer.pointer(pointer).unwrap();
					}

					channel::Request::Password(password) => {
						renderer.password(password).unwrap();
					}

					channel::Request::Lock => {
						renderer.lock().unwrap();
					}

					channel::Request::Stop => {
						renderer.stop().unwrap();
					}
				}
			},

			message = r.recv() => {
				match exit!(message) {
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
