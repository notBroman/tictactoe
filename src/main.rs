use::std::*;

pub struct GameBoard {
    board: [[char; 3]; 3],
    rows: usize,
    cols: usize,
    round: u8,
}

impl GameBoard{

    pub fn init(&mut self) {
        // initialise the board
        self.rows = 3;
        self.cols = 3;
        self.round = 0;

        for i in 0..self.rows{
            for j in 0..self.cols{
                self.board[i][j] = ' ';
            }
        }
    }

    fn render(&self) {
        // display the state of the board
        for i in 0..self.rows{
            print!("+-+-+-+\n");
            print!("|{}|{}|{}|\n", self.board[i][0], self.board[i][1], self.board[i][2]);
        }
        print!("+-+-+-+\n");
    }

    fn play(&mut self, player: usize, c: usize, r: usize) -> bool {
        // place a mark in the spot specified
        // player 1: X
        // player 2: O
        let symbol;
        if player == 1 {
            symbol = 'X';
        } else {
            symbol = 'O';
        }
        
        self.board[r][c] = symbol;
        false
    }

    pub fn game_loop(&mut self){
        self.render();
        let mut winner: bool = false;
        while self.round < 9 && !winner{
            
            self.round += 1;
        }

    }
}

fn setup_board(board: &mut GameBoard) {

}

fn main() {
    let mut game = GameBoard {
        board: [[ ' '; 3]; 3],
        rows: 3,
        cols: 3,
        round: 0,
    };
    println!("starting the game!\n\n");
    
    game.game_loop();
}
