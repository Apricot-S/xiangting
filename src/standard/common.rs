// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::unpack::UnpackedNumbers;
use crate::tile::TileFlags;
use std::cmp::Ordering;

pub(super) struct Entry {
    pub(super) numbers: UnpackedNumbers,
    pub(super) tiles: [TileFlags; 10],
}

#[inline]
pub(super) fn update_min(
    lhs_number: &mut u8,
    lhs_tiles: &mut TileFlags,
    rhs_number: u8,
    rhs_tiles: TileFlags,
) {
    match (*lhs_number).cmp(&rhs_number) {
        Ordering::Less => (),
        Ordering::Equal => *lhs_tiles |= rhs_tiles,
        Ordering::Greater => {
            *lhs_number = rhs_number;
            *lhs_tiles = rhs_tiles;
        }
    }
}
