// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::shoupai::{Shoupai, Shoupai3Player};
use crate::tile::TileFlags;

pub(in super::super) fn calculate_unnecessary_tiles(shoupai: &Shoupai) -> (u8, TileFlags) {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, discards, discard_candidates) =
        shoupai.bingpai.iter().enumerate().fold(
            (0, 0, 0u64, 0u64),
            |(num_duizi, num_kinds, discards, discard_candidates), (i, &count)| match count {
                0 => (num_duizi, num_kinds, discards, discard_candidates),
                1 => (
                    num_duizi,
                    num_kinds + 1,
                    discards,
                    discard_candidates | (1 << i),
                ),
                2 => (num_duizi + 1, num_kinds + 1, discards, discard_candidates),
                3..=4 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    discards | (1 << i),
                    discard_candidates,
                ),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let unnecessary_tiles = if num_kinds > 7 {
        discards | discard_candidates
    } else {
        discards
    };

    (replacement_number, unnecessary_tiles)
}

pub(in super::super) fn calculate_unnecessary_tiles_3_player(
    shoupai: &Shoupai3Player,
) -> (u8, TileFlags) {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, discards, discard_candidates) = shoupai
        .bingpai
        .iter()
        .enumerate()
        .filter(|(i, _)| !matches!(i, 1..=7))
        .fold(
            (0, 0, 0u64, 0u64),
            |(num_duizi, num_kinds, discards, discard_candidates), (i, &count)| match count {
                0 => (num_duizi, num_kinds, discards, discard_candidates),
                1 => (
                    num_duizi,
                    num_kinds + 1,
                    discards,
                    discard_candidates | (1 << i),
                ),
                2 => (num_duizi + 1, num_kinds + 1, discards, discard_candidates),
                3..=4 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    discards | (1 << i),
                    discard_candidates,
                ),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let unnecessary_tiles = if num_kinds > 7 {
        discards | discard_candidates
    } else {
        discards
    };

    (replacement_number, unnecessary_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TileCountsExt;
    use crate::tile::TileCounts;
}
