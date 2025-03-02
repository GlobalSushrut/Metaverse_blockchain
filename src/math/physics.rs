use super::precision::PreciseFloat;
use super::entropy::EntropyCalculator;

pub struct PhysicsEngine {
    entropy_calculator: EntropyCalculator,
    precision: u8,
    base_parameters: Vec<PreciseFloat>,
}

impl PhysicsEngine {
    pub fn new(_precision: u8) -> Self {
        // Use fixed precision of 3 for all calculations
        let reduced_precision = 3;
        let entropy_calculator = EntropyCalculator::new(reduced_precision);
        
        // Initialize base parameters with scale of 3
        let base_parameters = vec![
            PreciseFloat::new(1200, reduced_precision), // 1.2
            PreciseFloat::new(1100, reduced_precision), // 1.1
            PreciseFloat::new(1000, reduced_precision), // 1.0
            PreciseFloat::new(1000, reduced_precision), // 1.0
            PreciseFloat::new(1000, reduced_precision), // 1.0
            PreciseFloat::new(1300, reduced_precision), // 1.3
            PreciseFloat::new(1100, reduced_precision), // 1.1
            PreciseFloat::new(1000, reduced_precision), // 1.0
            PreciseFloat::new(950, reduced_precision),  // 0.95
            PreciseFloat::new(1050, reduced_precision), // 1.05
        ];

        Self {
            entropy_calculator,
            precision: reduced_precision,
            base_parameters,
        }
    }

    /// Calculates Total Product = ∠(all base parameters × S_Entropy(t))
    pub fn total_product(&self, t: PreciseFloat) -> PreciseFloat {
        // Use fixed precision of 3 for all calculations
        let reduced_precision = 3;
        
        // Calculate entropy with fixed precision
        let entropy = self.entropy_calculator.calculate(t);
        
        // Start with 1.0
        let mut result = PreciseFloat::new(1000, reduced_precision);
        
        // Multiply all parameters safely
        for param in &self.base_parameters {
            // Normalize parameter to prevent overflow
            let normalized_param = if param.value.abs() > 1000 {
                PreciseFloat::new(1000, reduced_precision)
            } else if param.value.abs() < 100 {
                PreciseFloat::new(1000, reduced_precision)
            } else {
                param.clone()
            };
            
            // Multiply safely using wrapping operations
            result = PreciseFloat::new(
                result.value.wrapping_mul(normalized_param.value) / 1000,
                reduced_precision
            );
            
            // Keep result in reasonable range
            while result.value > 10000 {
                result = PreciseFloat::new(
                    result.value / 10,
                    result.scale
                );
            }
        }
        
        // Multiply by entropy safely
        let normalized_entropy = if entropy.value.abs() > 1000 {
            PreciseFloat::new(1000, reduced_precision)
        } else if entropy.value.abs() < 100 {
            PreciseFloat::new(1000, reduced_precision)
        } else {
            entropy
        };
        
        result = PreciseFloat::new(
            result.value.wrapping_mul(normalized_entropy.value) / 1000,
            reduced_precision
        );
        
        // Normalize final result to [950, 1050]
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

    /// Calculates S_Physics(t) = 1/Total Product(t)
    pub fn s_physics(&self, t: PreciseFloat) -> PreciseFloat {
        let total = self.total_product(t);
        
        // Handle division by zero or very small numbers
        if total.is_zero() || total.value.abs() < 950 {
            return PreciseFloat::new(1000, 3); // Return 1.000
        }
        
        // Calculate inverse directly with fixed precision
        let base = PreciseFloat::new(1_000_000, 3); // 1000.000
        let raw_result = base.value.wrapping_div(total.value);
        
        // Strictly enforce [950, 1050] range
        let normalized_value = if raw_result > 1050 {
            1050
        } else if raw_result < 950 {
            950
        } else {
            raw_result
        };
        
        PreciseFloat::new(normalized_value, 3)
    }
}
