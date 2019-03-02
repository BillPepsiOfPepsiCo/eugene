#[macro_use]
extern crate serenity;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate parking_lot;
#[macro_use]
extern crate config;

use std::path::Path;

mod bot;
mod facts;

#[path = "ttt.rs"]
mod ttt;

fn main() {
	check_files();
    bot::init();
}

fn check_files() {
	let paths = vec![Path::new("./Eugene.toml"), Path::new("./facts/")];

	for f in paths.iter() {
		if f.exists() {
			println!("Required path found: {:?}", f);
		} else {
			println!("Required path not found: {:?}", f);
		}
	}
}
