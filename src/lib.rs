// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

// #![warn(missing_docs)]

//! A library for calculating the deficiency number (a.k.a. xiangting number, 向聴数).
//!
//! This library is based on the algorithm in [Cryolite's Nyanten](https://github.com/Cryolite/nyanten).  
//! However, it introduces the following additional features:
//!
//! - Supports rules that include and exclude melded tiles when determining if a hand contains four identical tiles.
//! - Supports three-player mahjong.

#[cfg(not(feature = "build-file"))]
mod tile;

use thiserror::Error;

pub use tile::Tile;

pub(crate) const NUM_TILE_INDEX: usize = 3 * 9 + 4 + 3;

pub type Bingpai = [Tile; NUM_TILE_INDEX];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuluMianzi {}

#[derive(Debug, Error)]
pub enum XiangtingError {
    #[error("hand contains an invalid pure hand: {0}")]
    Bingpai(#[from] BingpaiError),
}

#[derive(Debug, Error)]
pub enum BingpaiError {
    #[error("total tile count must be a multiple of 3 plus 1 or 2 but was {0}")]
    InvalidTileCount(u8),
}

pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_replacement_number_empty_bingpai() {
        let bingpai = [0; 34];
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(0)))
        ));
    }
}
