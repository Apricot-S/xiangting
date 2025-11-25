// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::env;
use std::hint::black_box;
use std::process;
use std::time::Instant;
use xiangting::{PlayerCount, calculate_replacement_number};

fn create_rng() -> Pcg64Mcg {
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

fn generate_random_pure_hand(rng: &mut impl Rng) -> [u8; 34] {
    let mut wall: [u8; 136] = std::array::from_fn(|i| (i / 4) as u8);
    wall.shuffle(rng);

    let hand_length = choose_hand_length(rng);

    fill_hand(&wall, hand_length)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <# OF HANDS>", args[0]);
        process::exit(1);
    }

    let Ok(num_hands) = args[1].parse::<i64>() else {
        eprintln!("Error: <# OF HANDS> must be a valid integer");
        process::exit(1);
    };

    if num_hands <= 0 {
        eprintln!("Error: <# OF HANDS> must be greater than 0");
        process::exit(1);
    }

    let mut rng = create_rng();

    let hands: Vec<_> = (0..num_hands)
        .map(|_| generate_random_pure_hand(&mut rng))
        .collect();

    println!("Running {} hands...", num_hands);

    let start = Instant::now();

    for hand in hands {
        let replacement_number = calculate_replacement_number(&hand, &PlayerCount::Four).unwrap();
        black_box(replacement_number);
    }

    let duration = start.elapsed();
    let total_ns = duration.as_nanos();
    let avg_ns = total_ns / (num_hands as u128);

    println!("Average time: {} ns / hand", avg_ns);
}
