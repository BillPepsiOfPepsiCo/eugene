#[macro_use] extern crate serenity;

mod bot;
mod facts;

fn main() {
    match facts::check() {
        Ok(_) => println!("Found facts dir"),
        Err(why) => println!("Couldn\'t find dir: {}", why),
    };
    bot::init();
}


