pub mod tally;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::math::precision::PreciseFloat;
use num_traits::ToPrimitive;

use self::tally::{TallyRecorder, TallyMetrics};

#[derive(Debug, Clone)]
pub struct OrchestratorState {
    pub reality_layers: HashMap<u32, RealityLayer>,
    pub quantum_tallies: HashMap<[u8; 32], QuantumTally>,
    pub entanglement_map: HashMap<[u8; 32], Vec<[u8; 32]>>,
    pub coherence_matrix: Vec<Vec<PreciseFloat>>,
    pub active_observers: u32,
}

#[derive(Debug, Clone)]
pub struct RealityLayer {
    pub layer_id: u32,
    pub quantum_state: Vec<u8>,
    pub observer_count: u32,
    pub coherence_score: PreciseFloat,
    pub entanglement_count: u32,
    pub last_sync: u64,
}

#[derive(Debug, Clone)]
pub struct QuantumTally {
    pub state_hash: [u8; 32],
    pub observer_votes: HashMap<[u8; 32], QuantumVote>,
    pub consensus_reached: bool,
    pub final_state: Option<Vec<u8>>,
    pub confidence_score: PreciseFloat,
}

#[derive(Debug, Clone)]
pub struct QuantumVote {
    pub observer_id: [u8; 32],
    pub observed_state: Vec<u8>,
    pub observation_time: u64,
    pub confidence: PreciseFloat,
}

#[derive(Debug, Clone)]
pub struct OrchestratorMetrics {
    pub tally_metrics: TallyMetrics,
    pub total_reality_layers: u32,
    pub active_observers: u32,
    pub entanglement_count: u32,
    pub coherence_score: f64,
}

pub struct Orchestrator {
    state: OrchestratorState,
    tally_recorder: TallyRecorder,
    coherence_threshold: PreciseFloat,
}

impl Orchestrator {
    /// Convert raw quantum state bytes into amplitudes and phases
    fn convert_quantum_state(&self, quantum_state: Vec<u8>) -> (Vec<PreciseFloat>, Vec<PreciseFloat>) {
        // Split quantum state into amplitude and phase components
        let n = quantum_state.len() / 2;
        let mut amplitudes = Vec::with_capacity(n);
        let mut phases = Vec::with_capacity(n);
        
        // Convert each pair of bytes into amplitude and phase
        for i in 0..n {
            let amp_byte = quantum_state[i * 2] as i128;
            let phase_byte = quantum_state[i * 2 + 1] as i128;
            
            // Scale to appropriate ranges
            let amplitude = PreciseFloat::new(amp_byte, 8); // Scale to [0,1]
            let phase = PreciseFloat::new(phase_byte, 8); // Scale to [0,2π]
            
            amplitudes.push(amplitude);
            phases.push(phase);
        }
        
        (amplitudes, phases)
    }

    pub fn record_quantum_state(
        &mut self,
        _observer_id: [u8; 32],
        quantum_state: Vec<u8>,
        reality_layer: u32,
        _metadata: HashMap<String, String>,
    ) -> Result<PreciseFloat, &'static str> {
        // Convert quantum state to amplitudes and phases
        let (amplitudes, phases) = self.convert_quantum_state(quantum_state);
        
