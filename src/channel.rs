use std::io::{Read, BufRead, BufReader, Write};
use std::thread;
use std::sync::mpsc::{Receiver, RecvError, Sender, SendError, channel};

use json::{self, JsonValue};

use {Password};

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

	/// Whether the dialog is being opened or closed.
	Dialog(bool),

	/// The password field has changed.
	Password(Password),

	/// Start the saver.
	Start,

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
	pub fn new<R: Read + Send + 'static, W: Write + Send + 'static>(input: R, output: W) -> Channel {
		let (sender, i_receiver) = channel();
		let (i_sender, receiver) = channel();

		// Reader.
		thread::spawn(move || {
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

						"dialog" => {
							Request::Dialog(message["active"].as_bool().unwrap_or(false))
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

		Channel {
			receiver: i_receiver,
			sender:   i_sender,
		}
	}

	pub fn recv(&self) -> Result<Request, RecvError> {
		self.receiver.recv()
	}

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
