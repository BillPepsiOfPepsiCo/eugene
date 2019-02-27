use serenity::{
    client::Client,
    framework::standard::{Args, StandardFramework},
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::{env, sync::Mutex};

#[path = "facts.rs"]
mod facts;
#[path = "ttt.rs"]
mod ttt;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {}

    fn ready(&self, _: Context, ready_event: Ready) {
        println!("Ready");
        println!("Ready details: {:?}", ready_event);
    }
}

enum TTTState {
    NOT_IN_PROGRESS,
    PROC_PLAYER1,
    PROC_PLAYER2,
    COMPLETE,
}

lazy_static! {
    static ref ttt_games_mx: Mutex<Vec<ttt::TicTTGame>> = Mutex::new(vec![]);
}

pub fn init() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("Expected token"), Handler)
        .expect("Client creation error");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .on("fact", |_context, msg, mut args| {
                let character: String = args.single::<String>().unwrap();
                match facts::get_fact(character) {
                    Err(why) => println!("Command error: {:?}", why),
                    Ok(fact) => {
                        match msg.channel_id.say(fact) {
                            Ok(msg) => println!("[Facts] Sent message: {:?}", msg),
                            Err(pourquoi) => {
                                println!("[Facts] Error sending message: {:?}", pourquoi)
                            }
                        };
                    }
                };

                Ok(())
            })
            .on("t3", |_context, msg, mut args| {
                //~tictactoe start <player1 piece> <player2-name> <player2-piece>
                let command = args.single::<String>()?;
                if args.len() == 4 && command == "start" {
                    let piece_p1 = args.single::<String>()?;
                    let name_p1 = (&msg.author.name).to_string();
                    let name_p2 = args.single::<String>()?;
                    let piece_p2 = args.single::<String>()?;
                    let player1: ttt::Player = ttt::Player::new(name_p1, piece_p1);
                    let player2: ttt::Player = ttt::Player::new(name_p2, piece_p2);

                    for game in ttt_games_mx.lock().unwrap().iter() {
                        if game.player1 == player1 {
                            msg.channel_id
                                .say(format!("{} is already in a game!", player1.name));
                            return Ok(());
                        } else if game.player2 == player2 {
                            msg.channel_id
                                .say(format!("{} is already in a game!", player2.name));
                            return Ok(());
                        }
                    }

                    msg.channel_id.say(format!(
                        "A new game of tic tac toe has been started between {} and {}!",
                        player1.name, player2.name
                    ));
                    let g = ttt::TicTTGame::new(player1, player2);
                    println!("{}", g);
                    ttt_games_mx.lock().unwrap().push(g);
                } else if args.len() == 1 {
                    //Playing the feud

                }

                Ok(())
            }),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
