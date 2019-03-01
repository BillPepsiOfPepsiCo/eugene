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

use ttt::{
    GameState::{Win_Player1, Win_Player2, Cat},
    TicTTGame,
    Player,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {}

    fn ready(&self, _: Context, ready_event: Ready) {
        println!("Ready");
        println!("Ready details: {:?}", ready_event);
    }
}


//The static vector that stores each in-progress game.
lazy_static! {
    static ref ttt_games_mx: Mutex<Vec<TicTTGame>> = Mutex::new(vec![]);
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
                    //This function is for detecting a leading @, in which case
                    //it is sanitized.
                    let sanitize_at = |s: String| -> String {
                        match s.find('@') {
                            Some(usize) => {
                                s[1..s.len() - 1].to_string()
                            },
                            None => s
                        }
                    };

                    let piece_p1 = args.single::<String>()?;
                    let name_p1 = sanitize_at((&msg.author.name).to_string());
                    let name_p2 = sanitize_at(args.single::<String>()?);
                    let piece_p2 = args.single::<String>()?;
                    let player1: Player = Player::new(name_p1, piece_p1);
                    let player2: Player = Player::new(name_p2, piece_p2);

                    for game in ttt_games_mx.lock().unwrap().iter() {
                        if game.player1 == player1 {
                            handleoutmsg(&msg, format!("{} is already in a game!", player1.name));
                            return Ok(());
                        } else if game.player2 == player2 {
                            handleoutmsg(&msg, format!("{} is already in a game!", player2.name));
                            return Ok(());
                        }
                    }

                    handleoutmsg(&msg, format!(
                        "A new game of tic tac toe has been started between {} and {}!\nUse ~t3 put <position> to choose a space on your turn. These are the positions:",
                        player1.name, player2.name
                    ));

                    handleoutmsg(&msg, format!("{}", TicTTGame::help_grid()));
                    let g = TicTTGame::new(player1, player2);
                    println!("{}", g);
                    ttt_games_mx.lock().unwrap().push(g);
                } else if args.len() == 2 {
                    match command.as_ref() {
                        "put" => {
                            let mut vec_mutex = ttt_games_mx.lock().unwrap();
                            let mut indexes_to_pop: Vec<usize> = Vec::new();

                            for index in 0..vec_mutex.len() {
                                let game = &mut vec_mutex[index];

                                if game.player1.name == msg.author.name || game.player2.name == msg.author.name {
                                    let position = args.single::<String>()?;
                                    let mut target_game: TicTTGame = vec_mutex.remove(index);

                                    match target_game.update_board(position) {
                                        Ok(_) => (),
                                        Err(why) => {
                                            handleoutmsg(&msg, why.to_string());
                                            return Ok(());
                                        },
                                    };

                                    match target_game.state {
                                        Win_Player1 => handleoutmsg(&msg, format!("{} has won!", target_game.player1.name)),
                                        Win_Player2 => handleoutmsg(&msg, format!("{} has won!", target_game.player2.name)),
                                        Cat => handleoutmsg(&msg, String::from("\nYou both lose! Congratulations!")),
                                        _ => {
                                            handleoutmsg(&msg, format!("```\n{}```\nState: {:?}", target_game.as_table(), target_game.state));
                                            vec_mutex.push(target_game);
                                        }
                                        
                                    };

                                    return Ok(());
                                }
                            }

                            handleoutmsg(&msg, String::from("You are not in a game!"));
                        },

                        &_ => handleoutmsg(&msg, String::from("Unknown command!")),
                    };
                }

                Ok(())
            }),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn handleoutmsg(msg: &Message, string: String) {
    match msg.channel_id.say(&string) {
        Ok(_) => println!("Sent message: {}\nIn reply to: [{}] {:?}", string, msg.author.name, msg.content),
        Err(why) => println!("Failed to send message: {}\nError: {}", string, why),
    };
}
