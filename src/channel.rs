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

use std::io::{Read, BufRead, BufReader, Write};
use std::thread;

use json::{self, JsonValue, object};
use crossbeam_channel::{unbounded, Receiver, RecvError, Sender, SendError};

use crate::error;
use crate::{Safety, Password, Pointer};

/// Communication between locker and saver.
pub struct Channel {
	receiver: Receiver<Request>,
	sender:   Sender<Response>,
}

#[derive(Clone, Debug)]
pub enum Request {
	/// Saver configuration.
	Config(JsonValue),

	/// Drawable target.
	Target {
		display: String,
		screen:  i32,
		window:  u64,
	},

	/// Resize the viewport.
	Resize {
		width: u32,
		height: u32,
	},

	/// Throttle the rendering.
	Throttle(bool),

	/// The screen has been blanked or unblanked.
	Blank(bool),

	/// The locker safety level.
	Safety(Safety),

	/// Pointer events.
	Pointer(Pointer),

	/// The password field has changed.
	Password(Password),

	/// Start the saver.
	Start,

	/// Lock the saver.
	Lock,

	/// Stop the saver.
	Stop,
}

#[derive(Clone, Debug)]
pub enum Response {
	/// The saver has been initialized.
	Initialized,

	/// The saver has started.
	Started,

	/// The saver has stopped.
	Stopped,
}

impl Channel {
	/// Open the channel on the given input and output streams.
	pub fn open<R: Read + Send + 'static, W: Write + Send + 'static>(input: R, output: W) -> error::Result<Channel> {
		let (sender, i_receiver) = unbounded();
		let (i_sender, receiver) = unbounded();

		// Reader.
		thread::spawn(move || {
			macro_rules! json {
				($body:expr) => (
					if let Some(value) = $body {
						value
					}
					else {
						continue;
					}
				);
			}

			for line in BufReader::new(input).lines() {
				if line.is_err() {
					break;
				}

				if let Ok(message) = json::parse(&line.unwrap()) {
					sender.send(match json!(message["type"].as_str()) {
						"config" => {
							Request::Config(message["config"].clone())
						}

						"target" => {
							Request::Target {
								display: json!(message["display"].as_str()).into(),
								screen:  json!(message["screen"].as_i32()),
								window:  json!(message["window"].as_u64()),
							}
						}

						"resize" => {
							Request::Resize {
								width:  json!(message["width"].as_u32()),
								height: json!(message["height"].as_u32()),
							}
						}

						"throttle" => {
							Request::Throttle(json!(message["throttle"].as_bool()))
						}

						"blank" => {
							Request::Blank(json!(message["blank"].as_bool()))
						}

						"safety" => {
							Request::Safety(match json!(message["safety"].as_str()) {
								"high"   => Safety::High,
								"medium" => Safety::Medium,
								"low"    => Safety::Low,

								_ =>
									continue
							})
						}

						"pointer" => {
							if !message["move"].is_null() {
								Request::Pointer(Pointer::Move {
									x: json!(message["move"]["x"].as_i32()),
									y: json!(message["move"]["y"].as_i32()),
								})
							}
							else if !message["button"].is_null() {
								Request::Pointer(Pointer::Button {
									x: json!(message["button"]["x"].as_i32()),
									y: json!(message["button"]["y"].as_i32()),

									button: json!(message["button"]["button"].as_u8()),
									press:  json!(message["button"]["press"].as_bool()),
								})
							}
							else {
								continue;
							}
						}

						"password" => {
							Request::Password(match json!(message["password"].as_str()) {
								"insert"  => Password::Insert,
								"delete"  => Password::Delete,
								"reset"   => Password::Reset,
								"check"   => Password::Check,
								"success" => Password::Success,
								"failure" => Password::Failure,

								_ =>
									continue
							})
						}

						"start" => {
							Request::Start
						}

						"lock" => {
							Request::Lock
						}

						"stop" => {
							Request::Stop
						}

						_ =>
							continue
					}).unwrap();
				}
			}
		});

		// Writer.
		thread::spawn(move || {
			let mut output = output;

			while let Ok(response) = receiver.recv() {
				output.write_all(json::stringify(match response {
					Response::Initialized => object!{
						"type" => "initialized"
					},

					Response::Started => object!{
						"type" => "started"
					},

					Response::Stopped => object!{
						"type" => "stopped"
					},
				}).as_bytes()).unwrap();

				output.write_all(b"\n").unwrap();
			}
		});

		Ok(Channel {
			receiver: i_receiver,
			sender:   i_sender,
		})
	}

	/// Receive a message from the locker.
	pub fn recv(&self) -> Result<Request, RecvError> {
		self.receiver.recv()
	}

	/// Send a message to the locker.
	pub fn send(&self, response: Response) -> Result<(), SendError<Response>> {
		self.sender.send(response)
	}
}

impl AsRef<Receiver<Request>> for Channel {
	fn as_ref(&self) -> &Receiver<Request> {
		&self.receiver
	}
}

impl AsRef<Sender<Response>> for Channel {
	fn as_ref(&self) -> &Sender<Response> {
		&self.sender
	}
}
