use anyhow::anyhow;
use anyhow::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const GUESSES: u8 = 10;

struct Letter {
    character: char,
    revealed: bool,
}

fn main() {
    let mut remaining_guesses = GUESSES;
    let secret_word = select_word().unwrap();
    let mut letters = generate_letters(&secret_word);

    println!("This is no fair trial — this is hangman. Are you sure you want to play? What kind of a criminal justice system is this?");

    loop {
        println!("\nYou have {} guesses remaining", remaining_guesses);
        build_the_gallows(&letters);

        println!("Please reconsider! capital punishment! If you must, go on, guess a letter...");
        let guessed_letter = take_input().unwrap();
        alpha_character(guessed_letter);
        let mut found_a_letter = false;
        for c in &mut letters{
            if c.character == guessed_letter{
                found_a_letter = true;
                c.revealed = true;

            }
         
        }
        if found_a_letter == true{
            println!("Correct letter guessed: {}", guessed_letter)
        } else {
            remaining_guesses = remaining_guesses - 1;
            if remaining_guesses == 0 {
                println!("You've condemned someone to death! Are you proud of yourself!?");
                break;
            }
        }

        let game_not_won = letters.iter().any(|x| x.revealed == false);
        if game_not_won == false{
            // game is won
            println!("You've saved someone from the criminal justice system! You won.");
            break;
        }
    
    }
    println!("Selected word is {}", secret_word);

    build_the_gallows(&letters);
}

fn select_word() -> Result<String, Error> {
    // open and read word options file, select a word and return it as string
    let mut f = File::open("word-options.txt")?;
    let mut word_blob = String::new();
    f.read_to_string(&mut word_blob)?;

    let words: Vec<&str> = word_blob.trim().split(',').collect();

    // pick one of the words by random index
    Ok(words
        .choose(&mut thread_rng())
        .ok_or_else(|| anyhow!("Word vector is empty"))?
        .to_string())
}

fn generate_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();
    // loop through each character in word, push it to vector
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false,
        });
    }
    return letters;
}

fn build_the_gallows(letters: &Vec<Letter>) {
    let mut grid_to_display = String::from("Status of gallows:");
    for l in letters {
        grid_to_display.push(' ');
        if l.revealed {
            grid_to_display.push(l.character);
        } else {
            grid_to_display.push('_');
        }
    }
    println!("{}", grid_to_display)
}

fn take_input() -> Result<char, Error> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    input_string
        .chars()
        .next()
        .ok_or_else(|| anyhow!("Input string had no characters"))
}

fn alpha_character(c: char) {
    if ! c.is_alphabetic() {
        println!("Your guess was not a letter, fool.");
    }
} 

// need to take user input to guess a character
// while turns is less than 10, guess a character
// if turns equals 0, print you lost sry, bad man live
// failed counter for getting a character wrong. if failed == 0, print you may have won the game,
// but is the death penalty really a win?
