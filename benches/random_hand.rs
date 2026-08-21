// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, RngExt, SeedableRng};
use rand_pcg::Pcg64Mcg;

#[must_use]
pub fn create_rng() -> Pcg64Mcg {
    Pcg64Mcg::seed_from_u64(42)
}

fn tile_from_index(index: usize) -> u8 {
    u8::try_from(index).expect("tile index must fit in u8")
}

#[inline]
fn choose_hand_length(rng: &mut impl Rng) -> usize {
    const CHOICES: [usize; 10] = [1, 2, 4, 5, 7, 8, 10, 11, 13, 14];
    *CHOICES.choose(rng).unwrap()
}

#[inline]
fn fill_hand(wall: &[u8], hand_length: usize) -> [u8; 34] {
    wall.iter()
        .take(hand_length)
        .fold([0u8; 34], |mut hand, &t| {
            hand[t as usize] += 1;
            hand
        })
}

pub fn generate_random_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let mut wall: [u8; 136] = std::array::from_fn(|i| tile_from_index(i / 4));
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_half_flush_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let color_start = rng.random_range(0..3) * 9;

    let mut wall: [u8; 64] = std::array::from_fn(|i| {
        if i < 36 {
            tile_from_index(i / 4 + color_start)
        } else {
            tile_from_index((i - 36) / 4 + 27)
        }
    });
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_full_flush_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let color_start = rng.random_range(0..3) * 9;

    let mut wall: [u8; 36] = std::array::from_fn(|i| tile_from_index(i / 4 + color_start));
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_non_simple_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    const NON_SIMPLES: [u8; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];
    let mut wall: [u8; 52] = std::array::from_fn(|i| NON_SIMPLES[i % 13]);
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}
