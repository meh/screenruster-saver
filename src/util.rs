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

use std::time::Duration;

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

pub trait DurationExt {
	fn as_msecs(&self) -> u64;
	fn as_nanosecs(&self) -> u64;
}

impl DurationExt for Duration {
	fn as_msecs(&self) -> u64 {
		self.as_secs() * 1_000 + (self.subsec_nanos() / 1_000) as u64
	}

	fn as_nanosecs(&self) -> u64 {
		self.as_secs() * 1_000_000 + self.subsec_nanos() as u64
	}
}
