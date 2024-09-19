// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use mt19937::MT19937;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng, SeedableRng};

pub fn create_rng() -> MT19937 {
    let mut seed = mt19937::Seed::default();
    thread_rng().fill(&mut seed.0[..]);
    MT19937::from_seed(seed)
}

#[inline]
fn choose_hand_length(rng: &mut impl Rng) -> usize {
    const CHOICES: [usize; 10] = [1, 2, 4, 5, 7, 8, 10, 11, 13, 14];
    *CHOICES.choose(rng).unwrap()
}

#[inline]
fn fill_hand(wall: &[u8], hand_length: usize) -> [u8; 34] {
    let mut hand = [0u8; 34];
    wall.iter()
        .take(hand_length)
        .for_each(|&t| hand[t as usize] += 1);
    hand
}

pub fn generate_random_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let mut wall = [0u8; 136];
    wall.iter_mut()
        .enumerate()
        .for_each(|(i, tile)| *tile = (i / 4) as u8);
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_half_flush_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let color_start = [0, 9, 18].choose(rng).unwrap();

    let mut wall = [0u8; 64];
    wall.iter_mut().enumerate().for_each(|(i, tile)| {
        if i < 36 {
            // Suits
            *tile = (i / 4 + color_start) as u8;
        } else {
            // Honors
            *tile = ((i - 36) / 4 + 27) as u8;
        }
    });
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_full_flush_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let color_start = [0, 9, 18].choose(rng).unwrap();

    let mut wall = [0u8; 36];
    wall.iter_mut()
        .enumerate()
        .for_each(|(i, tile)| *tile = (i / 4 + color_start) as u8);
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

pub fn generate_random_non_simple_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    const NON_SIMPLES: [u8; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];
    let mut wall = [0u8; 52];
    NON_SIMPLES
        .iter()
        .cycle()
        .take(52)
        .enumerate()
        .for_each(|(i, &tile)| {
            wall[i] = tile;
        });
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}
