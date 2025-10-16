// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::shoupai::Shoupai;
use crate::tile::TileFlags;

pub(in super::super) fn calculate_necessary_tiles(shoupai: &Shoupai) -> (u8, TileFlags) {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return (u8::MAX, 0);
    }

    const YAOJIUPAI_INDICES: [usize; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];
    let (num_kinds, has_jiangpai, waits, wait_candidates) = YAOJIUPAI_INDICES
        .iter()
        .map(|&i| &shoupai.bingpai[i])
        .enumerate()
        .fold(
            (0, false, 0u64, 0u64),
            |(num_kinds, has_jiangpai, waits, wait_candidates), (i, &count)| match count {
                0 => (num_kinds, has_jiangpai, waits | (1 << i), wait_candidates),
                1 => (
                    num_kinds + 1,
                    has_jiangpai,
                    waits,
                    wait_candidates | (1 << i),
                ),
                2..=4 => (num_kinds + 1, true, waits, wait_candidates),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 14 - num_kinds - (has_jiangpai as u8);

    let necessary_tiles = if has_jiangpai {
        waits
    } else {
        waits | wait_candidates
    };

    (replacement_number, necessary_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::tile::{TileCounts, TileFlags};
}
