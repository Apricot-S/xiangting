// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use std::fs::File;
use std::io::Write;
use std::{env, thread};

use shanten_dp::{Data, Mode, calc_shanten, calc_shanten2, make_tile_limits};
use xiangting::{
    PlayerCount, TileCounts, TileFlags, calculate_necessary_tiles, calculate_replacement_number,
    calculate_unnecessary_tiles,
};

#[derive(Debug, PartialEq)]
pub struct VerificationResult {
    replacement_number: i8,
    tiles: Option<TileFlags>,
}

pub trait VerificationTarget {
    const NAME: &'static str;

    fn shanten_dp(
        hand: &TileCounts,
        tile_limits: &[u8; 35],
        num_melds: usize,
    ) -> VerificationResult;

    fn xiangting(hand: &TileCounts, player_count: PlayerCount) -> VerificationResult;
}

pub trait PlayerMode {
    const COUNT: PlayerCount;
    const IS_THREE_PLAYER: bool;
    const NAME: &'static str;
}

pub struct Player<const THREE_PLAYER: bool>;

impl<const THREE_PLAYER: bool> PlayerMode for Player<THREE_PLAYER> {
    const COUNT: PlayerCount = if THREE_PLAYER {
        PlayerCount::Three
    } else {
        PlayerCount::Four
    };
    const IS_THREE_PLAYER: bool = THREE_PLAYER;
    const NAME: &'static str = if THREE_PLAYER { "3p" } else { "4p" };
}

pub struct ReplacementNumber;

impl VerificationTarget for ReplacementNumber {
    const NAME: &'static str = "replacement_number";

    fn shanten_dp(
        hand: &TileCounts,
        tile_limits: &[u8; 35],
        num_melds: usize,
    ) -> VerificationResult {
        let replacement_number =
            calc_shanten(hand, tile_limits, num_melds, Mode::all(), false, false)
                .unwrap()
                .unwrap()
                + 1;

        VerificationResult {
            replacement_number,
            tiles: None,
        }
    }

    fn xiangting(hand: &TileCounts, player_count: PlayerCount) -> VerificationResult {
        let replacement_number = calculate_replacement_number(hand, player_count).unwrap();

        VerificationResult {
            replacement_number: i8::try_from(replacement_number)
                .expect("replacement number must fit in i8"),
            tiles: None,
        }
    }
}

pub struct NecessaryTiles;

impl VerificationTarget for NecessaryTiles {
    const NAME: &'static str = "necessary_tiles";

    fn shanten_dp(
        hand: &TileCounts,
        tile_limits: &[u8; 35],
        num_melds: usize,
    ) -> VerificationResult {
        let Data { shanten, waits, .. } =
            calc_shanten2(hand, tile_limits, num_melds, Mode::all(), false, false)
                .unwrap()
                .unwrap();

        VerificationResult {
            replacement_number: shanten + 1,
            tiles: Some(waits),
        }
    }

    fn xiangting(hand: &TileCounts, player_count: PlayerCount) -> VerificationResult {
        let (replacement_number, necessary_tiles) =
            calculate_necessary_tiles(hand, player_count).unwrap();

        VerificationResult {
            replacement_number: i8::try_from(replacement_number)
                .expect("replacement number must fit in i8"),
            tiles: Some(necessary_tiles),
        }
    }
}

pub struct UnnecessaryTiles;

impl VerificationTarget for UnnecessaryTiles {
    const NAME: &'static str = "unnecessary_tiles";

    fn shanten_dp(
        hand: &TileCounts,
        tile_limits: &[u8; 35],
        num_melds: usize,
    ) -> VerificationResult {
        let Data {
            shanten, discards, ..
        } = calc_shanten2(hand, tile_limits, num_melds, Mode::all(), false, false)
            .unwrap()
            .unwrap();

        VerificationResult {
            replacement_number: shanten + 1,
            tiles: Some(discards),
        }
    }

    fn xiangting(hand: &TileCounts, player_count: PlayerCount) -> VerificationResult {
        let (replacement_number, unnecessary_tiles) =
            calculate_unnecessary_tiles(hand, player_count).unwrap();

        VerificationResult {
            replacement_number: i8::try_from(replacement_number)
                .expect("replacement number must fit in i8"),
            tiles: Some(unnecessary_tiles),
        }
    }
}

pub fn verify_correctness<const N: usize, T, P, F>(num_hands: u64, generate_hand: F) -> bool
where
    T: VerificationTarget,
    P: PlayerMode,
    F: Fn(u64) -> TileCounts + Copy + Send + 'static,
{
    let num_threads = env::var("NUM_THREADS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1);

    assert!(num_threads > 0, "NUM_THREADS must be greater than 0.");

    let chunk_size = num_hands / num_threads;
    let remainder = num_hands % num_threads;
    let mut handles = Vec::new();
    let tile_limits = make_tile_limits(P::IS_THREE_PLAYER);

    for i in 0..num_threads {
        let extra = u64::from(i < remainder);
        let begin = i * chunk_size + i.min(remainder);
        let end = begin + chunk_size + extra;

        let handle = thread::spawn(move || {
            for hash in begin..end {
                let hand = generate_hand(hash);
                let result_shanten_dp = T::shanten_dp(&hand, &tile_limits, (N - 1) / 3);
                let result_xiangting = T::xiangting(&hand, P::COUNT);

                if result_shanten_dp != result_xiangting {
                    return Some(format!(
                        "API: {}\nHand: {hand:?}\nshanten-dp: {result_shanten_dp:?}\nxiangting: {result_xiangting:?}\n",
                        T::NAME
                    ));
                }
            }

            None
        });

        handles.push(handle);
    }

    let results: Vec<_> = handles.into_iter().map(thread::JoinHandle::join).collect();
    assert!(
        !results.iter().any(Result::is_err),
        "Test failed due to a thread panic."
    );

    let mismatches: Vec<_> = results
        .into_iter()
        .filter_map(Result::ok)
        .flatten()
        .collect();

    if !mismatches.is_empty() {
        let file_name = format!("./mismatches_{}_{}_{:02}.txt", T::NAME, P::NAME, N - 1);
        let mut file = File::create(&file_name).expect("Failed to create file.");
        file.write_all(mismatches.join("").as_bytes())
            .expect("Failed to write to file.");
    }

    mismatches.is_empty()
}
