// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

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

// Number of the target tile
// Equal to the sum of S_TABLE[s][0], S_TABLE[s][1], and S_TABLE[s][2]
const N_TABLE: [u8; 8] = [0, 1, 2, 3, 4, 2, 3, 4];

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

pub(super) fn get_shupai_replacement_number(
    hand: &[u8; 9],
    num_meld: u8,
    num_pair: u8,
    i: u8,
    current_num_meld: u8,
    current_num_pair: u8,
    x: u8,
    y: u8,
    mut winning_hand: [u8; 9],
    replacement_number: u8,
    necesaary_tiles: u16,
) -> (u8, u16) {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(i <= 9);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);
    debug_assert!(x <= 4);
    debug_assert!(y <= 2);

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
        if i + 2 >= 9 && S_TABLE[s][2] > 0 {
            continue;
        }
        if x + N_TABLE[s] > 4 {
            continue;
        }

        if i + 2 < 9 {
            winning_hand[i as usize] += S_TABLE[s][2];
            winning_hand[i as usize + 1] += S_TABLE[s][2];
            winning_hand[i as usize + 2] += S_TABLE[s][2];
        }
        winning_hand[i as usize] += S_TABLE[s][1] * 3;
        winning_hand[i as usize] += S_TABLE[s][0] * 2;
        (distance, tiles) = get_shupai_replacement_number(
            hand,
            num_meld,
            num_pair,
            i + 1,
            current_num_meld + M_TABLE[s],
            current_num_pair + S_TABLE[s][0],
            y + S_TABLE[s][2],
            S_TABLE[s][2],
            winning_hand,
            distance,
            tiles,
        );
        winning_hand[i as usize] -= S_TABLE[s][0] * 2;
        winning_hand[i as usize] -= S_TABLE[s][1] * 3;
        if i + 2 < 9 {
            winning_hand[i as usize + 2] -= S_TABLE[s][2];
            winning_hand[i as usize + 1] -= S_TABLE[s][2];
            winning_hand[i as usize] -= S_TABLE[s][2];
        }
    }

    (distance, tiles)
}

pub(super) fn get_zipai_replacement_number(
    hand: &[u8; 7],
    num_meld: u8,
    num_pair: u8,
    i: u8,
    current_num_meld: u8,
    current_num_pair: u8,
    mut winning_hand: [u8; 7],
    replacement_number: u8,
    necesaary_tiles: u16,
) -> (u8, u16) {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(i <= 7);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);

    if i >= 7 {
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
        (distance, tiles) = get_zipai_replacement_number(
            hand,
            num_meld,
            num_pair,
            i + 1,
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
