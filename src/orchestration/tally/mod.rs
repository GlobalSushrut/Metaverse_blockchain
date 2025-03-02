use std::collections::HashMap;
use crate::math::precision::PreciseFloat;
use crate::math::quantum_retrogate::QuantumRetrogate;

pub mod compute;
use self::compute::{TallyComputer, TallyResult};


/// Represents a quantum state vector with its associated metrics
#[derive(Clone)]
pub struct QuantumStateVector {
    /// The quantum state amplitudes
    amplitudes: Vec<PreciseFloat>,
    /// Phase angles for each amplitude
    phases: Vec<PreciseFloat>,
    /// Coherence measure (0 to 1)
    coherence: PreciseFloat,
}

/// Tracks quantum state observations across reality layers
pub struct RealityLayer {
    /// Current quantum state vector
    state_vector: QuantumStateVector,
    /// Number of observers
    observer_count: u32,
    /// Layer stability metric
    stability: PreciseFloat,
    /// Layer coherence metric
    coherence: PreciseFloat,
    /// Entanglement coefficients with other layers
    entanglement: HashMap<u32, PreciseFloat>,
}

/// Records and processes quantum observations
pub struct TallyRecorder {
    /// Maps layer IDs to their quantum states
    reality_layers: HashMap<u32, RealityLayer>,
    /// Minimum required coherence
    coherence_threshold: PreciseFloat,
    /// Total processed observations
    observation_count: u64,
    /// Cryptographic tally computer
    tally_computer: TallyComputer,
    /// Latest tally result
    latest_result: Option<TallyResult>,
}

/// Metrics about quantum state measurements
#[derive(Debug, Clone)]
pub struct TallyMetrics {
    /// Total observations processed
    pub total_observations: u64,
    /// Number of active reality layers
    pub active_layers: usize,
    /// Average coherence across layers
    pub mean_coherence: PreciseFloat,
    /// Number of coherent states
    pub coherent_states: usize,
    /// Latest tally result
    latest_result: Option<TallyResult>,
}

impl QuantumStateVector {
    /// Create a new quantum state vector from amplitudes
    pub fn new(amplitudes: Vec<PreciseFloat>, phases: Vec<PreciseFloat>) -> Self {
        let coherence = Self::calculate_coherence(&amplitudes);
        Self {
            amplitudes,
            phases,
            coherence,
        }
    }

    /// Calculate quantum state coherence
    fn calculate_coherence(amplitudes: &[PreciseFloat]) -> PreciseFloat {
        let mut sum_squares = PreciseFloat::new(0, 6);
        
        // Calculate sum of probability amplitudes squared
        for amp in amplitudes {
            sum_squares = sum_squares + amp.mul(amp);
        }
        
        // Normalize to [0,1] range
        if sum_squares.is_zero() {
            PreciseFloat::new(0, 3)
        } else {
            PreciseFloat::new(1000, 3).div(&sum_squares.exp())
        }
    }

    /// Calculate overlap with another state vector
    pub fn calculate_overlap(&self, other: &Self) -> PreciseFloat {
        let mut overlap = PreciseFloat::new(0, 6);
        
        // Calculate quantum state overlap including phases
        for ((a1, p1), (a2, p2)) in self.amplitudes.iter().zip(&self.phases)
            .zip(other.amplitudes.iter().zip(&other.phases)) {
            
            let phase_diff = p1.sub(p2);
            let cos_phase = phase_diff.cos();
            overlap = overlap + a1.mul(a2).mul(&cos_phase);
        }
        
        overlap.mul(&overlap) // Square for probability
    }

    pub fn get_amplitudes(&self) -> &Vec<PreciseFloat> {
        &self.amplitudes
    }

    pub fn get_phases(&self) -> &Vec<PreciseFloat> {
        &self.phases
    }

    pub fn get_coherence(&self) -> PreciseFloat {
        self.coherence.clone()
    }
}


impl TallyRecorder {
    pub fn new(coherence_threshold: PreciseFloat) -> Self {
        Self {
            reality_layers: HashMap::new(),
            coherence_threshold,
            observation_count: 0,
            tally_computer: TallyComputer::new(18), // Using 18 decimal places for high precision
            latest_result: None,
        }
    }

    /// Record a new quantum state observation
    pub fn record_observation(
        &mut self,
        layer_id: u32,
        amplitudes: Vec<PreciseFloat>,
        phases: Vec<PreciseFloat>
    ) -> Result<PreciseFloat, &'static str> {
        if amplitudes.len() != phases.len() {
            return Err("Amplitude and phase vectors must have same length");
        }

        let new_state = QuantumStateVector::new(amplitudes.clone(), phases.clone());
        self.observation_count += 1;
        
        // Create operation data for tally
        let mut operation_data = Vec::new();
        operation_data.extend_from_slice(&(layer_id as u64).to_le_bytes());
        for amp in &amplitudes {
            operation_data.extend_from_slice(&amp.value.to_le_bytes());
        }
        for phase in &phases {
            operation_data.extend_from_slice(&phase.value.to_le_bytes());
        }
        
