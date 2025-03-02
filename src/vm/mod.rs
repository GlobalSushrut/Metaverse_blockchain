pub mod executor;
pub mod state;

use crate::math::precision::PreciseFloat;

#[derive(Debug, PartialEq, Clone)]
pub enum Language {
    Rust,
    Python,
    JavaScript
}

#[derive(Clone)]
pub struct CompilationMetrics {
    pub execution_time: PreciseFloat,
    pub memory_usage: PreciseFloat,
    pub instruction_count: u64
}

#[derive(Clone)]
pub struct LanguageVM {
    pub language: Language,
    pub metrics: CompilationMetrics
}

impl LanguageVM {
    pub fn new(_precision: u8, language: Language, metrics: CompilationMetrics) -> Self {
        Self {
            language,
            metrics
        }
    }
    
    pub fn calculate_optimized_efficiency(&self) -> PreciseFloat {
        // Calculate efficiency based on metrics
        let time_weight = PreciseFloat::new(4, 1); // 0.4
        let memory_weight = PreciseFloat::new(3, 1); // 0.3
        let instruction_weight = PreciseFloat::new(3, 1); // 0.3
        
        let time_score = self.metrics.execution_time.clone() * time_weight;
        let memory_score = self.metrics.memory_usage.clone() * memory_weight;
        let instruction_score = PreciseFloat::new(self.metrics.instruction_count as i128, 0) * instruction_weight;
        
        time_score + memory_score + instruction_score
    }
}
