// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

//! Deterministic, pre-generated inputs for comparing algorithm revisions.
//! Run with `cargo bench --bench algorithm`. No input generation is timed.

use std::hint::black_box;
use std::time::Instant;

use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use xiangting::{
    PlayerCount, TileCounts, calculate_necessary_tiles, calculate_replacement_number,
    calculate_unnecessary_tiles,
};

fn hands(three: bool, length: Option<usize>, flush: bool) -> Vec<TileCounts> {
    let mut rng = Pcg64Mcg::seed_from_u64(42);
    let mut wall: Vec<usize> = (0..34)
        .filter(|&i| (!three || !(1..=7).contains(&i)) && (!flush || (9..18).contains(&i)))
        .flat_map(|i| [i; 4])
        .collect();
    (0..100_000)
        .map(|i| {
            wall.shuffle(&mut rng);
            let n = length.unwrap_or([1, 2, 4, 5, 7, 8, 10, 11, 13, 14][i % 10]);
            let mut hand = [0; 34];
            for &tile in &wall[..n] {
                hand[tile] += 1;
            }
            hand
        })
        .collect()
}

fn measure<T>(pool: &[TileCounts], mut calculate: impl FnMut(&TileCounts) -> T) -> f64 {
    for hand in pool {
        black_box(calculate(black_box(hand)));
    }
    let mut samples = [0.0; 7];
    for sample in &mut samples {
        let start = Instant::now();
        for _ in 0..5 {
            for hand in pool {
                black_box(calculate(black_box(hand)));
            }
        }
        *sample =
            start.elapsed().as_secs_f64() * 1e9 / f64::from(u32::try_from(5 * pool.len()).unwrap());
    }
    samples.sort_by(f64::total_cmp);
    samples[3]
}

fn main() {
    for three in [false, true] {
        let rule = if three {
            PlayerCount::Three
        } else {
            PlayerCount::Four
        };
        for (label, length, flush) in [
            ("mixed", None, false),
            ("13", Some(13), false),
            ("14", Some(14), false),
            ("flush14", Some(14), true),
        ] {
            let pool = hands(three, length, flush);
            let number = measure(&pool, |h| calculate_replacement_number(h, rule).unwrap());
            let necessary = measure(&pool, |h| calculate_necessary_tiles(h, rule).unwrap());
            let unnecessary = measure(&pool, |h| calculate_unnecessary_tiles(h, rule).unwrap());
            println!(
                "{} {label}: {number:.2} {necessary:.2} {unnecessary:.2} ns/hand",
                if three { "3p" } else { "4p" }
            );
        }
    }
}