        // Record observation and return overlap score
        self.tally_recorder.record_observation(
            reality_layer,
            amplitudes,
            phases
        )
    }

    pub fn new(coherence_threshold: PreciseFloat) -> Self {
        Self {
            state: OrchestratorState {
                reality_layers: HashMap::new(),
                quantum_tallies: HashMap::new(),
                entanglement_map: HashMap::new(),
                coherence_matrix: Vec::new(),
                active_observers: 0,
            },
            tally_recorder: TallyRecorder::new(coherence_threshold.clone()),
            coherence_threshold,
        }
    }

    pub fn register_observation(&mut self, layer_id: u32, observer_id: [u8; 32], state: [u8; 64], confidence: PreciseFloat) -> Result<(), &'static str> {
        let _layer = self.state.reality_layers
            .entry(layer_id)
            .or_insert(RealityLayer {
                layer_id,
                quantum_state: vec![0; 64],
                observer_count: 0,
                coherence_score: PreciseFloat::new(0, 20),
                entanglement_count: 0,
                last_sync: 0,
            });

        let state_hash = self.calculate_state_hash(&state);
        let tally = self.state.quantum_tallies
            .entry(state_hash)
            .or_insert(QuantumTally {
                state_hash,
                observer_votes: HashMap::new(),
                consensus_reached: false,
                final_state: None,
                confidence_score: PreciseFloat::new(0, 20),
            });

        // Record the vote
        tally.observer_votes.insert(observer_id, QuantumVote {
            observer_id,
            observed_state: state.to_vec(),
            observation_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            confidence,
        });

        self.try_reach_consensus(state_hash)?;
        Ok(())
    }

    pub fn try_reach_consensus(&mut self, state_hash: [u8; 32]) -> Result<bool, &'static str> {
        let tally = self.state.quantum_tallies.get_mut(&state_hash).ok_or("Tally not found")?;
        
        if tally.consensus_reached {
            return Ok(true);
        }

        if tally.observer_votes.len() < 3 { // Minimum 3 observers required
            return Ok(false);
        }

        let mut vote_weights = HashMap::new();
        let mut total_confidence = PreciseFloat::new(0, 20);

        // Weight votes by observer confidence
        for vote in tally.observer_votes.values() {
            let weight = vote.confidence.clone();
            total_confidence = total_confidence + weight.clone();
            
            *vote_weights
                .entry(vote.observed_state.clone())
                .or_insert(PreciseFloat::new(0, 20)) = weight;
        }

        // Find the state with highest weighted votes
        if let Some((winning_state, weight)) = vote_weights
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        {
            let consensus_threshold = total_confidence.clone() * PreciseFloat::new(75, 2); // 75% consensus threshold
            if *weight >= consensus_threshold {
                tally.consensus_reached = true;
                tally.final_state = Some(winning_state.clone());
                tally.confidence_score = weight.clone() / total_confidence.clone();
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn calculate_state_hash(&self, state: &[u8; 64]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(state);
        hasher.finalize().into()
    }

    pub fn get_layer_state(&self, layer_id: u32) -> Option<&RealityLayer> {
        self.state.reality_layers.get(&layer_id)
    }

    pub fn get_consensus_state(&self, state_hash: &[u8; 32]) -> Option<&QuantumTally> {
        self.state.quantum_tallies.get(state_hash)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrchestrationMetrics {
    pub total_reality_layers: u32,
    pub active_observers: u32,
    pub total_tallies: u32,
    pub consensus_reached_count: u32,
    pub average_confidence: f64,
    pub entanglement_count: u32,
    pub coherence_score: f64,
}

impl Orchestrator {
    pub fn get_metrics(&self) -> OrchestrationMetrics {
        let consensus_tallies: Vec<_> = self.state.quantum_tallies
            .values()
            .filter(|t| t.consensus_reached)
            .collect();

        let avg_confidence = if !consensus_tallies.is_empty() {
            consensus_tallies
                .iter()
                .map(|t| t.confidence_score.to_f64())
                .filter_map(|x| x)
                .sum::<f64>() / consensus_tallies.len() as f64
        } else {
            0.0
        };

        OrchestrationMetrics {
            total_reality_layers: self.state.reality_layers.len() as u32,
            active_observers: self.state.active_observers,
            total_tallies: self.state.quantum_tallies.len() as u32,
            consensus_reached_count: consensus_tallies.len() as u32,
            average_confidence: avg_confidence,
            entanglement_count: self.state.entanglement_map.len() as u32,
            coherence_score: self.state.reality_layers
                .values()
                .map(|l| l.coherence_score.to_f64())
                .filter_map(|x| x)
                .sum::<f64>() / self.state.reality_layers.len().max(1) as f64,
        }
    }
}
