use super::precision::PreciseFloat;

/// Represents a node in the Flux Chaos network
pub struct ChaosNode {
    computation_power: PreciseFloat,
    stability_index: PreciseFloat,
}

impl ChaosNode {
    pub fn new(computation_power: PreciseFloat, stability_index: PreciseFloat) -> Self {
        Self {
            computation_power,
            stability_index,
        }
    }
}

pub struct FluxNetwork {
    nodes: Vec<ChaosNode>,
    precision: u8,
}

impl FluxNetwork {
    pub fn new(precision: u8) -> Self {
        Self {
            nodes: Vec::new(),
            precision,
        }
    }

    pub fn add_node(&mut self, node: ChaosNode) {
        self.nodes.push(node);
    }

    /// Implements Flux_n = ∑(Node_i/Computation_i)
    pub fn calculate_flux(&self) -> PreciseFloat {
        if self.nodes.is_empty() {
            return PreciseFloat::new(1000, 3); // Return 1.000 for empty network
        }

        let mut sum = PreciseFloat::new(0, 3);
        
        for node in &self.nodes {
            // Normalize inputs to prevent overflow
            let comp_power = if node.computation_power.value.abs() > 1000 {
                PreciseFloat::new(1000, 3)
            } else {
                node.computation_power.clone()
            };
            
            let stab_index = if node.stability_index.value.abs() < 100 {
                PreciseFloat::new(1000, 3) // Use 1.000 if stability is too low
            } else if node.stability_index.value.abs() > 1000 {
                PreciseFloat::new(1000, 3) // Use 1.000 if stability is too high
            } else {
                node.stability_index.clone()
            };
            
            let contribution = comp_power.div(&stab_index);
            sum = sum.add(&contribution);
        }
        
        // Normalize result to [950, 1050]
        let mut result = sum;
        while result.value > 1050 {
            result = PreciseFloat::new(result.value.wrapping_div(10), result.scale.saturating_sub(1));
        }
        while result.value < 950 {
            result = PreciseFloat::new(result.value.wrapping_mul(10), result.scale.saturating_add(1));
        }
        
        result
    }

    /// Calculates network stability based on flux
    pub fn network_stability(&self) -> PreciseFloat {
        let flux = self.calculate_flux();
        let one = PreciseFloat::new(10_i128.pow(self.precision as u32), self.precision);
        one.div(&flux.add(&one))
    }
}
