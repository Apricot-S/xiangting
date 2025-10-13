// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::analysis::XiangtingAnalysis;
use crate::shoupai::Shoupai;

pub(in super::super) fn analyze_xiangting(shoupai: &Shoupai) -> XiangtingAnalysis {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return XiangtingAnalysis {
            replacement_number: u8::MAX,
            necessary_tiles: 0,
            unnecessary_tiles: 0,
        };
    }

    let (num_duizi, num_kinds, waits, discards, wait_candidates, discard_candidates) =
        shoupai.bingpai.iter().enumerate().fold(
            (0, 0, 0u64, 0u64, 0u64, 0u64),
            |(num_duizi, num_kinds, waits, discards, wait_candidates, discard_candidates),
             (i, &count)| match count {
                0 => (
                    num_duizi,
                    num_kinds,
                    waits,
                    discards,
                    wait_candidates | (1 << i),
                    discard_candidates,
                ),
                1 => (
                    num_duizi,
                    num_kinds + 1,
                    waits | (1 << i),
                    discards,
                    wait_candidates,
                    discard_candidates | (1 << i),
                ),
                2 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    waits,
                    discards,
                    wait_candidates,
                    discard_candidates,
                ),
                3..=4 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    waits,
                    discards | (1 << i),
                    wait_candidates,
                    discard_candidates,
                ),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let necessary_tiles = if num_kinds < 7 {
        waits | wait_candidates
    } else {
        waits
    };

    let unnecessary_tiles = if num_kinds > 7 {
        discards | discard_candidates
    } else {
        discards
    };

    XiangtingAnalysis {
        replacement_number,
        necessary_tiles,
        unnecessary_tiles,
    }
}

pub(in super::super) fn analyze_xiangting_3_player(shoupai: &Shoupai) -> XiangtingAnalysis {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return XiangtingAnalysis {
            replacement_number: u8::MAX,
            necessary_tiles: 0,
            unnecessary_tiles: 0,
        };
    }

    let (num_duizi, num_kinds, waits, discards, wait_candidates, discard_candidates) = shoupai
        .bingpai
        .iter()
        .enumerate()
        .filter(|(i, _)| !matches!(i, 1..=7))
        .fold(
            (0, 0, 0u64, 0u64, 0u64, 0u64),
            |(num_duizi, num_kinds, waits, discards, wait_candidates, discard_candidates),
             (i, &count)| match count {
                0 => (
                    num_duizi,
                    num_kinds,
                    waits,
                    discards,
                    wait_candidates | (1 << i),
                    discard_candidates,
                ),
                1 => (
                    num_duizi,
                    num_kinds + 1,
                    waits | (1 << i),
                    discards,
                    wait_candidates,
                    discard_candidates | (1 << i),
                ),
                2 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    waits,
                    discards,
                    wait_candidates,
                    discard_candidates,
                ),
                3..=4 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    waits,
                    discards | (1 << i),
                    wait_candidates,
                    discard_candidates,
                ),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let necessary_tiles = if num_kinds < 7 {
        waits | wait_candidates
    } else {
        waits
    };

    let unnecessary_tiles = if num_kinds > 7 {
        discards | discard_candidates
    } else {
        discards
    };

    XiangtingAnalysis {
        replacement_number,
        necessary_tiles,
        unnecessary_tiles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TileCountsExt;
    use crate::tile::TileCounts;
}
