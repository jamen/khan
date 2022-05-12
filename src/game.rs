use std::mem;

use bitflags::bitflags;
use vampirc_uci::{UciPiece, UciSquare};

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalMoveError;

#[derive(Default, Debug)]
pub struct Position {
    pieces: [Option<Square>; 32],
    promotions: [Option<Piece>; 16],
    position_status: PositionStatus,
    en_passant: Option<Square>,
    reversible_moves: u16,
}

impl Position {
    pub fn starting() -> Self {
        use Square::*;
        Position {
            #[rustfmt::skip]
            pieces: [
                Some(A2), Some(B2), Some(C2), Some(D2), Some(E2), Some(F2), Some(G2), Some(H2),
                Some(A7), Some(B7), Some(C7), Some(D7), Some(E7), Some(F7), Some(G7), Some(H7),
                Some(A1), Some(B1), Some(C1), Some(D1), Some(E1), Some(F1), Some(G1), Some(H1),
                Some(A8), Some(B8), Some(C8), Some(D8), Some(E8), Some(F8), Some(G8), Some(H8),
            ],
            promotions: Default::default(),
            position_status: Default::default(),
            en_passant: None,
            reversible_moves: 0,
        }
    }

    pub fn halfmove(
        &mut self,
        from: Square,
        to: Square,
        promotion: Option<Piece>,
    ) -> Result<(), IllegalMoveError> {
        for (i, piece_square) in self.pieces.iter_mut().enumerate() {
            if *piece_square == Some(from) {
                *piece_square = Some(to);

                if let Some(promotion) = promotion {
                    let to_rank = to.rank();

                    if ((..8).contains(&i) && to_rank == 7)
                        || ((8..16).contains(&i) && to_rank == 0)
                    {
                        self.promotions[i] = Some(promotion);
                        break;
                    } else {
                        return Err(IllegalMoveError);
                    }
                }
            }
        }

        for piece_square in self.pieces.iter_mut() {
            if *piece_square == Some(to) {
                *piece_square = None;
            }
        }

        self.position_status ^= PositionStatus::HALFMOVE;

        Ok(())
    }
}

bitflags! {
    #[derive(Default)]
    pub struct PositionStatus: u8 {
        const HALFMOVE = 0b00001;
        const NO_WHITE_KINGSIDE_CASTLE = 0b00010;
        const NO_WHITE_QUEENSIDE_CASTLE = 0b00100;
        const NO_BLACK_KINGSIDE_CASTLE = 0b01000;
        const NO_BLACK_QUEENSIDE_CASTLE = 0b10000;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl Square {
    // New square from zero-indexed rank and file
    #[inline]
    fn new(rank: u8, file: u8) -> Option<Self> {
        let rank = if rank <= 7 { rank } else { return None };
        let file = if file <= 7 { file } else { return None };
        let index = rank * 8 + file;
        Some(
            // SAFETY: Checked above
            unsafe { mem::transmute::<u8, Square>(index) },
        )
    }

    // Zero-indexed rank
    #[inline]
    fn rank(self) -> u8 {
        8 - self as u8 / 8
    }

    // Zero-indexed file
    #[inline]
    fn file(self) -> u8 {
        self as u8 % 8
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TryIntoSquareError;

impl TryFrom<UciSquare> for Square {
    type Error = TryIntoSquareError;

    fn try_from(x: UciSquare) -> Result<Self, Self::Error> {
        let file = (x.file as u32).wrapping_sub(97) as u8;
        let rank = x.rank as u8;
        Square::new(file, rank).ok_or(TryIntoSquareError)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<UciPiece> for Piece {
    fn from(x: UciPiece) -> Self {
        match x {
            UciPiece::Pawn => Piece::Pawn,
            UciPiece::Knight => Piece::Knight,
            UciPiece::Bishop => Piece::Bishop,
            UciPiece::Rook => Piece::Rook,
            UciPiece::Queen => Piece::Queen,
            UciPiece::King => Piece::King,
        }
    }
}
