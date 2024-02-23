// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn curr_word(indices: &Vec<bool>, tgt_word_chars:&Vec<char>, word_len: usize) -> String {
    let mut word = String::new();
    let mut idx: usize = 0;
    while idx < word_len {
        if indices[idx] {
            word.push(tgt_word_chars[idx]);
        } else {
            word.push('-');
        }
        idx = idx + 1;
    }
    word
}

fn guess_char_match(indices_res: &Vec<bool>, tgt_word_chars: &Vec<char>, word_len: usize, ch: char) -> (usize, bool) {
    let mut idx = 0;
    let mut res = false;
    while idx < word_len {
        if !indices_res[idx] && ch == tgt_word_chars[idx] {
            res = true;
            break;
        }
        idx = idx + 1;
    }
    (idx, res)
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    //println!("random word: {}", secret_word);
    println!("Welcome to CS110L Hangman!");
    let word_len = secret_word_chars.len();
    let mut valid_indices = vec![false; word_len];
    let mut n_valid = 0;
    //let mut guess_char = Vec::new();
    // Your code here! :)
    while n_valid < word_len {
        println!("The word so far is {}", curr_word(&valid_indices, &secret_word_chars, word_len));
        println!("You have {} guesses left", word_len - n_valid);
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line");
        if let (idx, true) = guess_char_match(&valid_indices, &secret_word_chars, word_len, guess.chars().nth(0).unwrap()) {
            n_valid += 1;
            valid_indices[idx] = true;
        } else {
            println!("Sorry, that letter is not in the word");
        }
    }
}
