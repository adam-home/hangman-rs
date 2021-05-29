extern crate ncurses;

use std::collections::BTreeSet;
use std::ascii::AsciiExt;
use self::ncurses::*;

mod dict;
mod display;

fn make_guess(guesses: &mut BTreeSet<char>) {
    loop {
        mvprintw(16, 0, "Your guess: ");
        let mut letter = getch() as u8 as char;

        if !letter.is_alphabetic() {
            continue;
        }

        letter = letter.to_ascii_lowercase();

        if !guesses.contains(&letter) {
            guesses.insert(letter);
            break;
        }
    }
}

/*
 * If all letters in word are in guesses, then the word has been
 * guessed.
 *
 * Get the letters that are in guesses that are not in the word =>
 * incorrect_letters
 *
 * If the length of incorrect_letters > max_incorrect_guesses, the
 * player has lost.
 */
enum GuessResult {
    AllGuessed,
    GameOver,
    GuessAgain(u32)
}

fn is_word_guessed(word: &String, guesses: &BTreeSet<char>) -> GuessResult {
    let mut correctly_guessed = true;
    for letter in word.chars() {
        if !guesses.contains(&letter) {
            correctly_guessed = false;
            break;
        }
    }
    if correctly_guessed {
        return GuessResult::AllGuessed;
    }

    let mut incorrect_letters = 0;
    for guess in guesses {
        if !word.chars().any(|c| c == *guess) {
            incorrect_letters += 1;
        }
    }

    if incorrect_letters >= 10 {
        return GuessResult::GameOver;
    }

    return GuessResult::GuessAgain(incorrect_letters);
}

fn main() {
    initscr();

    mvprintw(0, 0, "Hangman!");

    let word = dict::choose_word();
    let mut guesses = BTreeSet::new();
    let mut wrong = 0;

    loop {
        display::display_status(&word, &guesses, wrong);

        // Ask the player to make a guess
        make_guess(&mut guesses);

        match is_word_guessed(&word, &guesses) {
            GuessResult::AllGuessed => {
                mvprintw(18, 0, "You won!\n");
                display::display_status(&word, &guesses, wrong);
                break;
            },
            GuessResult::GameOver => {
                mvprintw(18, 0, "You lost!\n");
                mvprintw(19, 0, &*format!("The word was {}\n", word));
                display::display_status(&word, &guesses, wrong+1);
                break;
            },
            GuessResult::GuessAgain(incorrect_count) => {
                wrong = incorrect_count;
            }
        }
    };

    mvprintw(20, 0, "Press a key");
    getch();

    endwin();
}
