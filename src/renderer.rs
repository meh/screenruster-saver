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

use std::time::{Instant, Duration};
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender, SendError};
use gl::{self, Surface};
use picto;
use log::warn;

use crate::{Display, Saver, State, Safety, Password, Pointer};
use crate::util::DurationExt;

pub struct Renderer {
	receiver: Receiver<Response>,
	sender:   Sender<Request>,
}

#[derive(Debug)]
pub enum Request {
	/// Resize the renderer viewport.
	Resize {
		width:  u32,
		height: u32,
	},

	/// Throttle the rendering.
	Throttle(bool),

	/// Pause the rendering on blank.
	Blank(bool),

	/// Change the safety level.
	Safety(Safety),

	/// The pointer has generated events.
	Pointer(Pointer),

	/// The password field has changed.
	Password(Password),

	/// Start the rendering.
	Start,

	/// The screen has been locked.
	Lock,

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

const STEP: u64 = 15_000_000;

impl Renderer {
	pub fn new<S: Saver + Send + 'static>(display: String, screen: i32, window: u64, mut saver: S) -> Renderer {
		let (sender, i_receiver) = unbounded();
		let (i_sender, receiver) = unbounded();

		thread::spawn(move || {
			let mut display  = Display::open(display, screen, window).unwrap();
			let mut blank    = false;
			let mut throttle = false;
			let mut skip     = false;

			// Put the current screen in a texture.
			let texture = {
				let image = display.screenshot::<picto::color::Rgba, u8>();
				let size  = image.dimensions();
				let image = gl::texture::RawImage2d::from_raw_rgba_reversed(
					&image.into_raw(), size);

				gl::texture::Texture2d::new(&display.context(), image).unwrap()
			};

			// Initialize the saver.
			saver.initialize(display.context());
			sender.send(Response::Initialized).unwrap();

			// Handle some initial settings before starting.
			while let Ok(message) = receiver.recv() {
				match message {
					Request::Start => {
						break;
					}

					Request::Throttle(value) => {
						throttle = value;
					}

					Request::Blank(value) => {
						blank = value;
					}

					Request::Safety(level) => {
						saver.safety(level);
					}

					Request::Lock => {
						saver.lock();
					}

					event => {
						warn!("unexpected event before start: {:?}", event);
					}
				}
			}

			// Start the saver.
			saver.start();
			sender.send(Response::Started).unwrap();

			let mut lag      = 0;
			let mut previous = Instant::now();

			'render: loop {
				let now     = Instant::now();
				let elapsed = now.duration_since(previous);

				// Calculate accumulated lag.
				previous  = now;
				lag      += elapsed.as_nanosecs();

				// Update the state by 15ms steps.
				while lag >= STEP {
					saver.update();

					if saver.state() == State::None {
						break 'render;
					}

					lag -= STEP;
				}

				// Handle requests.
				while let Ok(event) = receiver.try_recv() {
					match event {
						Request::Resize { width, height } => {
							display.resize(width, height);
							saver.resize(display.context());
						}

						Request::Throttle(value) => {
							saver.throttle(value);
							throttle = value;
						}

						Request::Blank(value) => {
							saver.blank(value);
							blank = value;
						}

						Request::Safety(level) => {
							saver.safety(level)
						}

						Request::Pointer(pointer) => {
							saver.pointer(pointer);
						}

						Request::Password(password) => {
							saver.password(password);
						}

						Request::Lock => {
							saver.lock();
						}

						Request::Stop => {
							saver.stop();
						}

						_ => ()
					}
				}

				// While throttling we skip every other frame, so it stays at 30 FPS.
				if throttle {
					if skip {
						skip = false;

						thread::sleep(Duration::from_millis(16));
						continue;
					}
					else {
						skip = true;
					}
				}

				// Do not waste time rendering when the screen is blanked.
				if !blank {
					let mut target = display.draw();
					target.clear_all((0.0, 0.0, 0.0, 1.0), 1.0, 0);
					saver.render(&mut target, &texture);
					target.finish().unwrap();
				}

				// If the rendering was too fast, throttle it at 60 FPS.
				if now.elapsed().as_nanosecs() < 16_000_000 {
					thread::sleep(Duration::new(0, 16_000_000 - now.elapsed().as_nanosecs() as u32));
				}
			}

			sender.send(Response::Stopped).unwrap();
		});

		Renderer {
			receiver: i_receiver,
			sender:   i_sender,
		}
	}

	pub fn resize(&self, width: u32, height: u32) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Resize { width: width, height: height })
	}

	pub fn throttle(&self, value: bool) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Throttle(value))
	}

	pub fn blank(&self, value: bool) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Blank(value))
	}

	pub fn safety(&self, value: Safety) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Safety(value))
	}

	pub fn pointer(&self, pointer: Pointer) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Pointer(pointer))
	}

	pub fn password(&self, password: Password) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Password(password))
	}

	pub fn start(&self) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Start)
	}

	pub fn lock(&self) -> Result<(), SendError<Request>> {
		self.sender.send(Request::Lock)
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
