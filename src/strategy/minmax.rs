//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::{max, min};
use std::fmt;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

fn minmax_rec(depth: u8, player: bool, state: &Configuration) -> i8 {
    if depth == 0 {
        return if player {
            -1 * state.value()
        } else {
            state.value()
        };
    }

    let value = if player { std::i8::MIN } else { std::i8::MAX };
    state
        .movements()
        .map(|movement| minmax_rec(depth - 1, !player, &state.play(&movement)))
        .reduce(|value, child_value| {
            if player {
                max(value, child_value)
            } else {
                min(value, child_value)
            }
        })
        .unwrap_or(value)
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mouvements = state.movements().collect_vec();
        if mouvements.is_empty() {
            None
        } else {
            Some(
                *mouvements
                    .par_iter()
                    .map(|movement| {
                        (
                            minmax_rec(self.0 - 1, false, &state.play(&movement)),
                            movement,
                        )
                    })
                    .reduce_with(|(best_value, best_move), (new_value, new_move)| {
                        if new_value > best_value {
                            (new_value, new_move)
                        } else {
                            (best_value, best_move)
                        }
                    })
                    .unwrap()
                    .1,
            )
        }
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 4..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
