// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod random_hand;

use criterion::{criterion_group, criterion_main, Criterion};
use random_hand::{
    create_rng, generate_random_full_flush_pure_hand, generate_random_half_flush_pure_hand,
    generate_random_non_simple_pure_hand, generate_random_pure_hand,
};
use xiangting::calculate_replacement_number;

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
        b.iter(|| calculate_replacement_number(hand.next().unwrap(), &None).unwrap())
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
        b.iter(|| calculate_replacement_number(hand.next().unwrap(), &None).unwrap())
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
        b.iter(|| calculate_replacement_number(hand.next().unwrap(), &None).unwrap())
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
        b.iter(|| calculate_replacement_number(hand.next().unwrap(), &None).unwrap())
    });
    group.finish();
}

criterion_group!(
    benches,
    xiangting_normal,
    xiangting_half_flush,
    xiangting_full_flush,
    xiangting_non_simple,
);
criterion_main!(benches);
