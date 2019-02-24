use serenity::{
    framework::standard::StandardFramework,
    client::Client,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::env;

#[path="facts.rs"]
mod facts;

struct Handler;

impl EventHandler for Handler {

    fn message(&self, _: Context, msg: Message) {

    }

    fn ready(&self, _: Context, ready_event: Ready) {
        println!("Ready");
    }
}

pub fn init() { 
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("Expected token"), Handler).expect("Client creation error");

    client.with_framework(StandardFramework
                          ::new()
                          .configure(|c| c.prefix("~"))
                          .on("fact", |_, msg, mut args| {
                              let character: String = args.single::<String>().unwrap();
                              match facts::get_fact(character) {
                                  Err(why) => println!("{}", why), 
                                  Ok(fact) => {
                                      msg.channel_id.say(fact);
                                  },
                              };
                            
                              Ok(())
                          }));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}


