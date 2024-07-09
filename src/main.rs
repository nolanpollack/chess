use std::num::ParseIntError;

use colored::Colorize;

const BOARD_SIZE: usize = 8;

#[derive(Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            panic!("Coordinate must be within the bounds of the board");
        }
        Coordinate { x, y }
    }
}

#[derive(Clone, Copy)]
enum ChessPiece {
    Black(PieceType),
    White(PieceType),
}

impl ChessPiece {
    fn can_take(&self, other: ChessPiece, from: Coordinate, to: Coordinate) -> bool {
        match self {
            ChessPiece::Black(t) => match other {
                ChessPiece::Black(_) => false,
                ChessPiece::White(_) => t.is_move_valid(from, to),
            },
            ChessPiece::White(t) => match other {
                ChessPiece::Black(_) => t.is_move_valid(from, to),
                ChessPiece::White(_) => false,
            },
        }
    }
}

#[derive(Clone, Copy)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    fn icon(&self) -> char {
        match self {
            PieceType::King => '♔',
            PieceType::Queen => '♕',
            PieceType::Rook => '♖',
            PieceType::Bishop => '♗',
            PieceType::Knight => '♘',
            PieceType::Pawn => '♙',
        }
    }

    // Assumes that the piece is not moving to its own spot
    fn is_move_valid(&self, from: Coordinate, to: Coordinate) -> bool {
        let x_move = from.x as i32 - to.x as i32;
        let y_move = from.y as i32 - to.y as i32;
        match self {
            PieceType::King => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                dx <= 1 && dy <= 1
            }
            PieceType::Queen => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                dx == dy || from.x == to.x || from.y == to.y
            }
            PieceType::Rook => from.x == to.x || from.y == to.y,
            PieceType::Bishop => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                dx == dy
            }
            PieceType::Knight => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
            }
            PieceType::Pawn => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                dx == 0 && dy == 1
            }
        }
    }
}

struct ChessBoard {
    squares: [[Option<ChessPiece>; BOARD_SIZE]; BOARD_SIZE],
}

impl ChessBoard {
    fn new() -> ChessBoard {
        ChessBoard {
            squares: [
                [
                    Some(ChessPiece::Black(PieceType::Rook)),
                    Some(ChessPiece::Black(PieceType::Knight)),
                    Some(ChessPiece::Black(PieceType::Bishop)),
                    Some(ChessPiece::Black(PieceType::Queen)),
                    Some(ChessPiece::Black(PieceType::King)),
                    Some(ChessPiece::Black(PieceType::Bishop)),
                    Some(ChessPiece::Black(PieceType::Knight)),
                    Some(ChessPiece::Black(PieceType::Rook)),
                ],
                [Some(ChessPiece::Black(PieceType::Pawn)); BOARD_SIZE],
                [None; BOARD_SIZE],
                [None; BOARD_SIZE],
                [None; BOARD_SIZE],
                [None; BOARD_SIZE],
                [Some(ChessPiece::White(PieceType::Pawn)); BOARD_SIZE],
                [
                    Some(ChessPiece::White(PieceType::Rook)),
                    Some(ChessPiece::White(PieceType::Knight)),
                    Some(ChessPiece::White(PieceType::Bishop)),
                    Some(ChessPiece::White(PieceType::Queen)),
                    Some(ChessPiece::White(PieceType::King)),
                    Some(ChessPiece::White(PieceType::Bishop)),
                    Some(ChessPiece::White(PieceType::Knight)),
                    Some(ChessPiece::White(PieceType::Rook)),
                ],
            ],
        }
    }

    fn print(&self) {
        println!("  0 1 2 3 4 5 6 7");
        for (i, row) in self.squares.iter().enumerate() {
            print!("{} ", i);
            for square in row.iter() {
                match square {
                    Some(piece) => match piece {
                        ChessPiece::Black(t) => print!("{} ", String::from(t.icon()).green()),
                        ChessPiece::White(t) => print!("{} ", t.icon()),
                    },
                    None => print!(". "),
                }
            }
            println!();
        }
    }

    fn move_piece(&mut self, from: Coordinate, to: Coordinate) {
        let piece = self.squares[from.x][from.y];
        self.squares[to.x][to.y] = piece;
        self.squares[from.x][from.y] = None;
    }

    fn check_move(&self, from: Coordinate, to: Coordinate) -> bool {
        let from_piece = self.squares[from.x][from.y];
        let to_piece = self.squares[to.x][to.y];

        if from == to {
            return false;
        }

        let from_piece = match from_piece {
            Some(from_piece) => from_piece,
            None => {
                return false;
            }
        };

        match to_piece {
            Some(to_piece) => to_piece.can_take(from_piece, from, to),
            None => true,
        }
    }
}

fn main() {
    let mut board = ChessBoard::new();
    loop {
        take_turn(&mut board);
        board.print();
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
            if board.check_move(from, to) {
                board.move_piece(from, to);
            }
        }
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
}

// Will parse the input string into a tuple of two tuples, each representing a coordinate on the board. If the input is invalid, it will return Error.
fn parse_input(input: &str) -> Result<(Coordinate, Coordinate), ParseIntError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 4 {
        //TODO: Return some error;
    }
    let from = parse_coords(parts[0], parts[1])?;
    let to = parse_coords(parts[2], parts[3])?;
    Ok((from, to))
}

fn parse_coords(x: &str, y: &str) -> Result<Coordinate, ParseIntError> {
    let x = x.parse::<usize>()?;
    let y = y.parse::<usize>()?;
    Ok(Coordinate::new(x, y))
}
