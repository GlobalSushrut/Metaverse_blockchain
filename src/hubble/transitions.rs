use super::state::HubbleState;
use crate::math::precision::PreciseFloat;

pub struct StateTransition {
    pub from_state: HubbleState,
    pub to_state: HubbleState,
    pub transition_cost: PreciseFloat
}

impl StateTransition {
    pub fn new(from: HubbleState, to: HubbleState) -> Self {
        Self {
            from_state: from,
            to_state: to,
            transition_cost: PreciseFloat::new(0, 0)
        }
    }

    pub fn validate(&self) -> bool {
        // Implementation will go here
        true
    }
}
