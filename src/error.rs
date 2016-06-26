use std::fmt;
use std::error;
use std::io;

use gl;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	ContextCreation(gl::GliumCreationError<Display>),
	SwapBuffers(gl::SwapBuffersError),
	Protocol,
}

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

impl From<Display> for Error {
	fn from(value: Display) -> Self {
		Error::ContextCreation(gl::GliumCreationError::BackendCreationError(value))
	}
}

impl From<gl::GliumCreationError<Display>> for Error {
	fn from(value: gl::GliumCreationError<Display>) -> Self {
		Error::ContextCreation(value)
	}
}

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

			Error::ContextCreation(..) =>
				"OpenGL error.",

			Error::SwapBuffers(ref err) =>
				err.description(),

			Error::Protocol =>
				"Protocol error.",
		}
	}
}
