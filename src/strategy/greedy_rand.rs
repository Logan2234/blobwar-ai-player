//! Dumb greedy algorithm but randomized.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use rand::prelude::{thread_rng, SliceRandom};
use std::fmt;

/// Dumb algorithm, however, it's randomized.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct GreedyRand();

impl fmt::Display for GreedyRand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GreedyRand")
    }
}

impl Strategy for GreedyRand {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut best_gain = std::i8::MIN;
        let mut best_movements = Vec::new();
        for movement in state.movements() {
            let curr_gain = state.play(&movement).value();
            if curr_gain > best_gain {
                best_gain = curr_gain;
                best_movements = vec![movement];
            } else if curr_gain == best_gain {
                best_movements.push(movement);
            }
        }
        best_movements.choose(&mut thread_rng()).copied()
    }
}
