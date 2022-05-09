use bitflags::bitflags;

use uci::UciPiece;
use vampirc_uci::{self as uci, UciSquare};

#[derive(Debug, PartialEq, Eq)]
pub enum PositionError {
    InvalidMove,
}

#[derive(Default, Debug)]
pub struct Position {
    pieces: [Option<Square>; 32],
    promotions: [Option<Promotion>; 16],
    lost_castles: LostCastles,
    enpassant: Option<Square>,
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
            lost_castles: Default::default(),
            enpassant: None,
        }
    }
    pub fn ply(
        &mut self,
        from: Square,
        to: Square,
        promotion: Option<Promotion>,
    ) -> Result<(), PositionError> {
        for (i, piece_square) in self.pieces.iter_mut().enumerate() {
            println!("info string Helllooo");
            if *piece_square == Some(from) {
                *piece_square = Some(to);
                if let Some(promotion) = promotion {
                    if i <= 7 {
                        self.promotions[i] = Some(promotion);
                        break;
                    } else {
                        return Err(PositionError::InvalidMove);
                    }
                }
            }
        }
        for piece_square in self.pieces.iter_mut() {
            if *piece_square == Some(to) {
                *piece_square = None;
            }
        }
        Ok(())
    }
}

bitflags! {
    #[derive(Default)]
    pub struct LostCastles: u8 {
        const WHITE_KINGSIDE = 0b0001;
        const WHITE_QUEENSIDE = 0b0010;
        const BLACK_KINGSIDE = 0b0100;
        const BLACK_QUEENSIDE = 0b1000;
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

#[derive(Debug, PartialEq, Eq)]
pub struct TryIntoSquareError;

impl TryFrom<UciSquare> for Square {
    type Error = TryIntoSquareError;

    fn try_from(x: UciSquare) -> Result<Self, Self::Error> {
        use Square::*;
        Ok(match x {
            UciSquare { file: 'a', rank: 1 } => A1,
            UciSquare { file: 'b', rank: 1 } => B1,
            UciSquare { file: 'c', rank: 1 } => C1,
            UciSquare { file: 'd', rank: 1 } => D1,
            UciSquare { file: 'e', rank: 1 } => E1,
            UciSquare { file: 'f', rank: 1 } => F1,
            UciSquare { file: 'g', rank: 1 } => G1,
            UciSquare { file: 'h', rank: 1 } => H1,
            UciSquare { file: 'a', rank: 2 } => A2,
            UciSquare { file: 'b', rank: 2 } => B2,
            UciSquare { file: 'c', rank: 2 } => C2,
            UciSquare { file: 'd', rank: 2 } => D2,
            UciSquare { file: 'e', rank: 2 } => E2,
            UciSquare { file: 'f', rank: 2 } => F2,
            UciSquare { file: 'g', rank: 2 } => G2,
            UciSquare { file: 'h', rank: 2 } => H2,
            UciSquare { file: 'a', rank: 3 } => A3,
            UciSquare { file: 'b', rank: 3 } => B3,
            UciSquare { file: 'c', rank: 3 } => C3,
            UciSquare { file: 'd', rank: 3 } => D3,
            UciSquare { file: 'e', rank: 3 } => E3,
            UciSquare { file: 'f', rank: 3 } => F3,
            UciSquare { file: 'g', rank: 3 } => G3,
            UciSquare { file: 'h', rank: 3 } => H3,
            UciSquare { file: 'a', rank: 4 } => A4,
            UciSquare { file: 'b', rank: 4 } => B4,
            UciSquare { file: 'c', rank: 4 } => C4,
            UciSquare { file: 'd', rank: 4 } => D4,
            UciSquare { file: 'e', rank: 4 } => E4,
            UciSquare { file: 'f', rank: 4 } => F4,
            UciSquare { file: 'g', rank: 4 } => G4,
            UciSquare { file: 'h', rank: 4 } => H4,
            UciSquare { file: 'a', rank: 5 } => A5,
            UciSquare { file: 'b', rank: 5 } => B5,
            UciSquare { file: 'c', rank: 5 } => C5,
            UciSquare { file: 'd', rank: 5 } => D5,
            UciSquare { file: 'e', rank: 5 } => E5,
            UciSquare { file: 'f', rank: 5 } => F5,
            UciSquare { file: 'g', rank: 5 } => G5,
            UciSquare { file: 'h', rank: 5 } => H5,
            UciSquare { file: 'a', rank: 6 } => A6,
            UciSquare { file: 'b', rank: 6 } => B6,
            UciSquare { file: 'c', rank: 6 } => C6,
            UciSquare { file: 'd', rank: 6 } => D6,
            UciSquare { file: 'e', rank: 6 } => E6,
            UciSquare { file: 'f', rank: 6 } => F6,
            UciSquare { file: 'g', rank: 6 } => G6,
            UciSquare { file: 'h', rank: 6 } => H6,
            UciSquare { file: 'a', rank: 7 } => A7,
            UciSquare { file: 'b', rank: 7 } => B7,
            UciSquare { file: 'c', rank: 7 } => C7,
            UciSquare { file: 'd', rank: 7 } => D7,
            UciSquare { file: 'e', rank: 7 } => E7,
            UciSquare { file: 'f', rank: 7 } => F7,
            UciSquare { file: 'g', rank: 7 } => G7,
            UciSquare { file: 'h', rank: 7 } => H7,
            UciSquare { file: 'a', rank: 8 } => A8,
            UciSquare { file: 'b', rank: 8 } => B8,
            UciSquare { file: 'c', rank: 8 } => C8,
            UciSquare { file: 'd', rank: 8 } => D8,
            UciSquare { file: 'e', rank: 8 } => E8,
            UciSquare { file: 'f', rank: 8 } => F8,
            UciSquare { file: 'g', rank: 8 } => G8,
            UciSquare { file: 'h', rank: 8 } => H8,
            _ => return Err(TryIntoSquareError),
        })
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Promotion {
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TryIntoPromotionError;

impl TryFrom<UciPiece> for Promotion {
    type Error = TryIntoPromotionError;
    fn try_from(x: UciPiece) -> Result<Self, Self::Error> {
        Ok(match x {
            UciPiece::Knight => Promotion::Knight,
            UciPiece::Bishop => Promotion::Bishop,
            UciPiece::Rook => Promotion::Rook,
            UciPiece::Queen => Promotion::Queen,
            _ => return Err(TryIntoPromotionError),
        })
    }
}
