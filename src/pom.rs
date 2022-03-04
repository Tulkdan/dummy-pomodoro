use std::fs::File;
use std::io::{Write};
use super::{DurationType};
use chrono::{DateTime, Duration, Local};
use serde::ser::{Serialize, Serializer, SerializeStruct};

const PATH: &str = "/tmp/pomodoro.yaml";

pub struct Pom {
	running: bool,
	ends_at: String,
}

impl Pom {
	pub fn start(duration: &DurationType) -> Pom {
		let pomodoro = match Self::read_file() {
			Ok(pomodoro) => pomodoro,
			Err(_) => return Self::new(duration, true)
		};

		if let Some((is_running, minutes_left)) = pomodoro.is_running() {
			if is_running && minutes_left > 0 {
				println!("Already has a pomodoro active with {} minutes left", minutes_left);
				return pomodoro;
			}
		}

		Self::new(duration, true)
	}

	pub fn write_file(&self) {
		let mut file = File::create(PATH).unwrap();

		let s = serde_yaml::to_string(self).unwrap();

		file.write_all(s.as_bytes()).unwrap();
	}

	pub fn stop() {
		let pomodoro = Self::read_file()
			.expect("Pomodoro not runnning");

		if !pomodoro.running {
			println!("Pomodoro not running");
			return
		}

		let empty_pom = Pom {
			running: false,
			ends_at: String::from("")
		};

		empty_pom.write_file();
	}

	pub fn print() {
		let pomodoro = Self::read_file().expect("Could not open file");

		let (_, minutes_left) = pomodoro.is_running().unwrap();

		if minutes_left < 0 {
			println!("💀 {} minutes passed\n", minutes_left);
			return
		}

		println!("🍅 {} minutes left\n", minutes_left);
	}
}

impl Pom {
	fn new(duration: &DurationType, running: bool) -> Pom {
		let now = Local::now();
		let mins_to_add = Duration::seconds(duration * 60);

		let duration = now + mins_to_add;

		Pom { running, ends_at: duration.to_rfc3339() }
	}

	fn read_file() -> Result<Pom, std::io::Error> {
		let file = File::open(PATH)?;

		let yaml: serde_yaml::Value = serde_yaml::from_reader(file).unwrap();

		let running = yaml.get("running")
			.unwrap()
			.as_bool()
			.unwrap();

		let ends_at = yaml.get("ends_at")
			.unwrap()
			.as_str()
			.unwrap()
			.to_string();

		Ok(Pom { running, ends_at })
	}

	fn is_running(&self) -> Option<(bool, DurationType)> {
		if !self.running {
			return None;
		}

		let end_time = DateTime::parse_from_rfc3339(self.ends_at.as_str()).unwrap();

		let minutes_left = Duration::milliseconds(
			end_time.timestamp_millis() - Local::now().timestamp_millis()
		).num_minutes();

		Some((true, minutes_left))
	}
}

impl Serialize for Pom {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Pom", 2)?;
        s.serialize_field("running", &self.running)?;
        s.serialize_field("ends_at", &self.ends_at)?;
        s.end()
    }
}

impl std::fmt::Display for Pom {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Status: {}; Time to end: {}", self.running, self.ends_at)
	}
}
