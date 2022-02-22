#[derive(Debug)]
pub struct Team {
	captured_total: u8,
	tens: u8,
}

impl Team {
	pub(crate) fn add_to_captured(&mut self, captured: u8) {
		self.captured_total += captured;
	}

	pub fn get_tens(&self) -> u8 {
		self.tens
	}

	pub fn get_total_captured(&self) -> u8 {
		self.captured_total
	}

	pub(crate) fn increment_tens(&mut self) {
		self.tens += 1;
	}

	pub(crate) fn new() -> Team {
		Team {
			captured_total: 0,
			tens: 0,
		}
	}
}
