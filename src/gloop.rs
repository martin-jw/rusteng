extern crate time;

enum StateMarker {
	Input,
	Update,
	Render,
	Stop
}

pub enum LoopState {
	Render { delta: f64 },
	Update { delta: f64 },
	Input
}

pub struct GameLoop {
	state_marker: StateMarker,
	prev_frame: f64,
	prev_tick: f64,
	elapsed_time: f64,
	frame_delta: f64
}

impl GameLoop {

	pub fn new() -> GameLoop {
		GameLoop {
			state_marker: StateMarker::Input,
			prev_tick: time::precise_time_s(),
			prev_frame: time::precise_time_s(),
			elapsed_time: 0f64,
			frame_delta: 1f64 / 60f64
		}
	}

	pub fn stop(&mut self) {
		self.state_marker = StateMarker::Stop;
	}
}

impl Iterator for GameLoop {
	type Item = LoopState;

	fn next(&mut self) -> Option<Self::Item> {

		let frame_time = time::precise_time_s();
		match self.state_marker {
			StateMarker::Input => {

				self.elapsed_time = frame_time - self.prev_tick;
				
				if self.elapsed_time > self.frame_delta {
					self.state_marker = StateMarker::Update;
				}
				else {
					self.state_marker = StateMarker::Render;
				}

				Some(LoopState::Input)
			}
			StateMarker::Update => {

				self.elapsed_time -= self.frame_delta;

				if self.elapsed_time > self.frame_delta {
					self.state_marker = StateMarker::Update;
					self.elapsed_time -= self.frame_delta;
				}
				else {
					self.state_marker = StateMarker::Render;
					self.prev_tick = frame_time;
				}

				Some(LoopState::Update{ delta: self.frame_delta })
			}
			StateMarker::Render => {
				self.state_marker = StateMarker::Input;

				let d = frame_time - self.prev_frame;
				self.prev_frame = frame_time;

				Some(LoopState::Render{ delta: d })
			}
			StateMarker::Stop => {
				None
			}
		}
	}
}