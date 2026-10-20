// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::hash::{hash_19m, hash_shupai, hash_zipai};
use super::lut::{DIRECT_FOUR, DIRECT_THREE, MERGE_ZS, MERGE_ZSP, ZS_PATTERNS, ZSP_PATTERNS};
use super::shupai_map::{SHUPAI_PATTERN_MAP, SHUPAI_PATTERNS};
use super::wanzi_19_map::{WANZI_19_PATTERN_MAP, WANZI_19_PATTERNS};
use super::zipai_map::{ZIPAI_PATTERN_MAP, ZIPAI_PATTERNS};
use crate::bingpai::{Bingpai, PlayerRule};

type Costs = [u8; 10];

pub(super) struct Forward {
    pub(super) hashes: [usize; 4], // man, pin, sou, honors
    pub(super) number: u8,
    ids: [usize; 4],
    zs: usize,
    zsp: usize,
    target: usize,
}

impl Forward {
    #[inline]
    pub(super) fn new<R: PlayerRule>(hand: &Bingpai<R>) -> Self {
        let counts = hand.tile_counts();
        let hz = hash_zipai(&counts[27..]);
        let iz = usize::from(ZIPAI_PATTERN_MAP[hz]);
        let hs = hash_shupai(&counts[18..27]);
        let is = usize::from(SHUPAI_PATTERN_MAP[hs]);
        let zs = usize::from(MERGE_ZS[iz * SHUPAI_PATTERNS.len() + is]);
        let hp = hash_shupai(&counts[9..18]);
        let ip = usize::from(SHUPAI_PATTERN_MAP[hp]);
        let zsp = usize::from(MERGE_ZSP[zs * SHUPAI_PATTERNS.len() + ip]);
        let target = 5 + usize::from(hand.num_required_bingpai_mianzi());
        let (hm, im, number) = if R::IS_THREE_PLAYER {
            let hm = hash_19m(&counts[..9]);
            let im = usize::from(WANZI_19_PATTERN_MAP[hm]);
            (
                hm,
                im,
                DIRECT_THREE[zsp * WANZI_19_PATTERNS.len() + im][target - 5],
            )
        } else {
            let hm = hash_shupai(&counts[..9]);
            let im = usize::from(SHUPAI_PATTERN_MAP[hm]);
            (
                hm,
                im,
                DIRECT_FOUR[zsp * SHUPAI_PATTERNS.len() + im][target - 5],
            )
        };
        Self {
            hashes: [hm, hp, hs, hz],
            number,
            ids: [im, ip, is, iz],
            zs,
            zsp,
            target,
        }
    }

    /// The same optimal states support either mask. Keep *all* equal minima.
    pub(super) fn needed<R: PlayerRule>(&self) -> [u16; 4] {
        let m = if R::IS_THREE_PLAYER {
            &WANZI_19_PATTERNS[self.ids[0]]
        } else {
            &SHUPAI_PATTERNS[self.ids[0]]
        };
        let (zsp, man) = backward_state(&ZSP_PATTERNS[self.zsp], m, self.target, self.number);
        let (zs, pin) = backward(
            &ZS_PATTERNS[self.zs],
            &SHUPAI_PATTERNS[self.ids[1]],
            &ZSP_PATTERNS[self.zsp],
            zsp,
        );
        let (honors, sou) = backward(
            &ZIPAI_PATTERNS[self.ids[3]],
            &SHUPAI_PATTERNS[self.ids[2]],
            &ZS_PATTERNS[self.zs],
            zs,
        );
        [man, pin, sou, honors]
    }
}

/// Project the optimal splits onto each child. Pair correlations are unnecessary
/// because the result is a union of tiles, rather than an enumeration of hands.
fn backward(a: &Costs, b: &Costs, c: &Costs, mut needed: u16) -> (u16, u16) {
    let (mut left, mut right) = (0, 0);
    while needed != 0 {
        let state = needed.trailing_zeros() as usize;
        needed &= needed - 1;
        let (left_states, right_states) = backward_state(a, b, state, c[state]);
        left |= left_states;
        right |= right_states;
    }
    (left, right)
}

