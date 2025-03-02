use num_complex::Complex64;

#[derive(Debug, Clone)]
pub struct QuantumState {
    pub amplitudes: Vec<Complex64>,
    pub dim: usize,
    pub is_mixed: bool, // false for pure state, true for maximally mixed state
}

impl QuantumState {
    pub fn new_pure_state(dim: usize, amplitudes: Vec<Complex64>) -> Self {
        // Normalize the amplitudes
        let norm: f64 = amplitudes.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt();
        let normalized = if norm == 0.0 {
            amplitudes
        } else {
            amplitudes.into_iter().map(|a| a / norm).collect()
        };
        Self {
            amplitudes: normalized,
            dim,
            is_mixed: false,
        }
    }

    pub fn new_maximally_mixed(dim: usize) -> Self {
        // For maximally mixed, we don't need amplitudes
        Self {
            amplitudes: vec![],
            dim,
            is_mixed: true,
        }
    }

    pub fn calculate_von_neumann_entropy(&self) -> f64 {
        if !self.is_mixed {
            // Pure state entropy is 0
            0.0
        } else {
            // Maximally mixed state entropy is log2(dim)
            (self.dim as f64).log2()
        }
    }
}
