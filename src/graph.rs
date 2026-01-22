
/// Nilpotent generator for graph representation
/// These satisfy s_i^2 = 0, s_i * s_j = -s_j * s_i
#[derive(Clone)]
pub struct NilpotentGenerator {
    index: usize,
}

impl NilpotentGenerator {
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    /// Clifford product with another generator
    pub fn multiply(&self, other: &Self) -> (f32, Vec<usize>) {
        if self.index == other.index {
            // s_i^2 = 0
            (0.0, vec![])
        } else {
            // s_i * s_j = -s_j * s_i (anticommutation)
            let sign = if self.index < other.index { 1.0 } else { -1.0 };
            let mut indices = vec![self.index, other.index];
            indices.sort();
            (sign, indices)
        }
    }
}

/// Clifford adjacency matrix for attack graph
pub struct CliffordAdjacencyMatrix {
    /// entries[i][j] contains (coefficient, generator_indices)
    entries: Vec<Vec<(f32, Vec<usize>)>>,
}

impl CliffordAdjacencyMatrix {
    pub fn new(size: usize) -> Self {
        Self {
            entries: vec![vec![(0.0, vec![]); size]; size],
        }
    }

    /// Set entry for attack from square i to square j
    pub fn set_attack(&mut self, from: usize, to: usize) {
        let gen_from = NilpotentGenerator::new(from);
        let gen_to = NilpotentGenerator::new(to);
        self.entries[from][to] = gen_from.multiply(&gen_to);
    }

    /// Compute matrix power to count k-cycles
    pub fn power(&self, k: usize) -> Self {
        if k == 0 {
            // Identity matrix
            let mut identity = Self::new(self.entries.len());
            for i in 0..self.entries.len() {
                identity.entries[i][i] = (1.0, vec![]);
            }
            return identity;
        }
        if k == 1 {
            return self.clone();
        }

        let mut result = self.clone();
        for _ in 1..k {
            result = result.multiply(self);
        }
        result
    }

    /// Multiply two Clifford adjacency matrices
    fn multiply(&self, other: &Self) -> Self {
        let n = self.entries.len();
        let mut result = Self::new(n);

        for i in 0..n {
            for j in 0..n {
                let mut acc_coeff = 0.0;
                let mut acc_indices: Vec<usize> = vec![];

                for k in 0..n {
                    let (coeff1, ind1) = &self.entries[i][k];
                    let (coeff2, ind2) = &other.entries[k][j];

                    if *coeff1 == 0.0 || *coeff2 == 0.0 {
                        continue;
                    }

                    // Multiply Clifford elements
                    let (sign, combined) = Self::combine_indices(ind1, ind2);
                    if combined.is_empty() && sign == 0.0 {
                        continue; // Nilpotent: product is zero
                    }

                    acc_coeff += sign * coeff1 * coeff2;
                    acc_indices = combined;
                }

                result.entries[i][j] = (acc_coeff, acc_indices);
            }
        }

        result
    }

    /// Combine generator indices with proper anticommutation
    fn combine_indices(ind1: &[usize], ind2: &[usize]) -> (f32, Vec<usize>) {
        let mut combined = ind1.to_vec();
        let mut sign = 1.0;

        for &idx in ind2 {
            if let Some(pos) = combined.iter().position(|&x| x == idx) {
                // s_i^2 = 0, cancel this generator
                combined.remove(pos);
                // Check if we need to swap (anticommutation)
                let swaps = combined[pos..].len();
                if swaps % 2 == 1 {
                    sign *= -1.0;
                }
            } else {
                // Insert in sorted order, track sign from swaps
                let pos = combined.iter().position(|&x| x > idx).unwrap_or(combined.len());
                let swaps = combined[pos..].len();
                if swaps % 2 == 1 {
                    sign *= -1.0;
                }
                combined.insert(pos, idx);
            }
        }

        (sign, combined)
    }

    /// Trace (sum of diagonal entries) - counts closed walks
    pub fn trace(&self) -> f32 {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let (coeff, indices) = &row[i];
                if indices.is_empty() {
                    *coeff
                } else {
                    0.0 // Non-scalar diagonal entries don't contribute
                }
            })
            .sum()
    }
}

impl Clone for CliffordAdjacencyMatrix {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
        }
    }
}
