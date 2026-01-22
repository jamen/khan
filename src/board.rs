use crate::clifford::Multivector;

/// Chess square represented as bivector in Cl(8,0) ⊗ Cl(8,0)
/// Using file basis {e1..e8} and rank basis {f1..f8}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Square {
    pub file: u8, // 0-7 for a-h
    pub rank: u8, // 0-7 for 1-8
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Self {
        assert!(file < 8 && rank < 8);
        Self { file, rank }
    }

    pub fn from_algebraic(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }
        let chars: Vec<char> = s.chars().collect();
        let file = (chars[0] as u8).checked_sub(b'a')?;
        let rank = (chars[1] as u8).checked_sub(b'1')?;
        if file < 8 && rank < 8 {
            Some(Self::new(file, rank))
        } else {
            None
        }
    }

    pub fn to_algebraic(&self) -> String {
        format!(
            "{}{}",
            (b'a' + self.file) as char,
            (b'1' + self.rank) as char
        )
    }

    /// Convert square to bivector representation
    pub fn to_bivector(&self) -> Multivector {
        // Square at (file, rank) is e_file ∧ f_rank
        // For simplicity, we encode this as a single bivector
        // using files 0-7 for file basis and offset ranks
        let file_mv = Multivector::basis_vector(self.file as usize);
        let rank_mv = Multivector::basis_vector(self.rank as usize);
        file_mv.outer_product(&rank_mv)
    }

    pub fn index(&self) -> usize {
        (self.rank * 8 + self.file) as usize
    }

    pub fn from_index(idx: usize) -> Self {
        Self::new((idx % 8) as u8, (idx / 8) as u8)
    }
}

/// Piece types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(&self) -> f32 {
        match self {
            PieceType::Pawn => 1.0,
            PieceType::Knight => 3.0,
            PieceType::Bishop => 3.0,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => 100.0,
        }
    }

    /// Get characteristic blade for this piece type
    pub fn to_blade(&self) -> Multivector {
        // Encode piece type as a specific multivector
        // Using different grades for different pieces
        match self {
            PieceType::Pawn => Multivector::scalar(1.0),
            PieceType::Knight => Multivector::basis_vector(0),
            PieceType::Bishop => Multivector::basis_vector(1),
            PieceType::Rook => Multivector::basis_vector(2),
            PieceType::Queen => Multivector::basis_vector(3),
            PieceType::King => Multivector::basis_vector(4),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn scalar(&self) -> f32 {
        match self {
            Color::White => 1.0,
            Color::Black => -1.0,
        }
    }

    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// A piece with position
#[derive(Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub square: Square,
}

impl Piece {
    /// Create a new piece
    pub fn new(piece_type: PieceType, color: Color, square: Square) -> Self {
        Self {
            piece_type,
            color,
            square,
        }
    }

    /// Convert piece to enriched multivector
    /// Encodes: position ∧ type ∧ color
    pub fn to_multivector(&self) -> Multivector {
        let pos_mv = self.square.to_bivector();
        let type_mv = self.piece_type.to_blade();
        let color_scalar = Multivector::scalar(self.color.scalar());

        // Combine: position ∧ (type_blade * color)
        let type_color = type_mv.geometric_product(&color_scalar);
        pos_mv.outer_product(&type_color)
    }
}
