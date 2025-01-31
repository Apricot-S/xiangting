// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

// Reference:
// https://github.com/gimite/MjaiClients/blob/master/src/org/ymatsux/mjai/client/ShantensuUtil.java
// https://github.com/gimite/mjai-manue/blob/master/coffee/shanten_analysis.coffee

use std::cmp::Ordering;

const NUM_SHUPAI_IDS: usize = 9;
const NUM_ZIPAI_IDS: usize = 7;
// 1-7{m,p,s}
const SEQUENCE_IDS: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];

fn get_necessary_tiles<const N: usize>(hand: &[u8; N], winning_hand: &[u8; N]) -> u16 {
    hand.iter()
        .zip(winning_hand)
        .enumerate()
        .fold(0u16, |necessary_tiles, (i, (&h, &w))| {
            if w > h {
                necessary_tiles | (1 << i)
            } else {
                necessary_tiles
            }
        })
}

fn update_upperbound_and_necessary_tiles_0_pair<const N: usize>(
    hand: &[u8; N],
    winning_hand: &mut [u8; N],
    current_distance: u8,
    current_necessary_tiles: &mut u16,
    upperbound: &mut u8,
    necessary_tiles: &mut u16,
) {
    match current_distance.cmp(upperbound) {
        Ordering::Less => {
            *upperbound = current_distance;
            *current_necessary_tiles = 0;
            *necessary_tiles = get_necessary_tiles(hand, winning_hand);
        }
        Ordering::Equal => {
            *necessary_tiles |= get_necessary_tiles(hand, winning_hand);
        }
        Ordering::Greater => {}
    }
}

fn update_upperbound_and_necessary_tiles_1_pair<const N: usize>(
    hand: &[u8; N],
    winning_hand: &mut [u8; N],
    current_distance: u8,
    current_necessary_tiles: &mut u16,
    upperbound: &mut u8,
    necessary_tiles: &mut u16,
    i: usize,
) {
    if winning_hand[i] > 2 {
        // Can't add a pair
        return;
    }

    // Add a pair
    winning_hand[i] += 2;

    let pair_distance = winning_hand[i].saturating_sub(hand[i]);
    let new_distance = current_distance + pair_distance;

    update_upperbound_and_necessary_tiles_0_pair(
        hand,
        winning_hand,
        new_distance,
        current_necessary_tiles,
        upperbound,
        necessary_tiles,
    );

    // Remove a pair
    winning_hand[i] -= 2;
}

pub(super) fn get_shupai_replacement_number(
    hand: &[u8; 9],
    winning_hand: &mut [u8; 9],
    current_distance: u8,
    mut current_necessary_tiles: u16,
    num_left_melds: u8,
    num_pair: u8,
    min_meld_id: usize,
    mut upperbound: u8,
) -> (u8, u16) {
    if num_left_melds == 0 {
        let mut necessary_tiles = 0u16;

        if num_pair == 0 {
            update_upperbound_and_necessary_tiles_0_pair(
                hand,
                winning_hand,
                current_distance,
                &mut current_necessary_tiles,
                &mut upperbound,
                &mut necessary_tiles,
            );
        } else {
            for i in 0..NUM_SHUPAI_IDS {
                update_upperbound_and_necessary_tiles_1_pair(
                    hand,
                    winning_hand,
                    current_distance,
                    &mut current_necessary_tiles,
                    &mut upperbound,
                    &mut necessary_tiles,
                    i,
                );
            }
        }

        return (upperbound, current_necessary_tiles | necessary_tiles);
    }

    // Add triplets
    if min_meld_id < NUM_SHUPAI_IDS {
        for i in min_meld_id..NUM_SHUPAI_IDS {
            if winning_hand[i] >= 2 {
                // Can't add a triplet
                continue;
            }

            let triplet_distance = if hand[i] <= winning_hand[i] {
                3
            } else {
                (winning_hand[i] + 3).saturating_sub(hand[i])
            };
            let new_distance = current_distance + triplet_distance;

            if triplet_distance < 3 && new_distance <= upperbound {
                winning_hand[i] += 3;
                (upperbound, current_necessary_tiles) = get_shupai_replacement_number(
                    hand,
                    winning_hand,
                    new_distance,
                    current_necessary_tiles,
                    num_left_melds - 1,
                    num_pair,
                    i,
                    upperbound,
                );
                winning_hand[i] -= 3;
            }
        }
    }

    // Add sequences
    let start_sequence_id = min_meld_id.saturating_sub(NUM_SHUPAI_IDS);

    for sequence_id in start_sequence_id..SEQUENCE_IDS.len() {
        let i = SEQUENCE_IDS[sequence_id];
        if winning_hand[i..=i + 2].iter().any(|&c| c == 4) {
            // Can't add a sequence
            continue;
        }

        let sequence_distance = (i..=i + 2).filter(|&i| hand[i] <= winning_hand[i]).count() as u8;
        let new_distance = current_distance + sequence_distance;

        if sequence_distance < 3 && new_distance <= upperbound {
            winning_hand[i..=i + 2].iter_mut().for_each(|c| *c += 1);
            (upperbound, current_necessary_tiles) = get_shupai_replacement_number(
                hand,
                winning_hand,
                new_distance,
                current_necessary_tiles,
                num_left_melds - 1,
                num_pair,
                sequence_id + NUM_SHUPAI_IDS,
                upperbound,
            );
            winning_hand[i..=i + 2].iter_mut().for_each(|c| *c -= 1);
        }
    }

    (upperbound, current_necessary_tiles)
}

