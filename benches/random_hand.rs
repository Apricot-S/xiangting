// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn create_rng() -> Pcg64Mcg {
    Pcg64Mcg::seed_from_u64(42)
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
    let mut wall: [u8; 136] = std::array::from_fn(|i| (i / 4) as u8);
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_half_flush_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let color_start = [0, 9, 18].choose(rng).unwrap();

    let suits: [u8; 36] = std::array::from_fn(|i| (i / 4 + color_start) as u8);
    let honors: [u8; 28] = std::array::from_fn(|i| (i / 4 + 27) as u8);
    let mut combined = suits.into_iter().chain(honors.into_iter());
    let mut wall: [u8; 64] = std::array::from_fn(|_| combined.next().unwrap());
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_full_flush_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let color_start = [0, 9, 18].choose(rng).unwrap();

    let mut wall: [u8; 36] = std::array::from_fn(|i| (i / 4 + color_start) as u8);
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
