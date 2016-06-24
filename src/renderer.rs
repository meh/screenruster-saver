use std::time::Instant;
use std::thread;
use std::sync::mpsc::{Receiver, Sender, SendError, channel};

use gl::{self, Surface};
use image::GenericImage;

use {Display, Saver, State, Password};
use util::DurationExt;

pub struct Renderer {
	receiver: Receiver<Response>,
	sender:   Sender<Request>,
}

#[derive(Debug)]
pub enum Request {
	/// Whether the dialog is being opened or closed.
	Dialog(bool),

	/// The password field has changed.
	Password(Password),

	/// Start the rendering.
	Start,

	/// Stop the rendering.
	Stop,
}

#[derive(Clone, Debug)]
pub enum Response {
	/// The renderer has been initialized.
	Initialized,

	/// The rendering has started.
	Started,

	/// The rendering has stopped.
	Stopped,
}

impl Renderer {
	pub fn new<S: Saver + Send + 'static>(display: String, screen: i32, window: u64, mut saver: S) -> Renderer {
		let (sender, i_receiver) = channel();
		let (i_sender, receiver) = channel();

		thread::spawn(move || {
			let display = Display::open(display, screen, window).unwrap();
			let texture = {
				let image = display.screenshot();
				let size  = image.dimensions();
				let image = gl::texture::RawImage2d::from_raw_rgba_reversed(
					image.to_rgba().into_raw(), size);

				gl::texture::Texture2d::new(&display.context(), image).unwrap()
			};

			saver.graphics(display.context());
			sender.send(Response::Initialized).unwrap();

			sender.send(Response::Started).unwrap();
			saver.begin();

			let     step     = (S::step() * 1_000_000.0).round() as u64;
			let mut lag      = 0;
			let mut previous = Instant::now();

			'render: loop {
				let now     = Instant::now();
				let elapsed = now.duration_since(previous);

				previous  = now;
				lag      += elapsed.as_nanosecs();

				// Update the state.
				while lag >= step {
					saver.update();

					if saver.state() == State::None {
						break 'render;
					}

					lag -= step;
				}

				// Check if we received any requests.
				if let Ok(event) = receiver.try_recv() {
					match event {
						Request::Stop => {
							saver.end();
						}

						Request::Dialog(active) => {
							saver.dialog(active);
						}

						Request::Password(password) => {
							saver.password(password);
						}

						_ => ()
					}
				}

				let mut target = display.draw();
				target.clear_all((0.0, 0.0, 0.0, 1.0), 1.0, 0);
				saver.render(&mut target, &texture);
				target.finish().unwrap();
			}

			sender.send(Response::Stopped).unwrap();
		});

		Renderer {
			receiver: i_receiver,
			sender:   i_sender,
		}
	}

	pub fn dialog(&self, active: bool) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Dialog(active))
	}

	pub fn password(&self, password: Password) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Password(password))
	}

	pub fn start(&self) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Start)
	}

	pub fn stop(&self) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Stop)
	}
}

impl AsRef<Receiver<Response>> for Renderer {
	fn as_ref(&self) -> &Receiver<Response> {
		&self.receiver
	}
}

impl AsRef<Sender<Request>> for Renderer {
	fn as_ref(&self) -> &Sender<Request> {
		&self.sender
	}
}
