/// Multivector in Cl(8,0) - geometric algebra over 8D Euclidean space
/// We use basis e1, e2, ..., e8 representing chess files a-h
#[derive(Clone, Debug)]
pub struct Multivector {
    /// Coefficients for each basis blade
    /// Index represents which basis vectors are present (bitmap)
    /// grades[0] = scalar, grades[1..9] = vectors, etc.
    pub blades: Vec<f32>,
}

impl Multivector {
    /// Create new multivector with 2^8 = 256 components
    pub fn new() -> Self {
        Self {
            blades: vec![0.0; 256],
        }
    }

    /// Create scalar multivector
    pub fn scalar(value: f32) -> Self {
        let mut mv = Self::new();
        mv.blades[0] = value;
        mv
    }

    /// Create basis vector e_i (for i in 0..8)
    pub fn basis_vector(i: usize) -> Self {
        assert!(i < 8);
        let mut mv = Self::new();
        mv.blades[1 << i] = 1.0;
        mv
    }

    /// Create bivector e_i ∧ e_j
    pub fn bivector(i: usize, j: usize) -> Self {
        assert!(i < 8 && j < 8 && i != j);
        let mut mv = Self::new();
        let (i, j, sign) = if i < j {
            (i, j, 1.0)
        } else {
            (j, i, -1.0)
        };
        mv.blades[(1 << i) | (1 << j)] = sign;
        mv
    }

    /// Geometric product
    pub fn geometric_product(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for i in 0..256 {
            if self.blades[i] == 0.0 {
                continue;
            }
            for j in 0..256 {
                if other.blades[j] == 0.0 {
                    continue;
                }

                // Compute product of basis blades i and j
                let (k, sign) = Self::blade_product(i, j);
                result.blades[k] += sign * self.blades[i] * other.blades[j];
            }
        }

        result
    }

    /// Inner product (grade-lowering part)
    pub fn inner_product(&self, other: &Self) -> f32 {
        self.geometric_product(other).blades[0]
    }

    /// Outer product (grade-raising part) - wedge product
    pub fn outer_product(&self, other: &Self) -> Self {
        let mut result = Self::new();

        for i in 0..256 {
            if self.blades[i] == 0.0 {
                continue;
            }
            for j in 0..256 {
                if other.blades[j] == 0.0 {
                    continue;
                }

                // Only keep grade-raising terms
                if (i & j) == 0 {
                    // No common basis vectors
                    let (k, sign) = Self::blade_product(i, j);
                    result.blades[k] += sign * self.blades[i] * other.blades[j];
                }
            }
        }

        result
    }

    /// Grade projection - extract grade-k part
    pub fn grade(&self, k: usize) -> Self {
        let mut result = Self::new();
        for i in 0usize..256 {
            if i.count_ones() as usize == k {
                result.blades[i] = self.blades[i];
            }
        }
        result
    }

    /// Compute product of two basis blades (bitmap representation)
    fn blade_product(a: usize, b: usize) -> (usize, f32) {
        let xor = a ^ b; // Result basis blade
        let common = a & b; // Common basis vectors

        // Count swaps needed to reorder
        let mut swaps = 0;
        let mut temp_a = a;
        for i in 0..8 {
            if (b & (1 << i)) != 0 {
                let mask = (1 << i) - 1;
                swaps += (temp_a & mask).count_ones();
                temp_a &= !(1 << i);
            }
        }

        // e_i * e_i = 1 for Euclidean signature
        let sign = if swaps % 2 == 0 { 1.0 } else { -1.0 };

        (xor, sign)
    }

    /// Reversion (reverse order of basis vectors)
    pub fn reverse(&self) -> Self {
        let mut result = Self::new();
        for i in 0usize..256 {
            let grade = i.count_ones();
            let sign = if (grade * (grade - 1) / 2) % 2 == 0 {
                1.0
            } else {
                -1.0
            };
            result.blades[i] = sign * self.blades[i];
        }
        result
    }

    /// Magnitude squared
    pub fn norm_squared(&self) -> f32 {
        let rev = self.reverse();
        self.geometric_product(&rev).blades[0]
    }

    /// Add two multivectors
    pub fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..256 {
            result.blades[i] = self.blades[i] + other.blades[i];
        }
        result
    }
}
