//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};

use std::fmt;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut best_gain = std::i8::MIN;
        let mut best_movement = None;
        for movement in state.movements() {
            let curr_gain = state.play(&movement).value();
            if curr_gain > best_gain {
                best_gain = curr_gain;
                best_movement = Some(movement);
            }
        }
        best_movement
    }
}
