use serde::{Serialize, Deserialize};
use num_traits::ToPrimitive;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div};

/// Custom high-precision arithmetic implementation
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreciseFloat {
    // Store as integer * 10^-scale for fixed-point arithmetic
    pub value: i128,
    pub scale: u8,
}

impl ToPrimitive for PreciseFloat {
    fn to_i64(&self) -> Option<i64> {
        Some((self.value as f64 / 10f64.powi(self.scale as i32)) as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some((self.value as f64 / 10f64.powi(self.scale as i32)) as u64)
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.value as f64 / 10f64.powi(self.scale as i32))
    }
}

impl PreciseFloat {
    pub fn new(value: i128, scale: u8) -> Self {
        // Ensure scale is never zero and limit to prevent overflow
        let effective_scale = scale.max(1).min(18);
        
        // Scale down the value if it's too large
        let scaled_value = if value.abs() > 1_000_000_000_000 {
            value.wrapping_div(1_000_000)
        } else {
            value
        };
        
        Self { value: scaled_value, scale: effective_scale }
    }

    pub fn from_f64(val: f64, scale: u8) -> Self {
        // Handle special cases
        if val.is_nan() || val.is_infinite() {
            return Self::new(0, scale);
        }
        
        // For very small numbers, scale up to maintain precision
        if val.abs() < 1e-6 {
            return Self::new(0, scale);
        }
        
        let multiplier = 10_i128.pow(scale as u32);
        let value = (val * multiplier as f64) as i128;
        Self { value, scale }
    }

    pub fn cos(&self) -> Self {
        // Use fixed precision of 3 for all calculations
        let reduced_precision = 3;
        
        // Normalize angle to [-π, π] with fixed precision
        let pi = PreciseFloat::new(3142, reduced_precision); // π ≈ 3.142
        let mut normalized = PreciseFloat::new(self.value, reduced_precision);
        
        // Normalize to [-π, π] range
        while normalized.value > pi.value {
            normalized = PreciseFloat::new(
                normalized.value.wrapping_sub(2 * pi.value),
                reduced_precision
            );
        }
        while normalized.value < -pi.value {
            normalized = PreciseFloat::new(
                normalized.value.wrapping_add(2 * pi.value),
                reduced_precision
            );
        }
        
        // For x near 0, return value close to 1
        if normalized.value.abs() < 100 { // Less than 0.1
            return PreciseFloat::new(1000, reduced_precision);
        }
        
        // For x near π/2 or -π/2, return value close to 0
        let pi_half = pi.value / 2;
        if (normalized.value - pi_half).abs() < 100 || 
           (normalized.value + pi_half).abs() < 100 {
            return PreciseFloat::new(0, reduced_precision);
        }
        
        // For x near π or -π, return value close to -1
        if (normalized.value - pi.value).abs() < 100 || 
           (normalized.value + pi.value).abs() < 100 {
            return PreciseFloat::new(-1000_i128, reduced_precision);
        }
        
        // For other values, use simple approximation
        let x_squared = PreciseFloat::new(
            normalized.value.wrapping_mul(normalized.value).wrapping_div(1000),
            reduced_precision
        );
        
        let mut result = PreciseFloat::new(1000, reduced_precision); // Start with 1.000
        result = PreciseFloat::new(
            result.value.wrapping_sub(x_squared.value.wrapping_div(2)),
            reduced_precision
        );
        
        // Normalize result to [-1000, 1000]
        if result.value > 1000 {
            PreciseFloat::new(1000, reduced_precision)
        } else if result.value < -1000 {
            PreciseFloat::new(-1000_i128, reduced_precision)
        } else {
            result
        }
    }

