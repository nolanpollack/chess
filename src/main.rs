use colored::Colorize;

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
}

struct ChessBoard {
    squares: [[Option<ChessPiece>; 8]; 8],
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
                [Some(ChessPiece::Black(PieceType::Pawn)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(ChessPiece::White(PieceType::Pawn)); 8],
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

        match from_piece_opt {
            Some(fromPiece) => match to_piece {
                Some(toPiece) => {
                    if fromPiece == toPiece {
                        return false;
                    }
                }
                None => {}
            },
            None => {
                return false;
            }
        }

        if from_piece_opt.is_none() {
            return false;
        }
        if from_piece_opt.is_some() && to_piece.is_some() {}

        match from_piece_opt {
            Some(ChessPiece::Black(PieceType::King)) => self.check_king(from, to),
            Some(ChessPiece::Black(PieceType::Queen)) => self.check_queen(from, to),
            Some(ChessPiece::Black(PieceType::Rook)) => self.check_rook(from, to),
            Some(ChessPiece::Black(PieceType::Bishop)) => self.check_bishop(from, to),
            Some(ChessPiece::Black(PieceType::Knight)) => self.check_knight(from, to),
            Some(ChessPiece::Black(PieceType::Pawn)) => self.check_pawn(from, to),
            _ => false,
        }

        true
    }
}

fn main() {
    let mut board = ChessBoard::new();
    board.print();
    loop {
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("Error reading input");
            continue;
        }
        match parse_input(&input) {
            Some(((fromx, fromy), (tox, toy))) => {
                println!(
                    "Legal move: {}",
                    board.check_move((fromx, fromy), (tox, toy))
                );
                if board.check_move((fromx, fromy), (tox, toy)) {
                    board.move_piece((fromx, fromy), (tox, toy));
                    board.print();
                }
            }
            None => {
                println!("Invalid input");
                continue;
            }
        };
    }
}

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
