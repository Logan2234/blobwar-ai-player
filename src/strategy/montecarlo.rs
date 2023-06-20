//! Monte-Carlo algorithm -> NEVER USE THIS
use std::fmt;

use itertools::Itertools;
use rand::prelude::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use super::Strategy;
use crate::configuration::{Configuration, Movement};

/// Monte-Carlo algorithm with given maximum number of recursions.
pub struct MonteCarlo(pub u32);

fn monte_carlo(state: &Configuration, player: bool, max_iterations: u32) -> Option<Movement> {
    Some(
        state
            .movements()
            .par_bridge()
            .map(|movement| {
                let score_sum: f32 = (0..max_iterations)
                    .into_par_iter()
                    .map(|_| {
                        let mut new_state = state.play(&movement);
                        let mut local_player = player;
                        while !new_state.game_over() {
                            let movements = new_state.movements().collect_vec();
                            if movements.is_empty() {
                                break;
                            }
                            let index = thread_rng().gen_range(0..movements.len());
                            let chosen_movement = movements.get(index);
                            new_state.apply_movement(chosen_movement.unwrap());
                            local_player = !local_player;
                        }
                        let result = new_state.value() as f32;
                        if local_player {
                            -result
                        } else {
                            result
                        }
                    })
                    .sum();
                (movement, score_sum / max_iterations as f32)
            })
            .reduce_with(|(best_move, best_score), (movement, score)| {
                if score > best_score {
                    (movement, score)
                } else {
                    (best_move, best_score)
                }
            })
            .unwrap()
            .0,
    )
}

impl Strategy for MonteCarlo {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        monte_carlo(state, true, self.0)
    }
}

impl fmt::Display for MonteCarlo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monte - Carlo (nb iterations: {})", self.0)
    }
}
