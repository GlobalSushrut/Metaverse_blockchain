use crate::math::precision::PreciseFloat;
use crate::math::quantum_state::QuantumState;
use crate::math::quantum_entropy::DecoherenceModel;
use std::collections::{HashMap, HashSet};
use num_complex::Complex64;
use super::types::QuantumNodeID;

/// Flux Chaos Node Implementation
#[derive(Clone)]
#[allow(dead_code)]
pub struct FluxNode {
    id: NodeId,
    state: NodeState,
    entropy: PreciseFloat,
    load_factor: PreciseFloat,
    connections: HashSet<NodeId>,
}

type NodeId = QuantumNodeID;

#[derive(Clone)]
pub struct NodeState {
    processing_power: PreciseFloat,
    reliability: PreciseFloat,
    uptime: u64,
    last_sync: u64,
}

pub struct FluxNetwork {
    precision: u8,
    nodes: HashMap<NodeId, FluxNode>,
    routing_table: HashMap<NodeId, Vec<RouteInfo>>,
    chaos_threshold: PreciseFloat,
}

#[allow(dead_code)]
struct RouteInfo {
    target: NodeId,
    entropy_cost: PreciseFloat,
    load_factor: PreciseFloat,
    path: Vec<NodeId>,
}

impl FluxNetwork {
    pub fn new(precision: u8) -> Self {
        Self {
            precision,
            nodes: HashMap::new(),
            routing_table: HashMap::new(),
            chaos_threshold: PreciseFloat::new(85, 2), // 0.85 threshold
        }
    }

    pub fn add_node(&mut self, id: NodeId, state: NodeState) -> Result<(), &'static str> {
        // Calculate initial entropy
        let entropy = self.calculate_node_entropy(&state);
        
        if entropy.value < self.chaos_threshold.value {
            return Err("Node entropy below threshold");
        }

        // Create node
        let node = FluxNode {
            id: id.clone(),  // Clone ID for node storage
            state,
            entropy,
            load_factor: PreciseFloat::new(0, self.precision),
            connections: HashSet::new(),
        };

