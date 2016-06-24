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
