mod chess_board;
mod coordinate;

use chess_board::chess_board::ChessBoard;
use coordinate::Coordinate;
use std::num::ParseIntError;

const BOARD_SIZE: usize = 8;

fn main() {
    let mut board = ChessBoard::new();
    loop {
        println!("{board}");
        take_turn(&mut board);
    }
}

// Will take in user input, parse it, and then check if the move is legal. If so, it will move the piece.
fn take_turn(board: &mut ChessBoard) {
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        println!("Error reading input");
        return;
    }

    match parse_input(&input) {
        Ok((from, to)) => {
            if board.check_move(from, to).is_none() {
                board.move_piece(from, to);
            }
        }
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
}

enum MoveError {
    InvalidMove,
    InvalidInput,
}

// Will parse the input string into a tuple of two tuples, each representing a coordinate on the board. If the input is invalid, it will return Error.
fn parse_input(input: &str) -> Result<(Coordinate, Coordinate), MoveError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 4 {
        return Err(MoveError::InvalidInput);
    }
    let from = match parse_coords(parts[0], parts[1]) {
        Ok(coords) => coords,
        Err(_) => return Err(MoveError::InvalidInput),
    };
    let to = match parse_coords(parts[2], parts[3]) {
        Ok(coords) => coords,
        Err(_) => return Err(MoveError::InvalidInput),
    };
    Ok((from, to))
}

fn parse_coords(x: &str, y: &str) -> Result<Coordinate, ParseIntError> {
    let x = x.parse::<usize>()?;
    let y = y.parse::<usize>()?;
    Ok(Coordinate::new(x, y))
}
