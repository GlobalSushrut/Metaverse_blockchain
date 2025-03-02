use crate::math::precision::PreciseFloat;
use std::collections::HashMap;

pub struct ContentVerification {
    content_hash: PreciseFloat,
    reputation: PreciseFloat,
    depth: PreciseFloat,
    precision: u8,
    content_registry: HashMap<ContentId, ContentMetadata>,
    verification_threshold: PreciseFloat,
}

type ContentId = [u8; 32];

#[derive(Clone)]
pub struct ContentMetadata {
    content_hash: [u8; 32],
    trust_score: PreciseFloat,
    verification_count: PreciseFloat,
    last_verified: u64,
    depth_factor: PreciseFloat,
}

pub struct VerificationMetrics {
    source_reliability: PreciseFloat,
    content_integrity: PreciseFloat,
    network_consensus: PreciseFloat,
    temporal_consistency: PreciseFloat,
}

impl ContentVerification {
    pub fn new(
        content_hash: PreciseFloat,
        reputation: PreciseFloat,
        depth: PreciseFloat,
        precision: u8,
    ) -> Self {
        Self {
            content_hash,
            reputation,
            depth,
            precision,
            content_registry: HashMap::new(),
            verification_threshold: PreciseFloat::new(95, 2), // 0.95 threshold
        }
    }

    pub fn register_content(
        &mut self,
        id: ContentId,
        content_hash: [u8; 32],
        metrics: VerificationMetrics
    ) -> Result<(), &'static str> {
        // Calculate initial trust score
        let trust_score = self.calculate_trust_score(&metrics);
        
        if trust_score.value < self.verification_threshold.value {
            return Err("Content verification failed");
        }

        let metadata = ContentMetadata {
            content_hash,
            trust_score,
            verification_count: PreciseFloat::new(1, self.precision),
            last_verified: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            depth_factor: self.depth.clone(),
        };

        self.content_registry.insert(id, metadata);
        Ok(())
    }

    fn calculate_trust_score(&self, metrics: &VerificationMetrics) -> PreciseFloat {
        // Weighted combination of verification metrics
        let base_score = metrics.source_reliability
            .mul(&PreciseFloat::new(35, 2)) // 0.35 weight
            .add(&metrics.content_integrity.mul(&PreciseFloat::new(30, 2))) // 0.30 weight
            .add(&metrics.network_consensus.mul(&PreciseFloat::new(20, 2))) // 0.20 weight
            .add(&metrics.temporal_consistency.mul(&PreciseFloat::new(15, 2))) // 0.15 weight
            .div(&PreciseFloat::new(100, 2)); // Normalize weights

        // Apply depth penalty
        base_score.div(&self.depth)
    }

    /// Enhanced content verification with temporal and consensus factors
    pub fn verify_content(&self) -> (PreciseFloat, bool) {
        let base_score = self.content_hash
            .mul(&self.reputation)
            .div(&self.depth);
        
        // Apply temporal decay factor
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let temporal_factor = self.calculate_temporal_factor(current_time);
        let verification_score = base_score.mul(&temporal_factor);
        
        let is_verified = verification_score.value >= self.verification_threshold.value;
        (verification_score, is_verified)
    }

    fn calculate_temporal_factor(&self, current_time: u64) -> PreciseFloat {
        let mut total_factor = PreciseFloat::new(0, self.precision);
        let mut count = 0;

        for metadata in self.content_registry.values() {
            let age = current_time.saturating_sub(metadata.last_verified);
            let age_penalty = if age < 3600 { // Less than 1 hour
                PreciseFloat::new(100, 2) // 1.0
            } else if age < 86400 { // Less than 1 day
                PreciseFloat::new(90, 2) // 0.9
            } else if age < 604800 { // Less than 1 week
                PreciseFloat::new(75, 2) // 0.75
            } else {
                PreciseFloat::new(50, 2) // 0.5
            };

            total_factor = total_factor.add(&age_penalty);
            count += 1;
        }

        if count == 0 {
            PreciseFloat::new(100, 2) // Default to 1.0 if no history
        } else {
            total_factor.div(&PreciseFloat::new(count as i128, 0))
        }
    }
}

pub struct TrustFactorCalculator {
    verification_count: PreciseFloat,
    malicious_reports: PreciseFloat,
    source_score: PreciseFloat,
    precision: u8,
}

impl TrustFactorCalculator {
    /// Implements T_Factor = V_Count/(R_Mal + 1) × I_Source
    pub fn calculate_trust_factor(&self) -> PreciseFloat {
        let one = PreciseFloat::new(10_i128.pow(self.precision as u32), self.precision);
        let denominator = self.malicious_reports.add(&one);
        
        self.verification_count
            .div(&denominator)
            .mul(&self.source_score)
    }

    pub fn update_verification_count(&mut self) {
        let one = PreciseFloat::new(10_i128.pow(self.precision as u32), self.precision);
        self.verification_count = self.verification_count.add(&one);
    }

    pub fn report_malicious(&mut self) {
        let one = PreciseFloat::new(10_i128.pow(self.precision as u32), self.precision);
        self.malicious_reports = self.malicious_reports.add(&one);
    }
}