        // Add node and update routing
        self.nodes.insert(id, node);
        self.update_routing_table();
        Ok(())
    }

    pub fn route_transaction(&self, from: &NodeId, to: &NodeId) -> Result<Vec<NodeId>, &'static str> {
        // Get optimal route
        let routes = self.routing_table.get(from)
            .ok_or("Source node not found")?;
        
        let route = routes.iter()
            .find(|r| r.target == *to)
            .ok_or("No route found")?;

        // Validate route entropy
        let route_entropy = self.calculate_route_entropy(&route.path);
        if route_entropy.value < self.chaos_threshold.value {
            return Err("Route entropy below threshold");
        }

        Ok(route.path.clone())
    }

    pub fn update_node_state(&mut self, id: &NodeId, new_state: NodeState) -> Result<(), &'static str> {
        // Pre-calculate quantum metrics
        let new_entropy = self.calculate_node_entropy(&new_state);
        
        // Get node state without holding mutable borrow
        let node_state = self.nodes.get(id).cloned();
        
        match node_state {
            Some(mut node) => {
                // Update quantum state
                node.state = new_state;
                node.entropy = new_entropy;
                
                // Calculate quantum-aware load factor
                node.load_factor = self.calculate_load_factor(&node);
                
                // Atomic update
                self.nodes.insert(id.clone(), node);
                Ok(())
            },
            None => Err("Node not found")
        }


    }

    fn calculate_node_entropy(&self, state: &NodeState) -> PreciseFloat {
        // Create quantum state vector representing node state
        let state_vector = vec![
            Complex64::new(state.processing_power.value as f64 / 1000.0, 0.0),
            Complex64::new(state.reliability.value as f64 / 1000.0, 0.0),
            Complex64::new((state.uptime as f64).min(1000.0) / 1000.0, 0.0)
        ];
        
        // Create quantum state
        let mut quantum_state = QuantumState::new_pure_state(3, state_vector);
        
        // Apply decoherence effects
        let decoherence = DecoherenceModel::new(0.1, 1.0);
        decoherence.apply_decoherence(&mut quantum_state, 1.0);
        
        // Calculate quantum entropy
        let quantum_score = quantum_state.calculate_von_neumann_entropy();
        
        // Calculate classical entropy component
        let sync_factor = if state.last_sync > 0 {
            1.0 / (1.0 + (state.last_sync as f64 / 3600.0)) // Decay over hours
        } else {
            0.0
        };
        
        let classical_score = sync_factor * quantum_score;
        
        // Combine quantum and classical scores
        let total_score = 0.7 * quantum_score + 0.3 * classical_score;
        
        // Convert to PreciseFloat with 6 decimal places
        PreciseFloat::new((total_score * 1_000_000.0) as i128, 6)
    }

    fn calculate_load_factor(&self, node: &FluxNode) -> PreciseFloat {
        // Calculate load factor based on node state and connections
        let base_load = PreciseFloat::new(node.connections.len() as i128, 0);
        let state_factor = node.state.processing_power.clone();
        
        base_load.div(&state_factor.mul(&PreciseFloat::new(100, 2))) // Normalize to 0-1 range
    }

    fn calculate_route_entropy(&self, path: &[NodeId]) -> PreciseFloat {
        let mut total_entropy = PreciseFloat::new(0, self.precision);
        let mut weight = PreciseFloat::new(1, 0);
        
        for id in path {
            if let Some(node) = self.nodes.get(id) {
                total_entropy = total_entropy.add(&node.entropy.mul(&weight));
                weight = weight.mul(&PreciseFloat::new(9, 1)); // 0.9 decay factor
            }
        }
        
        total_entropy.div(&PreciseFloat::new(path.len() as i128, 0))
    }

    fn update_routing_table(&mut self) {
        let mut new_table = HashMap::new();
        
        // Calculate routes for each node pair
        for &from_id in self.nodes.keys() {
            let mut routes = Vec::new();
            
            for &to_id in self.nodes.keys() {
                if from_id != to_id {
                    if let Some(path) = self.find_optimal_route(&from_id, &to_id) {
                        let entropy_cost = self.calculate_route_entropy(&path);
                        let load_factor = self.calculate_path_load(&path);
                        
                        routes.push(RouteInfo {
                            target: to_id,
                            entropy_cost,
                            load_factor,
                            path,
                        });
                    }
                }
            }
            
            new_table.insert(from_id, routes);
        }
        
        self.routing_table = new_table;
    }

    fn find_optimal_route(&self, from: &NodeId, to: &NodeId) -> Option<Vec<NodeId>> {
        let mut distances = HashMap::new();
        let mut unvisited = HashSet::new();
        let mut previous = HashMap::new();
        
        // Initialize distances
        for &id in self.nodes.keys() {
            distances.insert(id, PreciseFloat::new(i128::MAX, 0));
            unvisited.insert(id);
        }
        
        if let Some(entry) = distances.get_mut(from) {
            *entry = PreciseFloat::new(0, 0);
        }
        
        while !unvisited.is_empty() {
            // Find node with minimum distance
            let current = {
                let mut min_dist = PreciseFloat::new(i128::MAX, 0);
                let mut min_node = None;
                
                for node in &unvisited {
                    let default_dist = PreciseFloat::new(i128::MAX, 0);
                    let dist = distances.get(node).unwrap_or(&default_dist);
                    if dist.value < min_dist.value {
                        min_dist = dist.clone();
                        min_node = Some(*node);
                    }
                }
                
                min_node?
            };
            
            if current == *to {
                break;
            }
            
            unvisited.remove(&current);
            
            // Update distances to neighbors
            if let Some(node) = self.nodes.get(&current) {
                let current_dist = distances.get(&current)?.clone();
                
                for neighbor in &node.connections {
                    if unvisited.contains(neighbor) {
                        if let Some(neighbor_node) = self.nodes.get(neighbor) {
                            let edge_cost = neighbor_node.load_factor.clone();
                            let new_dist = current_dist.add(&edge_cost);
                            
                            if let Some(old_dist) = distances.get_mut(neighbor) {
                                if new_dist.value < old_dist.value {
                                    *old_dist = new_dist;
                                    previous.insert(*neighbor, current);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Reconstruct path
        let mut path = Vec::new();
        let mut current = *to;
        
        while current != *from {
            path.push(current);
            current = *previous.get(&current)?;
        }
        path.push(*from);
        path.reverse();
        
        Some(path)
    }

    fn calculate_path_load(&self, path: &[NodeId]) -> PreciseFloat {
        let mut total_load = PreciseFloat::new(0, self.precision);
        
        for window in path.windows(2) {
            if let Some(node) = self.nodes.get(&window[0]) {
                total_load = total_load.add(&node.load_factor);
            }
        }
        
        if let Some(last) = path.last() {
            if let Some(node) = self.nodes.get(last) {
                total_load = total_load.add(&node.load_factor);
            }
        }
        
        total_load
    }

    fn rebalance_network(&mut self) -> Result<(), &'static str> {
        let mut changes = Vec::new();
        
        // Find nodes that need rebalancing
        for (&id, node) in &self.nodes {
            if node.entropy.value < self.chaos_threshold.value {
                // Find alternative nodes with better entropy
                let mut alternatives: HashSet<QuantumNodeID> = HashSet::new();
                
                // Collect potential alternatives
                for (other_id, other_node) in &self.nodes {
                    if other_node.entropy.value > node.entropy.value {
                        alternatives.insert(*other_id);
                    }
                }
                
                // Find best alternatives for each connection
                for conn in &node.connections {
                    if let Some((&alt_id, _)) = self.nodes.iter()
                        .filter(|(node_id, _)| alternatives.contains(*node_id))
                        .min_by_key(|(_, node)| node.load_factor.value) {
                        changes.push((id, *conn, alt_id));
                    }
                }
            }
        }
        
        // Apply changes
        for (node_id, old_conn, new_conn) in changes {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.connections.remove(&old_conn);
                node.connections.insert(new_conn);
            }
        }
        
        self.update_routing_table();
        Ok(())
    }
}