pub(super) fn get_zipai_replacement_number(
    hand: &[u8; 7],
    winning_hand: &mut [u8; 7],
    current_distance: u8,
    mut current_necessary_tiles: u16,
    num_left_melds: u8,
    num_pair: u8,
    min_meld_id: usize,
    mut upperbound: u8,
) -> (u8, u16) {
    if num_left_melds == 0 {
        let mut necessary_tiles = 0u16;

        if num_pair == 0 {
            update_upperbound_and_necessary_tiles_0_pair(
                hand,
                winning_hand,
                current_distance,
                &mut current_necessary_tiles,
                &mut upperbound,
                &mut necessary_tiles,
            );
        } else {
            for i in 0..NUM_ZIPAI_IDS {
                update_upperbound_and_necessary_tiles_1_pair(
                    hand,
                    winning_hand,
                    current_distance,
                    &mut current_necessary_tiles,
                    &mut upperbound,
                    &mut necessary_tiles,
                    i,
                );
            }
        }

        return (upperbound, current_necessary_tiles | necessary_tiles);
    }

    // Add triplets
    for i in min_meld_id..NUM_ZIPAI_IDS {
        if winning_hand[i] >= 2 {
            // Can't add a triplet
            continue;
        }

        let triplet_distance = if hand[i] <= winning_hand[i] {
            3
        } else {
            (winning_hand[i] + 3).saturating_sub(hand[i])
        };
        let new_distance = current_distance + triplet_distance;

        if triplet_distance < 3 && new_distance <= upperbound {
            winning_hand[i] += 3;
            (upperbound, current_necessary_tiles) = get_zipai_replacement_number(
                hand,
                winning_hand,
                new_distance,
                current_necessary_tiles,
                num_left_melds - 1,
                num_pair,
                i,
                upperbound,
            );
            winning_hand[i] -= 3;
        }
    }

    (upperbound, current_necessary_tiles)
}

pub(super) fn get_19m_replacement_number(
    hand: &[u8; 9],
    winning_hand: &mut [u8; 9],
    current_distance: u8,
    mut current_necessary_tiles: u16,
    num_left_melds: u8,
    num_pair: u8,
    min_meld_id: usize,
    mut upperbound: u8,
) -> (u8, u16) {
    if num_left_melds == 0 {
        let mut necessary_tiles = 0u16;

        if num_pair == 0 {
            update_upperbound_and_necessary_tiles_0_pair(
                hand,
                winning_hand,
                current_distance,
                &mut current_necessary_tiles,
                &mut upperbound,
                &mut necessary_tiles,
            );
        } else {
            for i in [0, 8] {
                update_upperbound_and_necessary_tiles_1_pair(
                    hand,
                    winning_hand,
                    current_distance,
                    &mut current_necessary_tiles,
                    &mut upperbound,
                    &mut necessary_tiles,
                    i,
                );
            }
        }

        return (upperbound, current_necessary_tiles | necessary_tiles);
    }

    // Add triplets
    for i in (min_meld_id..NUM_SHUPAI_IDS).step_by(8) {
        if winning_hand[i] >= 2 {
            // Can't add a triplet
            continue;
        }

        let triplet_distance = if hand[i] <= winning_hand[i] {
            3
        } else {
            (winning_hand[i] + 3).saturating_sub(hand[i])
        };
        let new_distance = current_distance + triplet_distance;

        if triplet_distance < 3 && new_distance <= upperbound {
            winning_hand[i] += 3;
            (upperbound, current_necessary_tiles) = get_19m_replacement_number(
                hand,
                winning_hand,
                new_distance,
                current_necessary_tiles,
                num_left_melds - 1,
                num_pair,
                i,
                upperbound,
            );
            winning_hand[i] -= 3;
        }
    }

    (upperbound, current_necessary_tiles)
}
