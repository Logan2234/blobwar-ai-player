//! Alpha - Beta algorithm.
use std::fmt;

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::{max, min};

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

fn alpha_beta_rec(depth: u8, alpha: i8, beta: i8, player: bool, state: &Configuration) -> i8 {
    if depth == 0 {
        return if player {
            -1 * state.value()
        } else {
            state.value()
        };
    }

    let mut alpha = alpha;
    let mut beta = beta;

    if player {
        for movement in state.movements() {
            alpha = max(
                alpha,
                alpha_beta_rec(depth - 1, alpha, beta, !player, &state.play(&movement)),
            );
            if beta <= alpha {
                break;
            }
        }
        alpha
    } else {
        for movement in state.movements() {
            beta = min(
                beta,
                alpha_beta_rec(depth - 1, alpha, beta, !player, &state.play(&movement)),
            );
            if beta <= alpha {
                break;
            }
        }
        beta
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let alpha = std::i8::MIN;
        let beta = std::i8::MAX;

<<<<<<< HEAD
        Some(
            state
                .movements()
                .par_bridge()
                .map(|movement| {
                    let new_value = if self.0 == 1 {
                        state.play(&movement).value()
                    } else {
                        alpha_beta_rec(self.0 - 1, alpha, beta, false, &state.play(&movement))
                    };
                    (new_value, movement)
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
=======
        let mouvements = state.movements().collect_vec();
        if mouvements.is_empty() {
            None
        } else {
            Some(
                *mouvements
                    .par_iter()
                    .map(|movement| {
                        (
                            alpha_beta_rec(self.0 - 1, alpha, beta, false, &state.play(&movement)),
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
                    .1
            )
        }
>>>>>>> 63ac43fbaf32cac0b7c4c5258cdc709769d69020
    }
}

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 4..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}
