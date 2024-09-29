// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::NUM_TILE_INDEX;
use bitvec::prelude::*;

pub(super) type SingleColorTileFlag = BitArr!(for 9);

pub(super) fn to_flag(single_color_bingpai: &[u8]) -> SingleColorTileFlag {
    single_color_bingpai.iter().enumerate().fold(
        SingleColorTileFlag::ZERO,
        |mut flag, (i, &count)| {
            flag.set(i, count > 0);
            flag
        },
    )
}

pub(super) type AllTileFlag = BitArr!(for NUM_TILE_INDEX);

pub(super) fn merge_flags(
    m: SingleColorTileFlag,
    p: SingleColorTileFlag,
    s: SingleColorTileFlag,
    z: SingleColorTileFlag,
) -> AllTileFlag {
    let mut all_tiles = AllTileFlag::ZERO;

    all_tiles[0..9].copy_from_bitslice(&m[0..9]);
    all_tiles[9..18].copy_from_bitslice(&p[0..9]);
    all_tiles[18..27].copy_from_bitslice(&s[0..9]);
    all_tiles[27..34].copy_from_bitslice(&z[0..7]);

    all_tiles
}
