// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod baseline;
mod random_hand;

use std::env;

use criterion::{Criterion, criterion_group, criterion_main};
use random_hand::{
    create_rng, generate_random_full_flush_pure_hand, generate_random_half_flush_pure_hand,
    generate_random_non_simple_pure_hand, generate_random_pure_hand,
};
use shanten_dp::{Mode, calc_shanten, calc_shanten2, make_tile_limits};
use xiangting::{
    PlayerCount, calculate_necessary_tiles, calculate_replacement_number,
    calculate_unnecessary_tiles,
};

const DEFAULT_NUM_HAND_POOL: usize = 1_000_000;
const SAMPLE_SIZE: usize = 10_000;
const NUM_RESAMPLE: usize = 100_000;

fn num_hand_pool() -> usize {
    env::var("BENCH_HAND_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_NUM_HAND_POOL)
}

fn generate_hands(generate_hand: fn(&mut rand_pcg::Pcg64Mcg) -> [u8; 34]) -> Vec<[u8; 34]> {
    let mut rng = create_rng();
    (0..num_hand_pool())
        .map(|_| generate_hand(&mut rng))
        .collect()
}

fn next_hand<'a>(hands: &'a [[u8; 34]], index: &mut usize) -> &'a [u8; 34] {
    let hand = &hands[*index % hands.len()];
    *index += 1;
    hand
}

fn num_melds(hand: &[u8; 34]) -> usize {
    (hand.iter().sum::<u8>() as usize) / 3
}

fn shanten_number(c: &mut Criterion) {
    let hands = generate_hands(generate_random_pure_hand);
    let tile_limits = make_tile_limits(false);

    let mut group = c.benchmark_group("normal/shanten_number");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("xiangting", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            calculate_replacement_number(hand, PlayerCount::Four).unwrap()
        });
    });
    group.bench_function("shanten-dp", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            calc_shanten(
                hand,
                &tile_limits,
                num_melds(hand),
                Mode::all(),
                false,
                false,
            )
            .unwrap()
            .unwrap()
        });
    });
    group.finish();
}

fn variant_shanten_number(c: &mut Criterion) {
    let normal_hands = generate_hands(generate_random_pure_hand);
    let half_flush_hands = generate_hands(generate_random_half_flush_pure_hand);
    let full_flush_hands = generate_hands(generate_random_full_flush_pure_hand);
    let non_simple_hands = generate_hands(generate_random_non_simple_pure_hand);

    let mut group = c.benchmark_group("variant/shanten_number");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("normal", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&normal_hands, &mut index);
            calculate_replacement_number(hand, PlayerCount::Four).unwrap()
        });
    });
    group.bench_function("half_flush", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&half_flush_hands, &mut index);
            calculate_replacement_number(hand, PlayerCount::Four).unwrap()
        });
    });
    group.bench_function("full_flush", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&full_flush_hands, &mut index);
            calculate_replacement_number(hand, PlayerCount::Four).unwrap()
        });
    });
    group.bench_function("non_simple", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&non_simple_hands, &mut index);
            calculate_replacement_number(hand, PlayerCount::Four).unwrap()
        });
    });
    group.finish();
}

fn necessary_tiles(c: &mut Criterion) {
    let hands = generate_hands(generate_random_pure_hand);
    let tile_limits = make_tile_limits(false);

    let mut group = c.benchmark_group("normal/necessary_tiles");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("xiangting", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            calculate_necessary_tiles(hand, PlayerCount::Four).unwrap()
        });
    });
    group.bench_function("shanten-dp", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            calc_shanten2(
                hand,
                &tile_limits,
                num_melds(hand),
                Mode::all(),
                false,
                false,
            )
            .unwrap()
            .unwrap()
        });
    });
    group.bench_function("baseline", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            baseline::calculate_necessary_tiles(hand)
        });
    });
    group.finish();
}

fn unnecessary_tiles(c: &mut Criterion) {
    let hands = generate_hands(generate_random_pure_hand);
    let tile_limits = make_tile_limits(false);

    let mut group = c.benchmark_group("normal/unnecessary_tiles");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("xiangting", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            calculate_unnecessary_tiles(hand, PlayerCount::Four).unwrap()
        });
    });
    group.bench_function("shanten-dp", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            calc_shanten2(
                hand,
                &tile_limits,
                num_melds(hand),
                Mode::all(),
                false,
                false,
            )
            .unwrap()
            .unwrap()
        });
    });
    group.bench_function("baseline", |b| {
        let mut index = 0;
        b.iter(|| {
            let hand = next_hand(&hands, &mut index);
            baseline::calculate_unnecessary_tiles(hand)
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    shanten_number,
    variant_shanten_number,
    necessary_tiles,
    unnecessary_tiles,
);
criterion_main!(benches);
