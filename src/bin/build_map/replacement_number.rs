// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

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
const M_TABLE: [u8; 8] = [0, 1, 2, 1, 2, 0, 1, 2];
const N_TABLE: [u8; 8] = [0, 1, 2, 3, 4, 2, 3, 4];

fn get_hand_distance<const N: usize>(hand: &[u8; N], winning_hand: &[u8; N]) -> u8 {
    hand.iter()
        .zip(winning_hand)
        .map(|(&h, &w)| w.saturating_sub(h))
        .sum()
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
) -> u8 {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(i <= 9);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);
    debug_assert!(x <= 4);
    debug_assert!(y <= 2);

    if i >= 9 {
        if current_num_meld == num_meld && current_num_pair == num_pair {
            let distance = get_hand_distance(hand, &winning_hand);
            if distance < replacement_number {
                return distance;
            } else if distance == replacement_number {
                return distance;
            }
        }
        return replacement_number;
    }

    let mut result = replacement_number;

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
        result = get_shupai_replacement_number(
            hand,
            num_meld,
            num_pair,
            i + 1,
            current_num_meld + M_TABLE[s],
            current_num_pair + S_TABLE[s][0],
            y + S_TABLE[s][2],
            S_TABLE[s][2],
            winning_hand,
            replacement_number,
        );
        winning_hand[i as usize] -= S_TABLE[s][0] * 2;
        winning_hand[i as usize] -= S_TABLE[s][1] * 3;
        if i + 2 < 9 {
            winning_hand[i as usize + 2] -= S_TABLE[s][2];
            winning_hand[i as usize + 1] -= S_TABLE[s][2];
            winning_hand[i as usize] -= S_TABLE[s][2];
        }
    }

    result
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
) -> u8 {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(i <= 7);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);

    if i >= 7 {
        if current_num_meld == num_meld && current_num_pair == num_pair {
            let distance = get_hand_distance(hand, &winning_hand);
            if distance < replacement_number {
                return distance;
            } else if distance == replacement_number {
                return distance;
            }
        }
        return replacement_number;
    }

    let mut result = replacement_number;

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
        result = get_zipai_replacement_number(
            hand,
            num_meld,
            num_pair,
            i + 1,
            current_num_meld + M_TABLE[s],
            current_num_pair + S_TABLE[s][0],
            winning_hand,
            replacement_number,
        );
        winning_hand[i as usize] -= S_TABLE[s][0] * 2;
        winning_hand[i as usize] -= S_TABLE[s][1] * 3;
    }

    result
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
) -> u8 {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(i <= 16);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);

    if i >= 9 {
        if current_num_meld == num_meld && current_num_pair == num_pair {
            let distance = get_hand_distance(hand, &winning_hand);
            if distance < replacement_number {
                return distance;
            } else if distance == replacement_number {
                return distance;
            }
        }
        return replacement_number;
    }

    let mut result = replacement_number;

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
        result = get_19m_replacement_number(
            hand,
            num_meld,
            num_pair,
            i + 8,
            current_num_meld + M_TABLE[s],
            current_num_pair + S_TABLE[s][0],
            winning_hand,
            replacement_number,
        );
        winning_hand[i as usize] -= S_TABLE[s][0] * 2;
        winning_hand[i as usize] -= S_TABLE[s][1] * 3;
    }

    result
}
