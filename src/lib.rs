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

#![feature(type_ascription)]

use std::io;
use std::env;

#[cfg(feature = "renderer")]
pub use gl;
#[cfg(feature = "renderer")]
pub use picto;

pub use json;
pub use log::{debug, error, info, log, log_enabled, trace, warn};

#[macro_use]
mod util;

mod error;
pub use error::{Result, Error};

mod state;
pub use state::State;

mod safety;
pub use safety::Safety;

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


/// Initialize the saver.
pub fn init() -> Result<Channel> {
	use std::io::Write;

	// Initialize logger.
	{
		let mut builder = env_logger::Builder::new();
		let     pid     = unsafe { libc::getpid() };

		builder.format(move |buf, record| {
			writeln!(buf, "{}:{}:{}: {}", record.level(), pid, record.module_path().unwrap_or("unknown"), record.args())
		});

		if let Ok(log) = env::var("RUST_LOG") {
			builder.parse(&log);
		}

		builder.init();
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

	use crossbeam_channel::select;

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

	'main: loop {
		select! {
			recv(channel.as_ref()) -> message => {
				match exit!(message) {
					channel::Request::Target { .. } | channel::Request::Config(..) => {
						unreachable!();
					}

					channel::Request::Resize { width, height } => {
						renderer.resize(width, height).unwrap();
					}

					channel::Request::Throttle(value) => {
						renderer.throttle(value).unwrap();
					}

					channel::Request::Blank(value) => {
						renderer.blank(value).unwrap();
					}

					channel::Request::Safety(value) => {
						renderer.safety(value).unwrap();
					}

					channel::Request::Pointer(pointer) => {
						renderer.pointer(pointer).unwrap();
					}

					channel::Request::Password(password) => {
						renderer.password(password).unwrap();
					}

					channel::Request::Start => {
						renderer.start().unwrap();
					}

					channel::Request::Lock => {
						renderer.lock().unwrap();
					}

					channel::Request::Stop => {
						renderer.stop().unwrap();
					}
				}
			},

			recv(renderer.as_ref()) -> message => {
				match exit!(message) {
					renderer::Response::Initialized => {
						channel.send(channel::Response::Initialized).unwrap();
					}

					renderer::Response::Started => {
						channel.send(channel::Response::Started).unwrap();
					}

					renderer::Response::Stopped => {
						break 'main;
					}
				}
			}
		}
	}

	channel.send(channel::Response::Stopped).unwrap();

	Ok(())
}
