// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

const NUM_SUIT_IDS: usize = 9;
const NUM_HONOR_IDS: usize = 7;

// 1-7{m,p,s}
const SEQUENCE_IDS: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];

fn get_necesaary_tiles<const N: usize>(hand: &[u8; N], winning_hand: &[u8; N]) -> u16 {
    let mut necesaary_tiles = 0u16;

    for (i, (&h, &w)) in hand.iter().zip(winning_hand).enumerate() {
        if w > h {
            necesaary_tiles |= 1 << i;
        }
    }

    necesaary_tiles
}

// Reference:
// https://github.com/gimite/MjaiClients/blob/master/src/org/ymatsux/mjai/client/ShantensuUtil.java
// https://github.com/gimite/mjai-manue/blob/master/coffee/shanten_analysis.coffee
pub(super) fn get_shupai_replacement_number(
    hand: &[u8; 9],
    winning_hand: &mut [u8; 9],
    current_distance: u8,
    mut current_necesaary_tiles: u16,
    num_left_melds: u8,
    num_pair: u8,
    min_meld_id: usize,
    mut upperbound: u8,
) -> (u8, u16) {
    if num_left_melds == 0 {
        let mut necesaary_tiles = 0u16;

        for i in 0..NUM_SUIT_IDS {
            if num_pair == 1 && winning_hand[i] > 2 {
                // Can't add a pair
                continue;
            }

            if num_pair == 1 {
                // Add a pair
                winning_hand[i] += 2;
            }

            let pair_distance = winning_hand[i].saturating_sub(hand[i]);
            let new_distance = current_distance + pair_distance;

            if new_distance < upperbound {
                upperbound = new_distance;
                current_necesaary_tiles = 0;
                necesaary_tiles = get_necesaary_tiles(hand, winning_hand);
            } else if new_distance == upperbound {
                necesaary_tiles |= get_necesaary_tiles(hand, winning_hand);
            }

            if num_pair == 1 {
                // Remove a pair
                winning_hand[i] -= 2;
            }
        }

        return (upperbound, current_necesaary_tiles | necesaary_tiles);
    }

    // Add triplets
    if min_meld_id < NUM_SUIT_IDS {
        for i in min_meld_id..NUM_SUIT_IDS {
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
                (upperbound, current_necesaary_tiles) = get_shupai_replacement_number(
                    hand,
                    winning_hand,
                    new_distance,
                    current_necesaary_tiles,
                    num_left_melds - 1,
                    num_pair,
                    i,
                    upperbound,
                );
                winning_hand[i] -= 3;
            }
        }
    }

    // Sequences
    let start_sequence_id = if min_meld_id < NUM_SUIT_IDS {
        0
    } else {
        min_meld_id - NUM_SUIT_IDS
    };

    for sequence_id in start_sequence_id..SEQUENCE_IDS.len() {
        let i = SEQUENCE_IDS[sequence_id];
        if winning_hand[i] == 4 || winning_hand[i + 1] == 4 || winning_hand[i + 2] == 4 {
            // Can't add a Sequence
            continue;
        }

        #[rustfmt::skip]
        let sequence_distance = if hand[i] <= winning_hand[i] { 1 } else { 0 }
            + if hand[i + 1] <= winning_hand[i + 1] { 1 } else { 0 }
            + if hand[i + 2] <= winning_hand[i + 2] { 1 } else { 0 };
        let new_distance = current_distance + sequence_distance;

        if sequence_distance < 3 && new_distance <= upperbound {
            winning_hand[i] += 1;
            winning_hand[i + 1] += 1;
            winning_hand[i + 2] += 1;
            (upperbound, current_necesaary_tiles) = get_shupai_replacement_number(
                hand,
                winning_hand,
                new_distance,
                current_necesaary_tiles,
                num_left_melds - 1,
                num_pair,
                sequence_id + NUM_SUIT_IDS,
                upperbound,
            );
            winning_hand[i] -= 1;
            winning_hand[i + 1] -= 1;
            winning_hand[i + 2] -= 1;
        }
    }

    (upperbound, current_necesaary_tiles)
}