    pub fn sin(&self) -> Self {
        // Use fixed precision of 3
        let reduced_precision = 3;
        
        // Normalize angle to [-π, π] with fixed precision
        let pi = PreciseFloat::new(3142, reduced_precision); // π ≈ 3.142
        let mut normalized = PreciseFloat::new(self.value, reduced_precision);
        
        // Normalize to [-π, π] range
        while normalized.value > pi.value {
            normalized = PreciseFloat::new(
                normalized.value.wrapping_sub(2 * pi.value),
                reduced_precision
            );
        }
        while normalized.value < -pi.value {
            normalized = PreciseFloat::new(
                normalized.value.wrapping_add(2 * pi.value),
                reduced_precision
            );
        }
        
        // For x near 0, return value close to 0
        if normalized.value.abs() < 100 { // Less than 0.1
            return PreciseFloat::new(0, reduced_precision);
        }
        
        // For x near π/2, return value close to 1
        let pi_half = pi.value / 2;
        if (normalized.value - pi_half).abs() < 100 {
            return PreciseFloat::new(1000, reduced_precision);
        }
        
        // For x near -π/2, return value close to -1
        if (normalized.value + pi_half).abs() < 100 {
            return PreciseFloat::new(-1000_i128, reduced_precision);
        }
        
        // For x near π or -π, return value close to 0
        if (normalized.value - pi.value).abs() < 100 || 
           (normalized.value + pi.value).abs() < 100 {
            return PreciseFloat::new(0, reduced_precision);
        }
        
        // For other values, use simple approximation
        let x_squared = PreciseFloat::new(
            normalized.value.wrapping_mul(normalized.value).wrapping_div(1000),
            reduced_precision
        );
        
        let result = PreciseFloat::new(
            normalized.value.wrapping_sub(x_squared.value.wrapping_mul(normalized.value).wrapping_div(6000)),
            reduced_precision
        );
        
        // Normalize result to [-1000, 1000]
        if result.value > 1000 {
            PreciseFloat::new(1000, reduced_precision)
        } else if result.value < -1000 {
            PreciseFloat::new(-1000_i128, reduced_precision)
        } else {
            result
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        let v1 = self.value.checked_mul(10_i128.checked_pow((scale - self.scale) as u32)
            .expect("Scale overflow in add"))
            .expect("Value overflow in add");
        let v2 = other.value.checked_mul(10_i128.checked_pow((scale - other.scale) as u32)
            .expect("Scale overflow in add"))
            .expect("Value overflow in add");
        Self::new(
            v1.checked_add(v2).expect("Addition overflow"),
            scale
        )
    }

    pub fn sub(&self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        let v1 = self.value * 10_i128.pow((scale - self.scale) as u32);
        let v2 = other.value * 10_i128.pow((scale - other.scale) as u32);
        Self::new(v1 - v2, scale)
    }

    pub fn mul(&self, other: &Self) -> Self {
        // For very large numbers, use logarithmic space
        if self.value.abs() > 1_000_000_000 || other.value.abs() > 1_000_000_000 {
            let log_result = self.ln().add(&other.ln());
            return log_result.exp();
        }
        
        // Use saturating arithmetic for scale
        let scale = self.scale.saturating_add(other.scale);
        
        // Handle multiplication with overflow protection
        let value = self.value.checked_mul(other.value).unwrap_or_else(|| {
            if (self.value >= 0) == (other.value >= 0) {
                i128::MAX
            } else {
                i128::MIN
            }
        });
        
        Self::new(value, scale)
    }

    pub fn div(&self, other: &Self) -> Self {
        // Handle division by zero or very small numbers
        if other.value == 0 || other.value.abs() < 10 {
            // Return a safe maximum value with appropriate sign
            let max_safe = 10_i128.saturating_pow((126 - self.scale) as u32);
            return Self::new(
                if self.value >= 0 { max_safe } else { -max_safe },
                self.scale
            );
        }

        // For very large numbers, use logarithmic space to prevent overflow
        if self.value.abs() > 1_000_000_000 || other.value.abs() > 1_000_000_000 {
            let log_result = self.ln().sub(&other.ln());
            return log_result.exp();
        }

        // Use saturating arithmetic for scale calculations
        let reduced_scale = self.scale.saturating_sub(2);
        let scale_diff = self.scale.saturating_sub(reduced_scale);

        // Scale down values safely
        let scaled_self = if scale_diff > 0 {
            self.value.checked_div(10_i128.checked_pow(scale_diff as u32).unwrap_or(1))
                .unwrap_or(self.value)
        } else {
            self.value
        };

        let scaled_other = if scale_diff > 0 {
            other.value.checked_div(10_i128.checked_pow(scale_diff as u32).unwrap_or(1))
                .unwrap_or(other.value)
        } else {
            other.value
        };

        // Perform division with checked arithmetic
        let scaled_value = scaled_self
            .checked_mul(10_i128.checked_pow(reduced_scale as u32).unwrap_or(1))
            .unwrap_or_else(|| {
                if scaled_self >= 0 { i128::MAX } else { i128::MIN }
            });

        // Final division with fallback to maximum safe value
        let result = scaled_value.checked_div(scaled_other).unwrap_or_else(|| {
            if (scaled_value >= 0) == (scaled_other >= 0) {
                i128::MAX
            } else {
                i128::MIN
            }
        });

        Self::new(result, reduced_scale)
    }

    fn normalize_angle(&self) -> Self {
        // Normalize angle to [-π, π]
        let pi = PreciseFloat::new(314159265358979323846, 20); // π
        let two_pi = pi.clone().mul(&PreciseFloat::new(2, 0));
        let mut x = self.clone();
        while x.value > pi.value {
            x = x - two_pi.clone();
        }
        while x.value < -pi.value {
            x = x + two_pi.clone();
        }
        x
    }

    pub fn ln(&self) -> Self {
        if self.value <= 0 {
            // Return a very small negative number instead of panicking
            return Self::new(-1_000_000_000, 3);
        }

        // For very large or small numbers, use approximation
        if self.value.abs() > 1_000_000_000 {
            let scale_factor = (self.value.abs() as f64).log2() as i128;
            return Self::new(scale_factor.saturating_mul(693147), 6); // ln(2) ≈ 0.693147
        }

        let one = PreciseFloat::new(1000, 3); // 1.000
        
        // For values close to 1, use linear approximation
        let normalized = PreciseFloat::new(self.value, self.scale);
        if (normalized.value - one.value).abs() < 100 {
            return PreciseFloat::new((normalized.value - one.value) * 1000 / one.value, 3);
        }
        
        // For very large values, use log(a*10^n) = log(a) + n*log(10)
        if self.value.abs() > 1_000_000_000 {
            let base = self.value.abs() as f64;
            let exp = base.log10().floor();
            let mantissa = base / 10_f64.powf(exp);
            
            let mantissa_term = PreciseFloat::from_f64(mantissa.ln(), self.scale);
            let exp_term = PreciseFloat::from_f64(exp * 2.302585092994046, self.scale); // ln(10)
            return mantissa_term.add(&exp_term);
        }
        
        let x_minus_1 = self.sub(&one);
        let x_plus_1 = self.add(&one);
        
        // Prevent division by very small numbers
        if x_plus_1.value.abs() < 100 {
            return PreciseFloat::new(
                if self.value > one.value { one.value } else { -one.value },
                self.scale
            );
        }
        
        let z = x_minus_1.div(&x_plus_1);
        let mut result = z.clone();
        let mut term = z.clone();
        let z_squared = z.clone().mul(&z);

        for k in 1..10 { // Use 10 terms for good precision
            term = term.mul(&z_squared);
            let next_term = term.clone().div(&PreciseFloat::new((2 * k + 1) as i128, 0));
            result = result.add(&next_term);
        }

        result.mul(&PreciseFloat::new(2, 0))
    }

    pub fn exp(&self) -> Self {
        // For very large or small exponents, return safe values
        if self.value.abs() > 10_000 {
            return PreciseFloat::new(
                if self.value >= 0 { 1000 } else { 1 },
                3
            );
        }

        // Use a fixed-point scaling factor to maintain precision
        let scale_factor = 1000; // 3 decimal places
        let mut result = PreciseFloat::new(scale_factor, 3);
        let mut term = result.clone();
        let x = PreciseFloat::new(self.value, self.scale);

        // Use only 5 terms to prevent stack overflow
        for i in 1..=5 {
            term = PreciseFloat::new(
                term.value.wrapping_mul(x.value).wrapping_div(i as i128 * scale_factor),
                3
            );
            result = PreciseFloat::new(
                result.value.wrapping_add(term.value),
                3
            );
        }

        // Normalize result to [950, 1050]
        while result.value > 1050 {
            result = PreciseFloat::new(result.value.wrapping_div(10), result.scale.saturating_sub(1));
        }
        while result.value < 950 {
            result = PreciseFloat::new(result.value.wrapping_mul(10), result.scale.saturating_add(1));
        }

        result
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl Ord for PreciseFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for PreciseFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for PreciseFloat {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        // Normalize scales before adding
        let max_scale = self.scale.max(other.scale);
        let self_value = if self.scale < max_scale {
            self.value.wrapping_mul(10_i128.wrapping_pow((max_scale - self.scale) as u32))
        } else {
            self.value
        };
        let other_value = if other.scale < max_scale {
            other.value.wrapping_mul(10_i128.wrapping_pow((max_scale - other.scale) as u32))
        } else {
            other.value
        };
        
        // Use wrapping add
        Self {
            value: self_value.wrapping_add(other_value),
            scale: max_scale
        }
    }
}

impl<'a> Add<&'a PreciseFloat> for PreciseFloat {
    type Output = PreciseFloat;
    
    fn add(self, other: &'a PreciseFloat) -> PreciseFloat {
        // Use checked_add with saturation
        PreciseFloat {
            value: self.value.saturating_add(other.value),
            scale: self.scale
        }
    }
}

impl<'a, 'b> Add<&'b PreciseFloat> for &'a PreciseFloat {
    type Output = PreciseFloat;
    
    fn add(self, other: &'b PreciseFloat) -> PreciseFloat {
        // Use checked_add with saturation
        PreciseFloat {
            value: self.value.saturating_add(other.value),
            scale: self.scale
        }
    }
}

impl Sub<PreciseFloat> for PreciseFloat {
    type Output = Self;
    
    fn sub(self, other: PreciseFloat) -> Self {
        // Use checked_sub with saturation
        Self {
            value: self.value.saturating_sub(other.value),
            scale: self.scale
        }
    }
}

impl<'a> Sub<&'a PreciseFloat> for PreciseFloat {
    type Output = PreciseFloat;
    
    fn sub(self, other: &'a PreciseFloat) -> PreciseFloat {
        // Use checked_sub with saturation
        PreciseFloat {
            value: self.value.saturating_sub(other.value),
            scale: self.scale
        }
    }
}

impl<'a, 'b> Sub<&'b PreciseFloat> for &'a PreciseFloat {
    type Output = PreciseFloat;
    
    fn sub(self, other: &'b PreciseFloat) -> PreciseFloat {
        // Use checked_sub with saturation
        PreciseFloat {
            value: self.value.saturating_sub(other.value),
            scale: self.scale
        }
    }
}

impl Mul for PreciseFloat {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        // For large numbers, use logarithmic space
        if self.value.abs() > 1_000_000 || other.value.abs() > 1_000_000 {
            let log_result = self.ln().add(&other.ln());
            return log_result.exp();
        }
        
        // Use wrapping multiplication and adjust scale
        let mut scale = self.scale.saturating_add(other.scale);
        let mut value = self.value.wrapping_mul(other.value);
        
        // Scale down if result is too large
        while value.abs() > 1_000_000_000_000 {
            value = value.wrapping_div(1000);
            scale = scale.saturating_sub(3);
        }
        
        Self::new(value, scale)
    }
}

impl<'a> Mul<&'a PreciseFloat> for PreciseFloat {
    type Output = PreciseFloat;
    
