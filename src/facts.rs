use rand::prelude::*;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn check() -> Result<(), &'static str> {
    if !Path::new("./facts/").exists() {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            println!("Path in .: {}", path.unwrap().path().display());
        }
        return Err("Facts dir does not exist");
    }

    Ok(())
}

pub fn get_fact(character: String) -> Result<String, &'static str> {
    let fact_dir = format!("./facts/{}.facts", character);

    if Path::new(&fact_dir).exists() {
        let file = match OpenOptions::new().read(true).open(&fact_dir) {
            Ok(file) => file,
            Err(error) => {
                println!("Path: {}", &fact_dir);
                println!("Error: {:?}", error);
                return Ok(String::from("We\'re closed! (File open error)"));
            }
        };

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        match buf_reader.read_to_string(&mut contents) {
            Ok(size) => println!("Read a buf ({}) of size {}", fact_dir, size),
            Err(why) => return Ok(String::from("I don\'t know how to read! (File read error)")),
        }

        let contents_vector: Vec<&str> = contents.split('\n').collect();
        let fact_num: u32 = rand::thread_rng().gen_range(0, contents_vector.len() as u32 - 1);

        for entry in &contents_vector {
            println!("{}", entry);
        }

        return Ok(String::from(contents_vector[fact_num as usize]));
    } else {
        return Ok(String::from("I don\'t know who that is! :crab:"));
    }
}
