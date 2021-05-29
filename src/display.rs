extern crate ncurses;

use std::char;
use std::collections::BTreeSet;
use self::ncurses::*;

pub fn display_status(word: &String, guesses: &BTreeSet<char>, incorrect: u32) {
    mvprintw(12, 0, "Guesses: ");
    for letter in guesses {
        printw(&*format!("{}", letter));
    }

    mvprintw(14, 0, "Word: ");
    for letter in word.chars() {
        if guesses.contains(&letter) {
            printw(&*format!("{}", letter));
        }
        else {
            printw("_");
        }
    }

    draw_victim(incorrect);

    refresh();
}

//    0123456
//  0 +----+
//  1 |/   |
//  2 |    O
//  3 |   /|\
//  4 |    |
//  5 |   / \
//  6 |
//  7 |\
//  8 +-----
pub fn draw_victim(fragments: u32) {
    if fragments > 0 {
        mvprintw(8, 0, "|\n|\\\n+-----");
    }
    if fragments > 1 {
        mvprintw(2, 0, "+\n|\n|\n|\n|\n|");
    }
    if fragments > 2 {
        mvprintw(2, 1, "----+");
        mvprintw(3, 1, "/");
    }
    if fragments > 3 {
        mvprintw(3, 5, "|");
    }
    if fragments > 4 {
        mvprintw(4, 5, "O");
    }
    if fragments > 5 {
        mvprintw(5, 5, "|");
        mvprintw(6, 5, "|");
    }
    if fragments > 6 {
        mvprintw(5, 4, "/");
    }
    if fragments > 7 {
        mvprintw(5, 6, "\\");
    }
    if fragments > 8 {
        mvprintw(7, 4, "/");
    }
    if fragments > 9 {
        mvprintw(7, 6, "\\");
    }
}
