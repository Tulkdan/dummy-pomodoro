use std::{env, process};
use self::pom::Pom;

mod pom;

extern crate chrono;
extern crate serde;

type DurationType = i64;

fn help() {
	println!("Usage: pom [start|stop|help]\n");
	println!("Run pom without any options to get status of the current session\n");
	println!("start");
	println!("  - Starts pomodoro session(default duration: 25m)");
	println!("  - Takes duration(in minutes) of pomodoro session as an optional argument\n");
	println!("stop");
	println!("  - Stop pomodoro session\n");
	println!("help");
	println!("  - Displays this help information");
}

fn get_duration() -> Option<DurationType> {
	let dur = 25;

	Some(dur)
}

fn main() {
	let mut args = env::args().skip(1);

	if args.len() == 0 {
		Pom::print();
		process::exit(1)
	}

	let first_arg = args.nth(0).unwrap();
	match first_arg.as_str() {
		"start" => {
			if let Some(duration) = get_duration() {
				let pomodoro_timer = Pom::start(&duration);
				pomodoro_timer.write_file();
			}
		},
		"stop" => { Pom::stop(); },
		"help" => { help(); },
		_ => { help(); }
	}
}
