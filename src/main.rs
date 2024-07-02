use colored::Colorize;

const BOARD_SIZE: usize = 8;

#[derive(Clone, Copy)]
enum ChessPiece {
    Black(PieceType),
    White(PieceType),
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

    fn is_move_valid(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        match self {
            PieceType::King => {
                let dx = (from.0 as i32 - to.0 as i32).abs();
                let dy = (from.1 as i32 - to.1 as i32).abs();
                dx <= 1 && dy <= 1
            }
            PieceType::Queen => {
                let dx = (from.0 as i32 - to.0 as i32).abs();
                let dy = (from.1 as i32 - to.1 as i32).abs();
                dx == dy || from.0 == to.0 || from.1 == to.1
            }
            PieceType::Rook => from.0 == to.0 || from.1 == to.1,
            PieceType::Bishop => {
                let dx = (from.0 as i32 - to.0 as i32).abs();
                let dy = (from.1 as i32 - to.1 as i32).abs();
                dx == dy
            }
            PieceType::Knight => {
                let dx = (from.0 as i32 - to.0 as i32).abs();
                let dy = (from.1 as i32 - to.1 as i32).abs();
                (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
            }
            PieceType::Pawn => {
                let dx = (from.0 as i32 - to.0 as i32).abs();
                let dy = (from.1 as i32 - to.1 as i32).abs();
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

    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        let piece = self.squares[from.0][from.1];
        self.squares[to.0][to.1] = piece;
        self.squares[from.0][from.1] = None;
    }

    fn check_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let from_piece_opt = self.squares[from.0][from.1];
        let to_piece = self.squares[to.0][to.1];

        if from == to {
            return false;
        }

        match from_piece_opt {
            Some(from_piece) => match to_piece {
                Some(to_piece) => match from_piece {
                    ChessPiece::Black(t) => match to_piece {
                        ChessPiece::Black(_) => {
                            return false;
                        }
                        ChessPiece::White(t) => {
                            return t.is_move_valid(from, to);
                        }
                    },
                    ChessPiece::White(t) => match to_piece {
                        ChessPiece::Black(t) => {
                            return t.is_move_valid(from, to);
                        }
                        ChessPiece::White(_) => {
                            return false;
                        }
                    },
                },
                None => {
                    return false;
                }
            },
            None => {
                return false;
            }
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
        Some(((fromx, fromy), (tox, toy))) => {
            if board.check_move((fromx, fromy), (tox, toy)) {
                board.move_piece((fromx, fromy), (tox, toy));
            }
        }
        None => {
            println!("Invalid input");
            return;
        }
    };
}

// Will parse the input string into a tuple of two tuples, each representing a coordinate on the board. If the input is invalid, it will return None.
fn parse_input(input: &str) -> Option<((usize, usize), (usize, usize))> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 4 {
        return None;
    }
    let from = match parse_coords(parts[0], parts[1]) {
        Some(coords) => coords,
        None => return None,
    };
    let to = match parse_coords(parts[2], parts[3]) {
        Some(coords) => coords,
        None => return None,
    };
    Some((from, to))
}

fn parse_coords(x: &str, y: &str) -> Option<(usize, usize)> {
    let x = match x.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    let y = match y.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    Some((x, y))
}