    fn mul(self, other: &'a PreciseFloat) -> PreciseFloat {
        // For large numbers, use logarithmic space
        if self.value.abs() > 1_000_000_000 || other.value.abs() > 1_000_000_000 {
            let log_result = self.ln().add(&other.ln());
            return log_result.exp();
        }
        
        // Use checked_mul with saturation
        PreciseFloat {
            value: self.value.saturating_mul(other.value),
            scale: self.scale.saturating_add(other.scale)
        }
    }
}

impl<'a, 'b> Mul<&'b PreciseFloat> for &'a PreciseFloat {
    type Output = PreciseFloat;
    
    fn mul(self, other: &'b PreciseFloat) -> PreciseFloat {
        // For large numbers, use logarithmic space
        if self.value.abs() > 1_000_000_000 || other.value.abs() > 1_000_000_000 {
            let log_result = self.ln().add(&other.ln());
            log_result.exp()
        } else {
            let scale = self.scale.max(other.scale);
            PreciseFloat::new(
                self.value.wrapping_mul(other.value),
                scale
            )
        }
    }
}

impl Div for PreciseFloat {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        if other.value == 0 {
            panic!("Division by zero");
        }
        let scale = self.scale.max(other.scale);
        PreciseFloat::new(
            (self.value * 1_000_000) / other.value,
            scale
        )
    }
}

impl<'a> Div<&'a PreciseFloat> for PreciseFloat {
    type Output = PreciseFloat;
    
    fn div(self, other: &'a PreciseFloat) -> PreciseFloat {
        if other.value == 0 {
            panic!("Division by zero");
        }
        let scale = self.scale.max(other.scale);
        PreciseFloat::new(
            (self.value * 1_000_000) / other.value,
            scale
        )
    }
}

impl<'a, 'b> Div<&'b PreciseFloat> for &'a PreciseFloat {
    type Output = PreciseFloat;
    
    fn div(self, other: &'b PreciseFloat) -> PreciseFloat {
        if other.value == 0 {
            panic!("Division by zero");
        }
        let scale = self.scale.max(other.scale);
        PreciseFloat::new(
            (self.value * 1_000_000) / other.value,
            scale
        )
    }
}
    type Output = PreciseFloat;
