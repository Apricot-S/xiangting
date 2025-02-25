// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

//! Replacement number calculation algorithm using pruning DFS made efficient by
//! using Decomposition Elements.
//!
//! Reference:
//!
//! https://github.com/gimite/MjaiClients/blob/master/src/org/ymatsux/mjai/client/ShantensuUtil.java
//! https://qiita.com/Cryolite/items/75d504c7489426806b87

#![allow(clippy::too_many_arguments)]

use std::cmp::Ordering;

/// An element that represents the number of blocks (meld, pair) made up of
/// certain tiles in a winning hand.
struct DecompositionElement {
    num_sequence: u8,
    num_triplet: u8,
    num_pair: u8,
}

/// Table of decomposition elements.
///
/// (3, 0, 0) and (4, 0, 0) are not necessary for calculating the partial replacement number.
/// Combinations with `num_sequence` greater than 2 can be covered by other elements.
///
/// Example:
/// * (123 123 123) contains the same number of tiles as (111) (222) (333).
/// * (123 123 123 123) contains the same number of tiles as (123 111) (222) (333).
#[rustfmt::skip]
const D_TABLE: [DecompositionElement; 8] = [
    DecompositionElement { num_sequence: 0, num_triplet: 0, num_pair: 0 },
    DecompositionElement { num_sequence: 0, num_triplet: 0, num_pair: 1 },
    DecompositionElement { num_sequence: 0, num_triplet: 1, num_pair: 0 },
    DecompositionElement { num_sequence: 1, num_triplet: 0, num_pair: 0 },
    DecompositionElement { num_sequence: 1, num_triplet: 0, num_pair: 1 },
    DecompositionElement { num_sequence: 1, num_triplet: 1, num_pair: 0 },
    DecompositionElement { num_sequence: 2, num_triplet: 0, num_pair: 0 },
    DecompositionElement { num_sequence: 2, num_triplet: 0, num_pair: 1 },
];

/// Table of number of melds included in decomposition elements.
const M_TABLE: [u8; 8] = [
    D_TABLE[0].num_sequence + D_TABLE[0].num_triplet,
    D_TABLE[1].num_sequence + D_TABLE[1].num_triplet,
    D_TABLE[2].num_sequence + D_TABLE[2].num_triplet,
    D_TABLE[3].num_sequence + D_TABLE[3].num_triplet,
    D_TABLE[4].num_sequence + D_TABLE[4].num_triplet,
    D_TABLE[5].num_sequence + D_TABLE[5].num_triplet,
    D_TABLE[6].num_sequence + D_TABLE[6].num_triplet,
    D_TABLE[7].num_sequence + D_TABLE[7].num_triplet,
];

/// Table of number of tiles included in decomposition elements.
const N_TABLE: [u8; 8] = [
    D_TABLE[0].num_sequence + 3 * D_TABLE[0].num_triplet + 2 * D_TABLE[0].num_pair,
    D_TABLE[1].num_sequence + 3 * D_TABLE[1].num_triplet + 2 * D_TABLE[1].num_pair,
    D_TABLE[2].num_sequence + 3 * D_TABLE[2].num_triplet + 2 * D_TABLE[2].num_pair,
    D_TABLE[3].num_sequence + 3 * D_TABLE[3].num_triplet + 2 * D_TABLE[3].num_pair,
    D_TABLE[4].num_sequence + 3 * D_TABLE[4].num_triplet + 2 * D_TABLE[4].num_pair,
    D_TABLE[5].num_sequence + 3 * D_TABLE[5].num_triplet + 2 * D_TABLE[5].num_pair,
    D_TABLE[6].num_sequence + 3 * D_TABLE[6].num_triplet + 2 * D_TABLE[6].num_pair,
    D_TABLE[7].num_sequence + 3 * D_TABLE[7].num_triplet + 2 * D_TABLE[7].num_pair,
];

fn get_hand_distance<const N: usize>(target_hand: &[u8; N], hand: &[u8; N]) -> u8 {
    target_hand
        .iter()
        .zip(hand.iter())
        .fold(0u8, |distance, (&t, &h)| distance + t.saturating_sub(h))
}

fn get_necessary_tiles<const N: usize>(target_hand: &[u8; N], hand: &[u8; N]) -> u16 {
    target_hand
        .iter()
        .zip(hand.iter())
        .enumerate()
        .fold(0u16, |necessary_tiles, (i, (&t, &h))| {
            if t > h {
                necessary_tiles | (1 << i)
            } else {
                necessary_tiles
            }
        })
}

