// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "bench")]
mod random_hand;

#[cfg(feature = "bench")]
pub use random_hand::{
    create_rng, generate_random_full_flush_pure_hand, generate_random_half_flush_pure_hand,
    generate_random_non_simple_pure_hand, generate_random_pure_hand,
};
