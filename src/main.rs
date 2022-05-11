use std::process;
use text_io::read;

struct TicTacToe {
    board: [char; 9],
    number_of_moves: u16,
    move_of: char,
    input_check: bool,
    is_game_over: bool,
}
impl TicTacToe {
    pub fn check_winner(&self) -> String {
        //horizantal
        if self.board[0] == self.board[1] && self.board[2] == self.board[1] && self.board[0] != '-'
        {
            if self.board[0] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        } else if self.board[3] == self.board[4] && self.board[5] == self.board[4]&& self.board[3] != '-'
        {
            if self.board[3] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        } else if self.board[6] == self.board[7] && self.board[7] == self.board[8]&& self.board[6] != '-'
        {
            if self.board[6] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        }
        //vertical
        else if self.board[0] == self.board[3] && self.board[6] == self.board[1] && self.board[0] != '-'
        {
            if self.board[0] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        } else if self.board[1] == self.board[4] && self.board[7] == self.board[1] && self.board[1] != '-'
        {
            if self.board[1] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        } else if self.board[2] == self.board[5] && self.board[8] == self.board[1] && self.board[2] != '-'
        {
            if self.board[2] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        }
        //diagonal
        else if self.board[0] == self.board[4] && self.board[8] == self.board[1] && self.board[0] != '-'
        {
            if self.board[0] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        } else if self.board[2] == self.board[4] && self.board[8] == self.board[1] && self.board[2] != '-'
        {
            if self.board[2] == 'X'{
                return "X".to_string();
            }
            else{
                return "O".to_string()
            }
        }
        else{
            return "NONE".to_string();
        }


    }

    pub fn check_tie(&self) -> bool {
        if self.number_of_moves == 9 {
            return true;
        } else {
            return false;
        };
    }

    pub fn increment_moves(&mut self) {
        println!("number of moves = {}", self.number_of_moves);
        self.number_of_moves += 1;
    }

    pub fn print_board(&self) {
        println!(" {} | {} | {}", self.board[0], self.board[1], self.board[2]);
        println!("----------");
        println!(" {} | {} | {}", self.board[3], self.board[4], self.board[5]);
        println!("----------");
        println!(" {} | {} | {}", self.board[6], self.board[7], self.board[8]);
    }

    pub fn take_input(&mut self) {
        self.check_winner();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        self.input_check = false;
        let user_index: i32 = read!();
        let game_index = user_index - 1;
        if self.board[game_index as usize] != '-' {
            println!("Stop cheating dude...");
            process::exit(1);
        } else {
            self.board[game_index as usize] = self.move_of;
            if self.move_of == 'X' {
                self.move_of = 'O';
            } else {
                self.move_of = 'X';
            }
            self.input_check = true;
            self.check_winner();
            return;
        }
    }
}

fn main() {
    let mut ttt = TicTacToe {
        number_of_moves: 1,
        move_of: 'X',
        board: ['-', '-', '-', '-', '-', '-', '-', '-', '-'],
        is_game_over: false,
        input_check: false,
    };

    while 69 > 1 {
        ttt.print_board();
        ttt.take_input();
        if ttt.check_winner() != "NONE" {
            println!("Winner is {}", ttt.check_winner());
            ttt.print_board();
            return;
        } else if ttt.check_tie() {
            ttt.is_game_over = true;
            println!("Game Ends in a tie");
            return;
        }
        ttt.increment_moves();
    }
}