pub(super) fn get_zipai_replacement_number(
    hand: &[u8; 7],
    winning_hand: &mut [u8; 7],
    current_distance: u8,
    mut current_necesaary_tiles: u16,
    num_left_melds: u8,
    num_pair: u8,
    min_meld_id: usize,
    mut upperbound: u8,
) -> (u8, u16) {
    if num_left_melds == 0 {
        let mut necesaary_tiles = 0u16;

        for i in 0..NUM_HONOR_IDS {
            if num_pair == 1 && winning_hand[i] > 2 {
                // Can't add a pair
                continue;
            }

            if num_pair == 1 {
                // Add a pair
                winning_hand[i] += 2;
            }

            let pair_distance = winning_hand[i].saturating_sub(hand[i]);
            let new_distance = current_distance + pair_distance;

            if new_distance < upperbound {
                upperbound = new_distance;
                current_necesaary_tiles = 0;
                necesaary_tiles = get_necesaary_tiles(hand, winning_hand);
            } else if new_distance == upperbound {
                necesaary_tiles |= get_necesaary_tiles(hand, winning_hand);
            }

            if num_pair == 1 {
                // Remove a pair
                winning_hand[i] -= 2;
            }
        }

        return (upperbound, current_necesaary_tiles | necesaary_tiles);
    }

    // Add triplets
    if min_meld_id < NUM_HONOR_IDS {
        for i in min_meld_id..NUM_HONOR_IDS {
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
                (upperbound, current_necesaary_tiles) = get_zipai_replacement_number(
                    hand,
                    winning_hand,
                    new_distance,
                    current_necesaary_tiles,
                    num_left_melds - 1,
                    num_pair,
                    i,
                    upperbound,
                );
                winning_hand[i] -= 3;
            }
        }
    }

    (upperbound, current_necesaary_tiles)
}

// Combination of the number of blocks containing the target tile
// S_TABLE[s][0] : Number of pairs
// S_TABLE[s][1] : Number of triplets
// S_TABLE[s][2] : Number of sequences
const S_TABLE: [[u8; 3]; 8] = [
    [0, 0, 0],
    [0, 0, 1],
    [0, 0, 2],
    [0, 1, 0],
    [0, 1, 1],
    [1, 0, 0],
    [1, 0, 1],
    [1, 0, 2],
];

// Number of melds containing the target tile
// Equal to the sum of S_TABLE[s][1] and S_TABLE[s][2]
const M_TABLE: [u8; 8] = [0, 1, 2, 1, 2, 0, 1, 2];

fn get_hand_distance<const N: usize>(hand: &[u8; N], winning_hand: &[u8; N]) -> (u8, u16) {
    let mut distance = 0u8;
    let mut necesaary_tiles = 0u16;

    for (i, (&h, &w)) in hand.iter().zip(winning_hand).enumerate() {
        if w > h {
            distance += w - h;
            necesaary_tiles |= 1 << i;
        }
    }

    (distance, necesaary_tiles)
}

pub(super) fn get_19m_replacement_number(
    hand: &[u8; 9],
    num_meld: u8,
    num_pair: u8,
    i: u8,
    current_num_meld: u8,
    current_num_pair: u8,
    mut winning_hand: [u8; 9],
    replacement_number: u8,
    necesaary_tiles: u16,
) -> (u8, u16) {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(i <= 16);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);

    if i >= 9 {
        if current_num_meld == num_meld && current_num_pair == num_pair {
            let (distance, tiles) = get_hand_distance(hand, &winning_hand);
            if distance < replacement_number {
                return (distance, tiles);
            } else if distance == replacement_number {
                return (distance, necesaary_tiles | tiles);
            }
        }
        return (replacement_number, necesaary_tiles);
    }

    let mut distance = replacement_number;
    let mut tiles = necesaary_tiles;

    for s in 0..S_TABLE.len() {
        if current_num_meld + M_TABLE[s] > num_meld {
            continue;
        }
        if current_num_pair + S_TABLE[s][0] > num_pair {
            continue;
        }
        if S_TABLE[s][2] > 0 {
            continue;
        }

        winning_hand[i as usize] += S_TABLE[s][1] * 3;
        winning_hand[i as usize] += S_TABLE[s][0] * 2;
        (distance, tiles) = get_19m_replacement_number(
            hand,
            num_meld,
            num_pair,
            i + 8,
            current_num_meld + M_TABLE[s],
            current_num_pair + S_TABLE[s][0],
            winning_hand,
            distance,
            tiles,
        );
        winning_hand[i as usize] -= S_TABLE[s][0] * 2;
        winning_hand[i as usize] -= S_TABLE[s][1] * 3;
    }

    (distance, tiles)
}
