use bitflags::bitflags;

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
                Some(A1), Some(B1), Some(C1), Some(D1), Some(E1), Some(F1), Some(G1), Some(H1),
                Some(A8), Some(B8), Some(C8), Some(D8), Some(E8), Some(F8), Some(G8), Some(H8),
                Some(A2), Some(B2), Some(C2), Some(D2), Some(E2), Some(F2), Some(G2), Some(H2),
                Some(A7), Some(B7), Some(C7), Some(D7), Some(E7), Some(F7), Some(G7), Some(H7),
            ],
            promotions: Default::default(),
            lost_castles: Default::default(),
            enpassant: None,
        }
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Promotion {
    Knight,
    Bishop,
    Rook,
    Queen,
}

// #[derive(Copy, Clone, Debug)]
// pub enum SquareState {
//     Empty,
//     WhitePawn,
//     WhiteKnight,
//     WhiteBishop,
//     WhiteRook,
//     WhiteQueen,
//     WhiteKing,
//     BlackPawn,
//     BlackKnight,
//     BlackBishop,
//     BlackRook,
//     BlackQueen,
//     BlackKing,
// }

// impl Default for SquareState {
//     fn default() -> Self {
//         SquareState::Empty
//     }
// }

// NOTE: Random checkmate analysis stuff. Might resume in the future.
//
// pub struct Game {
//     pieces: Vec<FivePiece>,
//     distances_and_kinds: Vec<DistanceAndKind>;
// }

// //
// pub struct Mate {
//     context: u8,
//     moves: [u8; 9]
// }

// /// A byte can perfectly encode a move's distance and piece type without the location or piece that moved. By having these in series
// #[repr(transparent)]
// pub struct DistanceAndKind(pub u8);