pub(super) fn get_shupai_replacement_number(
    hand: &[u8; 9],
    num_meld: u8,
    num_pair: u8,
    current_rank: usize,
    current_num_meld: u8,
    current_num_pair: u8,
    target_hand: &mut [u8; 9],
    mut upper_bound: u8,
    mut necessary_tiles: u16,
) -> (u8, u16) {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(current_rank <= 9);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);
    debug_assert!(target_hand.iter().all(|&c| c <= 4));
    debug_assert!(necessary_tiles <= 0x1FF);

    if current_rank == 9 {
        if current_num_meld == num_meld && current_num_pair == num_pair {
            let distance = get_hand_distance(target_hand, hand);
            match distance.cmp(&upper_bound) {
                Ordering::Less => {
                    upper_bound = distance;
                    necessary_tiles = get_necessary_tiles(target_hand, hand);
                }
                Ordering::Equal => necessary_tiles |= get_necessary_tiles(target_hand, hand),
                Ordering::Greater => (),
            }
        }
        return (upper_bound, necessary_tiles);
    }

    for (d, (&m, &n)) in D_TABLE.iter().zip(M_TABLE.iter().zip(N_TABLE.iter())) {
        if current_num_meld + m > num_meld {
            continue;
        }
        if current_num_pair + d.num_pair > num_pair {
            continue;
        }
        if current_rank >= 7 && d.num_sequence > 0 {
            // No sequence may start with 8, 9.
            continue;
        }
        if target_hand[current_rank] + n > 4 {
            // The number of copies of each tile in the hand must not exceed four.
            continue;
        }

        target_hand[current_rank] += n;
        if current_rank < 7 {
            target_hand[current_rank + 1] += d.num_sequence;
            target_hand[current_rank + 2] += d.num_sequence;
        }

        let lower_bound = get_hand_distance(target_hand, hand);
        if lower_bound <= upper_bound {
            let (temp_r, temp_n) = get_shupai_replacement_number(
                hand,
                num_meld,
                num_pair,
                current_rank + 1,
                current_num_meld + m,
                current_num_pair + d.num_pair,
                target_hand,
                upper_bound,
                necessary_tiles,
            );

            match temp_r.cmp(&upper_bound) {
                Ordering::Less => {
                    upper_bound = temp_r;
                    necessary_tiles = temp_n;
                }
                Ordering::Equal => necessary_tiles |= temp_n,
                Ordering::Greater => (),
            }
        }

        if current_rank < 7 {
            target_hand[current_rank + 2] -= d.num_sequence;
            target_hand[current_rank + 1] -= d.num_sequence;
        }
        target_hand[current_rank] -= n;
    }

    (upper_bound, necessary_tiles)
}

pub(super) fn get_zipai_replacement_number(
    hand: &[u8; 7],
    num_meld: u8,
    num_pair: u8,
    current_rank: usize,
    current_num_meld: u8,
    current_num_pair: u8,
    target_hand: &mut [u8; 7],
    mut upper_bound: u8,
    mut necessary_tiles: u16,
) -> (u8, u16) {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(current_rank <= 7);
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);
    debug_assert!(target_hand.iter().all(|&c| c <= 4));
    debug_assert!(necessary_tiles <= 0x7F);

    if current_rank == 7 {
        if current_num_meld == num_meld && current_num_pair == num_pair {
            let distance = get_hand_distance(target_hand, hand);
            match distance.cmp(&upper_bound) {
                Ordering::Less => {
                    upper_bound = distance;
                    necessary_tiles = get_necessary_tiles(target_hand, hand);
                }
                Ordering::Equal => necessary_tiles |= get_necessary_tiles(target_hand, hand),
                Ordering::Greater => (),
            }
        }
        return (upper_bound, necessary_tiles);
    }

    // Sequences cannot be formed with honors.
    for (d, (&m, &n)) in D_TABLE
        .iter()
        .zip(M_TABLE.iter().zip(N_TABLE.iter()))
        .take(3)
    {
        debug_assert!(d.num_sequence == 0);

        if current_num_meld + m > num_meld {
            continue;
        }
        if current_num_pair + d.num_pair > num_pair {
            continue;
        }
        if target_hand[current_rank] + n > 4 {
            // The number of copies of each tile in the hand must not exceed four.
            continue;
        }

        target_hand[current_rank] += n;

        let lower_bound = get_hand_distance(target_hand, hand);
        if lower_bound <= upper_bound {
            let (temp_r, temp_n) = get_zipai_replacement_number(
                hand,
                num_meld,
                num_pair,
                current_rank + 1,
                current_num_meld + m,
                current_num_pair + d.num_pair,
                target_hand,
                upper_bound,
                necessary_tiles,
            );

            match temp_r.cmp(&upper_bound) {
                Ordering::Less => {
                    upper_bound = temp_r;
                    necessary_tiles = temp_n;
                }
                Ordering::Equal => necessary_tiles |= temp_n,
                Ordering::Greater => (),
            }
        }

        target_hand[current_rank] -= n;
    }

    (upper_bound, necessary_tiles)
}

