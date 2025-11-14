// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod baseline;
mod random_hand;

use criterion::{Criterion, criterion_group, criterion_main};
use random_hand::{
    create_rng, generate_random_full_flush_pure_hand, generate_random_half_flush_pure_hand,
    generate_random_non_simple_pure_hand, generate_random_pure_hand,
};
use xiangting::{PlayerCount, calculate_necessary_tiles, calculate_replacement_number};

const NUM_HAND: usize = 100_000_000;
const SAMPLE_SIZE: usize = 10_000;
const NUM_RESAMPLE: usize = 100_000;

fn xiangting_normal(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("Normal", |b| {
        let mut hand = hands.iter();
        b.iter(|| {
            calculate_replacement_number(hand.next().unwrap(), None, &PlayerCount::Four).unwrap()
        })
    });
    group.finish();
}

fn xiangting_half_flush(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_half_flush_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("Half Flush", |b| {
        let mut hand = hands.iter();
        b.iter(|| {
            calculate_replacement_number(hand.next().unwrap(), None, &PlayerCount::Four).unwrap()
        })
    });
    group.finish();
}

fn xiangting_full_flush(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_full_flush_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("Full Flush", |b| {
        let mut hand = hands.iter();
        b.iter(|| {
            calculate_replacement_number(hand.next().unwrap(), None, &PlayerCount::Four).unwrap()
        })
    });
    group.finish();
}

fn xiangting_non_simple(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_non_simple_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("Non-Simple", |b| {
        let mut hand = hands.iter();
        b.iter(|| {
            calculate_replacement_number(hand.next().unwrap(), None, &PlayerCount::Four).unwrap()
        })
    });
    group.finish();
}

fn necessary_tiles_baseline(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("Necessary tiles Baseline", |b| {
        let mut hand = hands.iter();
        b.iter(|| baseline::calculate_necessary_tiles(hand.next().unwrap()))
    });
    group.finish();
}

fn necessary_tiles_proposed(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("Necessary tiles Proposed", |b| {
        let mut hand = hands.iter();
        b.iter(|| calculate_necessary_tiles(hand.next().unwrap(), None))
    });
    group.finish();
}

fn unnecessary_tiles_baseline(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("Unnecessary tiles Baseline", |b| {
        let mut hand = hands.iter();
        b.iter(|| baseline::calculate_unnecessary_tiles(hand.next().unwrap()))
    });
    group.finish();
}

criterion_group!(
    benches_number,
    xiangting_normal,
    xiangting_half_flush,
    xiangting_full_flush,
    xiangting_non_simple,
);
criterion_group!(
    benches_tiles,
    necessary_tiles_baseline,
    necessary_tiles_proposed,
    unnecessary_tiles_baseline,
);
criterion_main!(benches_number, benches_tiles);
