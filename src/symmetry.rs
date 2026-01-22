use crate::board::Square;
use crate::clifford::Multivector;

/// Element of dihedral group D4 (8-fold symmetry of square)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum D4Element {
    Identity,
    Rotate90,
    Rotate180,
    Rotate270,
    ReflectHorizontal,
    ReflectVertical,
    ReflectDiag1, // a1-h8
    ReflectDiag2, // a8-h1
}

impl D4Element {
    pub fn all() -> [D4Element; 8] {
        use D4Element::*;
        [
            Identity,
            Rotate90,
            Rotate180,
            Rotate270,
            ReflectHorizontal,
            ReflectVertical,
            ReflectDiag1,
            ReflectDiag2,
        ]
    }

    /// Apply symmetry to a square
    pub fn apply(&self, sq: Square) -> Square {
        let (f, r) = (sq.file, sq.rank);
        match self {
            D4Element::Identity => sq,
            D4Element::Rotate90 => Square::new(r, 7 - f),
            D4Element::Rotate180 => Square::new(7 - f, 7 - r),
            D4Element::Rotate270 => Square::new(7 - r, f),
            D4Element::ReflectHorizontal => Square::new(f, 7 - r),
            D4Element::ReflectVertical => Square::new(7 - f, r),
            D4Element::ReflectDiag1 => Square::new(r, f),
            D4Element::ReflectDiag2 => Square::new(7 - r, 7 - f),
        }
    }

    /// Convert to rotor (even subalgebra element)
    pub fn to_rotor(&self) -> Multivector {
        // Approximate representation as rotors in Cl(8,0)
        // For proper implementation, need conformal model
        match self {
            D4Element::Identity => Multivector::scalar(1.0),
            D4Element::Rotate90 => {
                // Rotation by π/2 about center
                let mut mv = Multivector::new();
                mv.blades[0] = 0.707; // cos(π/4)
                mv.blades[0b11] = -0.707; // -sin(π/4) * e12
                mv
            }
            D4Element::Rotate180 => {
                // Rotation by π
                Multivector::bivector(0, 1)
            }
            // ... implement others
            _ => Multivector::scalar(1.0), // Placeholder
        }
    }
}