pub(super) fn get_19m_replacement_number(
    hand: &[u8; 9],
    num_meld: u8,
    num_pair: u8,
    current_rank: usize,
    current_num_meld: u8,
    current_num_pair: u8,
    target_hand: &mut [u8; 9],
    mut upper_bound: u8,
    mut necessary_tiles: u16,
) -> (u8, u16) {
    debug_assert!(num_meld <= 4);
    debug_assert!(num_pair <= 1);
    debug_assert!(matches!(current_rank, 0 | 8 | 16));
    debug_assert!(current_num_meld <= num_meld);
    debug_assert!(current_num_pair <= num_pair);
    debug_assert!(target_hand.iter().all(|&c| c <= 4));
    debug_assert!(matches!(necessary_tiles, 0x0 | 0x1 | 0x100 | 0x101));

    if current_rank == 16 {
        if current_num_meld == num_meld && current_num_pair == num_pair {
            let distance = get_hand_distance(target_hand, hand);
            match distance.cmp(&upper_bound) {
                Ordering::Less => {
                    upper_bound = distance;
                    necessary_tiles = get_necessary_tiles(target_hand, hand);
                }
                Ordering::Equal => necessary_tiles |= get_necessary_tiles(target_hand, hand),
                Ordering::Greater => (),
            }
        }
        return (upper_bound, necessary_tiles);
    }

    // Sequences cannot be formed with 1m or 9m in three-player mahjong.
    for (d, (&m, &n)) in D_TABLE
        .iter()
        .zip(M_TABLE.iter().zip(N_TABLE.iter()))
        .take(3)
    {
        debug_assert!(d.num_sequence == 0);

        if current_num_meld + m > num_meld {
            continue;
        }
        if current_num_pair + d.num_pair > num_pair {
            continue;
        }
        if target_hand[current_rank] + n > 4 {
            // The number of copies of each tile in the hand must not exceed four.
            continue;
        }

        target_hand[current_rank] += n;

        let lower_bound = get_hand_distance(target_hand, hand);
        if lower_bound <= upper_bound {
            let (temp_r, temp_n) = get_19m_replacement_number(
                hand,
                num_meld,
                num_pair,
                current_rank + 8,
                current_num_meld + m,
                current_num_pair + d.num_pair,
                target_hand,
                upper_bound,
                necessary_tiles,
            );

            match temp_r.cmp(&upper_bound) {
                Ordering::Less => {
                    upper_bound = temp_r;
                    necessary_tiles = temp_n;
                }
                Ordering::Equal => necessary_tiles |= temp_n,
                Ordering::Greater => (),
            }
        }

        target_hand[current_rank] -= n;
    }

    (upper_bound, necessary_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_shupai_replacement_number_empty() {
        let hand = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut replacement_number = [0u8; 10];
        let mut necessary_tiles = [0u16; 10];
        for num_pair in 0..=1 {
            for num_meld in 0..=4 {
                let mut target = [0u8; 9];
                let (r, n) = get_shupai_replacement_number(
                    &hand,
                    num_meld,
                    num_pair,
                    0,
                    0,
                    0,
                    &mut target,
                    14,
                    0,
                );
                replacement_number[(num_pair * 5 + num_meld) as usize] = r;
                necessary_tiles[(num_pair * 5 + num_meld) as usize] = n;
            }
        }

        assert_eq!(replacement_number[0], 0);
        assert_eq!(replacement_number[1], 3);
        assert_eq!(replacement_number[2], 6);
        assert_eq!(replacement_number[3], 9);
        assert_eq!(replacement_number[4], 12);
        assert_eq!(replacement_number[5], 2);
        assert_eq!(replacement_number[6], 5);
        assert_eq!(replacement_number[7], 8);
        assert_eq!(replacement_number[8], 11);
        assert_eq!(replacement_number[9], 14);

        assert_eq!(necessary_tiles[0], 0);
        assert_eq!(necessary_tiles[1], 0b111111111);
        assert_eq!(necessary_tiles[2], 0b111111111);
        assert_eq!(necessary_tiles[3], 0b111111111);
        assert_eq!(necessary_tiles[4], 0b111111111);
        assert_eq!(necessary_tiles[5], 0b111111111);
        assert_eq!(necessary_tiles[6], 0b111111111);
        assert_eq!(necessary_tiles[7], 0b111111111);
        assert_eq!(necessary_tiles[8], 0b111111111);
        assert_eq!(necessary_tiles[9], 0b111111111);
    }
}