        // Convert quantum state to bytes
        let mut quantum_data = Vec::new();
        for amp in new_state.get_amplitudes() {
            quantum_data.extend_from_slice(&amp.value.to_le_bytes());
        }
        
        // Compute new tally with quantum state
        let result = self.tally_computer.compute_tally(
            &quantum_data,
            &operation_data,
            &[0u8; 32] // Empty proof for now, will be replaced with ZK proof
        );
        self.latest_result = Some(result);

        // Get or create reality layer
        let layer = self.reality_layers.entry(layer_id).or_insert_with(|| RealityLayer {
            state_vector: new_state.clone(),
            observer_count: 0,
            stability: PreciseFloat::new(1000, 3), // Start at 1.0
            coherence: PreciseFloat::new(1000, 3), // Start at 1.0
            entanglement: HashMap::new(),
        });

        // Calculate overlap with existing state
        let overlap = layer.state_vector.calculate_overlap(&new_state);
        
        // Update layer state with bounds checking
        layer.observer_count = layer.observer_count.saturating_add(1);
        layer.stability = layer.stability.mul(&overlap).min(PreciseFloat::new(1000, 3)); // Cap at 1.0

        // Process quantum state with improved memory efficiency
        const CHUNK_SIZE: usize = 32; // Increased chunk size for better performance
        let mut coherence = PreciseFloat::new(0, 8);
        let layer_id = layer_id; // Store current layer ID
        
        // Process amplitudes in chunks using iterators to avoid stack allocation
        for (i, chunk) in new_state.get_amplitudes().chunks(CHUNK_SIZE).enumerate() {
            let mut retrogate = QuantumRetrogate::new(5); // Increased qubits but process less frequently
            let phase_chunk = new_state.get_phases()
                .iter()
                .skip(i * CHUNK_SIZE)
                .take(chunk.len())
                .cloned()
                .collect::<Vec<_>>();
                
            retrogate.update_state(chunk.to_vec(), phase_chunk);
            coherence = coherence + retrogate.calculate_retrogate();
            
            // Explicitly drop retrogate to free memory
            drop(retrogate);
        }
        
        // Normalize coherence using integer division
        let chunk_count = (new_state.get_amplitudes().len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
        coherence = coherence / PreciseFloat::new(chunk_count as i128, 0);
        
        // Release the mutable borrow
        let _ = layer;
        
        // Calculate entanglement with a streaming approach
        let mut entanglement_updates = Vec::new();
        
        // Process other layers
        for (&other_id, other_layer) in self.reality_layers.iter() {
            if other_id != layer_id {
                let mut other_coherence = PreciseFloat::new(0, 8);
                
                // Process other layer's amplitudes in chunks
                for chunk in other_layer.state_vector.get_amplitudes().chunks(CHUNK_SIZE) {
                    let mut other_retrogate = QuantumRetrogate::new(3);
                    other_retrogate.update_state(
                        chunk.to_vec(),
                        vec![PreciseFloat::new(0, 8); chunk.len()]
                    );
                    other_coherence = other_coherence + other_retrogate.calculate_retrogate();
                }
                
                // Normalize other coherence
                other_coherence = other_coherence / PreciseFloat::new(
                    other_layer.state_vector.get_amplitudes().len() as i128 / CHUNK_SIZE as i128,
                    0
                );
                
                // Calculate entanglement
                let entanglement = coherence.clone() * other_coherence;
                entanglement_updates.push((other_id, entanglement));
            }
        }
        
        // Update the layer with calculated values
        if let Some(layer) = self.reality_layers.get_mut(&layer_id) {
            layer.coherence = coherence;
            
            // Apply entanglement updates
            for (other_id, entanglement) in entanglement_updates {
                layer.entanglement.insert(other_id, entanglement);
            }
        }

        Ok(overlap)
    }

    /// Get metrics about the quantum state measurements
    pub fn get_metrics(&self) -> TallyMetrics {
        let mut total_coherence = PreciseFloat::new(0, 3);
        let mut coherent_count = 0;

        for layer in self.reality_layers.values() {
            total_coherence = total_coherence + layer.state_vector.coherence.clone();
            if layer.state_vector.coherence >= self.coherence_threshold {
                coherent_count += 1;
            }
        }

        let mean_coherence = if !self.reality_layers.is_empty() {
            total_coherence.div(&PreciseFloat::new(self.reality_layers.len() as i128, 0))
        } else {
            PreciseFloat::new(0, 3)
        };

        TallyMetrics {
            total_observations: self.observation_count,
            active_layers: self.reality_layers.len(),
            mean_coherence,
            coherent_states: coherent_count,
            latest_result: self.latest_result.clone(),
        }
    }

    /// Get the quantum state for a specific reality layer
    pub fn get_layer_state(&self, layer_id: u32) -> Option<&RealityLayer> {
        self.reality_layers.get(&layer_id)
    }

    /// Calculate entanglement between two reality layers
    pub fn get_layer_entanglement(&self, layer1: u32, layer2: u32) -> Option<PreciseFloat> {
        self.reality_layers.get(&layer1)
            .and_then(|l1| l1.entanglement.get(&layer2).cloned())
    }

}
