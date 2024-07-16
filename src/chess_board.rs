use std::fmt::Display;

use crate::coordinate::Coordinate;
use colored::Colorize;

use crate::BOARD_SIZE;

pub mod chess_board {
    use super::*;
    pub struct ChessBoard {
        squares: [[Option<ChessPiece>; BOARD_SIZE]; BOARD_SIZE],
    }

    impl Display for ChessBoard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut result = String::new();
            result.push_str("  0 1 2 3 4 5 6 7\n");
            for (i, row) in self.squares.iter().enumerate() {
                result.push_str(&format!("{} ", i));
                for square in row.iter() {
                    match square {
                        Some(piece) => match piece {
                            ChessPiece::Black(t) => {
                                result.push_str(&format!("{} ", String::from(t.icon()).green()))
                            }
                            ChessPiece::White(t) => result.push_str(&format!("{} ", t.icon())),
                        },
                        None => result.push_str(". "),
                    }
                }
                result.push_str("\n");
            }
            write!(f, "{}", result)
        }
    }

    impl ChessBoard {
        pub fn new() -> ChessBoard {
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

        pub fn get_piece(&self, coord: Coordinate) -> Option<ChessPiece> {
            self.squares[coord.y][coord.x]
        }

        fn set_piece(&mut self, coord: Coordinate, piece: Option<ChessPiece>) {
            self.squares[coord.y][coord.x] = piece;
        }

        pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) {
            let piece = self.get_piece(from).unwrap();
            self.set_piece(to, Some(piece));
            self.set_piece(from, None);
        }

        // Returns None if the move is valid, otherwise returns the reason why the move is invalid
        pub fn check_move(&self, from: Coordinate, to: Coordinate) -> Option<MoveFailure> {
            let from_piece = self.get_piece(from);
            let to_piece = self.get_piece(to);

            if from == to {
                return Some(MoveFailure::MoveToSameSpot);
            }

            let from_piece = match from_piece {
                Some(from_piece) => from_piece,
                None => {
                    return Some(MoveFailure::NoPieceAtFrom);
                }
            };

            to_piece?.can_take(from_piece, from, to)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChessPiece {
    Black(PieceType),
    White(PieceType),
}

impl ChessPiece {
    fn can_take(&self, other: ChessPiece, from: Coordinate, to: Coordinate) -> Option<MoveFailure> {
        match self {
            ChessPiece::Black(t) => match other {
                ChessPiece::Black(_) => Some(MoveFailure::SameColorTake),
                ChessPiece::White(_) => t.is_move_valid(from, to),
            },
            ChessPiece::White(t) => match other {
                ChessPiece::Black(_) => t.is_move_valid(from, to),
                ChessPiece::White(_) => Some(MoveFailure::SameColorTake),
            },
        }
    }

    fn piece_type(&self) -> PieceType {
        match self {
            ChessPiece::Black(t) => *t,
            ChessPiece::White(t) => *t,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PieceType {
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
    fn is_move_valid(&self, from: Coordinate, to: Coordinate) -> Option<MoveFailure> {
        let x_move = to.x as i32 - from.x as i32;
        let y_move = to.y as i32 - from.y as i32;
        match self {
            PieceType::King => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                if dx <= 1 && dy <= 1 {
                    None
                } else {
                    Some(MoveFailure::IllegalKingMove)
                }
            }
            PieceType::Queen => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                if dx == dy || from.x == to.x || from.y == to.y {
                    None
                } else {
                    Some(MoveFailure::IllegalQueenMove)
                }
            }
            PieceType::Rook => {
                if from.x == to.x || from.y == to.y {
                    None
                } else {
                    Some(MoveFailure::IllegalRookMove)
                }
            }
            PieceType::Bishop => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                if dx == dy {
                    None
                } else {
                    Some(MoveFailure::IllegalBishopMove)
                }
            }
            PieceType::Knight => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
                    None
                } else {
                    Some(MoveFailure::IllegalKnightMove)
                }
            }
            PieceType::Pawn => {
                let dx = x_move.abs();
                let dy = y_move.abs();
                if dx == 0 && dy == 1 {
                    None
                } else {
                    Some(MoveFailure::IllegalPawnMove)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MoveFailure {
    MoveToSameSpot,
    NoPieceAtFrom,
    SameColorTake,
    IllegalKingMove,
    IllegalQueenMove,
    IllegalRookMove,
    IllegalBishopMove,
    IllegalKnightMove,
    IllegalPawnMove,
}

#[cfg(test)]
mod tests {
    use chess_board::ChessBoard;

    use super::*;

    mod chess_board_tests {
        #[test]
        fn test_board_initialization() {
            use super::*;
            let board = ChessBoard::new();
            let from_piece = board.get_piece(Coordinate::new(0, 1));
            let to_piece = board.get_piece(Coordinate::new(0, 2));

            assert!(from_piece.is_some());
            assert!(to_piece.is_none());
        }
    }

    #[test]
    fn test_move_no_piece() {
        let board = ChessBoard::new();
        let from = Coordinate::new(2, 2);
        let to = Coordinate::new(0, 1);

        assert!(board.get_piece(from).is_none());
        assert_eq!(board.check_move(from, to), Some(MoveFailure::NoPieceAtFrom));
    }

    #[test]
    fn test_move_same_spot() {
        let board = ChessBoard::new();
        let from = Coordinate::new(0, 1);
        let to = Coordinate::new(0, 1);

        assert!(board.get_piece(from).is_some());
        assert_eq!(
            board.check_move(from, to),
            Some(MoveFailure::MoveToSameSpot)
        );
    }

    #[test]
    fn test_pawn_legal_move() {
        let board = ChessBoard::new();
        let from = Coordinate::new(0, 1);
        let to = Coordinate::new(0, 2);

        assert!(board.get_piece(from).unwrap().piece_type() == PieceType::Pawn);
        assert_eq!(board.check_move(from, to), None, "{}", board);
    }
    #[test]
    fn test_pawn_illegal_moves() {
        let board = ChessBoard::new();
        let from = Coordinate::new(0, 1);
        let to = Coordinate::new(0, 4);

        assert!(board.get_piece(from).unwrap().piece_type() == PieceType::Pawn);
        assert_eq!(
            board.check_move(from, to),
            Some(MoveFailure::IllegalPawnMove)
        );
    }
}
