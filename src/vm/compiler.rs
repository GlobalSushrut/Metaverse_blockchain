use crate::math::precision::PreciseFloat;

#[derive(Clone, Debug)]
pub enum Language {
    JavaScript,
    Python,
    Rust,
    Cpp,
}

pub struct CompilationMetrics {
    compile_time: PreciseFloat,
    execution_efficiency: PreciseFloat,
    parallel_factor: PreciseFloat,
    optimization_level: PreciseFloat,
    storage_access_time: PreciseFloat,
}

pub struct LanguageVM {
    precision: u8,
    language: Language,
    metrics: CompilationMetrics,
}

impl LanguageVM {
    pub fn new(precision: u8, language: Language, metrics: CompilationMetrics) -> Self {
        Self {
            precision,
            language,
            metrics,
        }
    }

    /// Implements C_Lang = 1/T_Compile × L_Execution
    pub fn calculate_compilation_efficiency(&self) -> PreciseFloat {
        let one = PreciseFloat::new(10_i128.pow(self.precision as u32), self.precision);
        one.div(&self.metrics.compile_time)
            .mul(&self.metrics.execution_efficiency)
    }

    /// Implements C_Optimized = C_Lang + (P + O)/S
    pub fn calculate_optimized_efficiency(&self) -> PreciseFloat {
        let base_efficiency = self.calculate_compilation_efficiency();
        let optimization_term = self.metrics.parallel_factor
            .add(&self.metrics.optimization_level)
            .div(&self.metrics.storage_access_time);
        
        base_efficiency.add(&optimization_term)
    }
}

pub struct VMSecurityMetrics {
    isolation_factor: PreciseFloat,
    attack_surface: PreciseFloat,
    state_trust: PreciseFloat,
}

impl VMSecurityMetrics {
    /// Implements S_VM = E_Iso/A_VM × S_Trust
    pub fn calculate_security_score(&self) -> PreciseFloat {
        self.isolation_factor
            .div(&self.attack_surface)
            .mul(&self.state_trust)
    }
}

pub struct LanguageOptimizer {
    static_typing_factor: PreciseFloat,
    execution_complexity: PreciseFloat,
    memory_overhead: PreciseFloat,
}

impl LanguageOptimizer {
    /// Implements O_Lang = T_Static/E_Complexity × 1/M_Overhead
    pub fn calculate_optimization_factor(&self) -> PreciseFloat {
        self.static_typing_factor
            .div(&self.execution_complexity)
            .div(&self.memory_overhead)
    }
}
