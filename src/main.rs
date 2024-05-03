use::std::*;

pub struct GameBoard {
    board: [[char; 3]; 3],
    rows: usize,
    cols: usize,
    round: u8,
}

impl GameBoard{

    pub fn render(&self) {
        // display the state of the board
    }

    pub fn play(&mut self, player: usize, col: usize, row: usize) {
        // place a mark in the spot specified
        // player 1: X
        // player 2: O
        
    }
}

fn main() {
    println!("Hello, world!");
}