#[inline]
#[allow(clippy::needless_range_loop)] // The same split indexes both cost arrays and bitsets.
fn backward_state(a: &Costs, b: &Costs, state: usize, cost: u8) -> (u16, u16) {
    let (mut left, mut right) = (0, 0);
    let melds = state % 5;
    for meld in 0..=melds {
        let other = state - meld;
        if a[meld] + b[other] == cost {
            left |= 1 << meld;
            right |= 1 << other;
        }
    }
    if state >= 5 {
        for meld in 0..=melds {
            let paired = 5 + meld;
            let other = melds - meld;
            if a[paired] + b[other] == cost {
                left |= 1 << paired;
                right |= 1 << other;
            }
        }
    }
    (left, right)
}

pub(super) fn union_masks(masks: &[u16; 10], mut needed: u16) -> u16 {
    let mut result = 0;
    while needed != 0 {
        result |= masks[needed.trailing_zeros() as usize];
        needed &= needed - 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Enumerate the Cartesian product independently of the runtime split loops.
    // Besides the minimum, test all minimizers, including output state zero.
    fn check_costs(a: &Costs, b: &Costs, costs: &Costs, first_state: usize) {
        let mut all_left = 0;
        let mut all_right = 0;
        let mut needed = 0;
        for (state, &cost) in costs.iter().enumerate().skip(first_state) {
            let mut minimum = u8::MAX;
            let (mut expected_left, mut expected_right) = (0, 0);
            for (i, &a_cost) in a.iter().enumerate() {
                for (j, &b_cost) in b.iter().enumerate() {
                    if i / 5 + j / 5 != state / 5 || i % 5 + j % 5 != state % 5 {
                        continue;
                    }
                    let candidate = a_cost + b_cost;
                    if candidate < minimum {
                        minimum = candidate;
                        expected_left = 0;
                        expected_right = 0;
                    }
                    if candidate == minimum {
                        expected_left |= 1 << i;
                        expected_right |= 1 << j;
                    }
                }
            }
            assert_eq!(cost, minimum);
            assert_eq!(
                backward(a, b, costs, 1 << state),
                (expected_left, expected_right)
            );
            all_left |= expected_left;
            all_right |= expected_right;
            needed |= 1 << state;
        }
        assert_eq!(backward(a, b, costs, needed), (all_left, all_right));
        assert_eq!(backward(a, b, costs, 0), (0, 0));
    }

    fn check_table(left: &[Costs], right: &[Costs], output: &[Costs], table: &[u8]) {
        for (left_id, a) in left.iter().enumerate() {
            for (right_id, b) in right.iter().enumerate() {
                let costs = &output[usize::from(table[left_id * right.len() + right_id])];
                check_costs(a, b, costs, 0);
            }
        }
    }

    fn check_final_table(left: &[Costs], right: &[Costs], table: &[[u8; 5]]) {
        for (left_id, a) in left.iter().enumerate() {
            for (right_id, b) in right.iter().enumerate() {
                let mut costs = [0; 10];
                costs[5..].copy_from_slice(&table[left_id * right.len() + right_id]);
                check_costs(a, b, &costs, 5);
            }
        }
    }

    #[test]
    fn every_lut_entry_and_all_optimal_predecessors() {
        check_table(&ZIPAI_PATTERNS, &SHUPAI_PATTERNS, &ZS_PATTERNS, &MERGE_ZS);
        check_table(&ZS_PATTERNS, &SHUPAI_PATTERNS, &ZSP_PATTERNS, &MERGE_ZSP);
        check_final_table(&ZSP_PATTERNS, &SHUPAI_PATTERNS, &DIRECT_FOUR);
        check_final_table(&ZSP_PATTERNS, &WANZI_19_PATTERNS, &DIRECT_THREE);
    }
}
