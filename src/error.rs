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
//  0. You just DO WHAT THE FUCK YOU WANT TO.use std::io::{Read, BufRead, BufReader, Write};

use std::fmt;
use std::error;
use std::io;
use std::env;

#[cfg(feature = "renderer")]
use gl;
use log;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	#[cfg(feature = "renderer")]
	ContextCreation(gl::GliumCreationError<Display>),
	#[cfg(feature = "renderer")]
	SwapBuffers(gl::SwapBuffersError),
	Env(env::VarError),
	Logger(log::SetLoggerError),
	Protocol,
}

#[cfg(feature = "renderer")]
#[derive(Debug)]
pub enum Display {
	NotFound,
	Visual,
	Context,
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
	}
}

impl From<env::VarError> for Error {
	fn from(value: env::VarError) -> Self {
		Error::Env(value)
	}
}

impl From<log::SetLoggerError> for Error {
	fn from(value: log::SetLoggerError) -> Self {
		Error::Logger(value)
	}
}

#[cfg(feature = "renderer")]
impl From<Display> for Error {
	fn from(value: Display) -> Self {
		Error::ContextCreation(gl::GliumCreationError::BackendCreationError(value))
	}
}

#[cfg(feature = "renderer")]
impl From<gl::GliumCreationError<Display>> for Error {
	fn from(value: gl::GliumCreationError<Display>) -> Self {
		Error::ContextCreation(value)
	}
}

#[cfg(feature = "renderer")]
impl From<gl::SwapBuffersError> for Error {
	fn from(value: gl::SwapBuffersError) -> Self {
		Error::SwapBuffers(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Io(ref err) =>
				err.description(),

			#[cfg(feature = "renderer")]
			Error::ContextCreation(..) =>
				"OpenGL error.",

			#[cfg(feature = "renderer")]
			Error::SwapBuffers(ref err) =>
				err.description(),

			Error::Env(ref err) =>
				err.description(),

			Error::Logger(ref err) =>
				err.description(),

			Error::Protocol =>
				"Protocol error.",
		}
	}
}
