#[macro_use]
extern crate serenity;
#[macro_use]
extern crate lazy_static;

mod bot;
mod facts;

#[path = "ttt.rs"]
mod ttt;

fn main() {
    match facts::check() {
        Ok(_) => println!("Found facts dir"),
        Err(why) => println!("Couldn\'t find dir: {}", why),
    };
    bot::init();
}
