use super::precision::PreciseFloat;

pub struct EntropyCalculator {
    precision: u8,
}

impl EntropyCalculator {
    pub fn new(precision: u8) -> Self {
        Self { precision }
    }

    /// Implements S_Entropy(t) = 1 + 0.02 cos(t)
    pub fn calculate(&self, t: PreciseFloat) -> PreciseFloat {
        // Use fixed precision of 3 for all calculations
        let reduced_precision = 3;
        
        // Calculate 1.0 with scale of 3 (1000)
        let one = PreciseFloat::new(1000, reduced_precision);
        
        // Calculate 0.02 with scale of 3 (20)
        let coefficient = PreciseFloat::new(20, reduced_precision);

        // Calculate cos(t) with reduced scaling
        let cos_t = t.cos();
        
        // Ensure cos(t) is in [-1, 1]
        let normalized_cos = if cos_t.value > 1000 {
            PreciseFloat::new(1000, reduced_precision)
        } else if cos_t.value < -1000 {
            // Handle negative values safely using wrapping_neg
            PreciseFloat::new(-1000_i128, reduced_precision)
        } else {
            cos_t
        };
        
        // Calculate 0.02 * cos(t) safely
        let product = PreciseFloat::new(
            coefficient.value.wrapping_mul(normalized_cos.value).wrapping_div(1000),
            reduced_precision
        );
        
        // Add to 1.0 safely
        let mut result = PreciseFloat::new(
            one.value.wrapping_add(product.value),
            reduced_precision
        );
        
        // Normalize result to [950, 1050] range
        while result.value > 1050 {
            result = PreciseFloat::new(
                result.value.wrapping_div(10),
                result.scale.saturating_sub(1)
            );
        }
        while result.value < 950 {
            result = PreciseFloat::new(
                result.value.wrapping_mul(10),
                result.scale.saturating_add(1)
            );
        }
        
        result
    }
}

/// Factorial Retrograde Chain implementation
pub struct FRC {
    factorials: Vec<PreciseFloat>,
    precision: u8,
}

impl FRC {
    pub fn new(n: usize, precision: u8) -> Self {
        let mut factorials = Vec::with_capacity(n);
        let reduced_precision = precision.saturating_sub(2);
        
        // Use saturating_pow to prevent overflow
        let base = 10_i128.saturating_pow(reduced_precision as u32);
            
        // Start with ln(1) = 0
        let mut log_factorial = PreciseFloat::new(0, reduced_precision);
        factorials.push(PreciseFloat::new(base, reduced_precision)); // exp(0) = 1
        
        // Calculate factorials in log space with bounds checking
        for i in 1..=n.min(170) { // Limit n to prevent overflow
            // Add ln(i) to previous log factorial safely
            let log_i = PreciseFloat::new(i as i128, reduced_precision).ln();
            log_factorial = log_factorial.add(&log_i);
            
            // Convert back from log space and store
            factorials.push(log_factorial.exp());
        }

        Self { factorials, precision: reduced_precision }
    }

    /// Calculates FRC(n) = ∑k!
    pub fn calculate(&self, n: usize) -> PreciseFloat {
        let mut sum = PreciseFloat::new(0, self.precision);
        
        // Use logarithmic summation to prevent overflow
        for k in 0..=n.min(self.factorials.len() - 1) {
            // Convert current sum to log space if not zero
            let log_sum = if !sum.is_zero() { sum.ln() } else { 
                // Use saturating_pow for safer arithmetic
                let small_value = 10_i128.saturating_pow(self.precision as u32);
                PreciseFloat::new(-small_value, self.precision) // Very small number in log space
            };
            
            // Get log of current factorial
            let log_factorial = self.factorials[k].ln();
            
            // Add in log space using log(a + b) = log(a) + log(1 + exp(log(b) - log(a)))
            let diff = log_factorial.sub(&log_sum);
            let exp_diff = diff.exp();
            
            // Use saturating arithmetic for the base value
            let base = 10_i128.saturating_pow(self.precision as u32);
            let one = PreciseFloat::new(base, self.precision);
            
            // Perform addition in log space safely
            let log_result = log_sum.add(&(one.add(&exp_diff)).ln());
            
            // Convert back from log space
            sum = log_result.exp();
        }
        sum
    }
}
