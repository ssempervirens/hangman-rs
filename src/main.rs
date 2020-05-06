use rand::Rng;

use std::fs::File;
use std::io::prelude::*;
use anyhow::Error;


fn main() {
    let secret_word = select_word().unwrap();
    println!("Selected word is {}", secret_word);
}

fn select_word() -> Result<String, Error> {
    // open and read word options file, select a word and return it as string
    let mut f = File::open("word-options.txt")?;
    let mut word_blob = String::new();
        f.read_to_string(& mut word_blob)?;

    let words: Vec<&str> = word_blob.trim().split(',').collect();

    // pick one of the words by random index
    let rand_index = rand::thread_rng().gen_range(0, words.len());

    Ok(words[rand_index].into())
}

// need to take user input to guess a character
// while turns is less than 10, guess a character
// if turns equals 0, print you lost sry, bad man live
// failed counter for getting a character wrong. if failed == 0, print you may have won the game,
// but is the death penalty really a win?

