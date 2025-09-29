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
mod bingpai;
#[cfg(not(feature = "build-file"))]
mod constants;
#[cfg(not(feature = "build-file"))]
mod tile;

#[cfg(all(test, not(feature = "build-file")))]
mod test_utils;

use thiserror::Error;

pub use bingpai::Bingpai;
pub use tile::Tile;

pub(crate) const MAX_NUM_SHOUPAI: u8 = 14;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuluMianzi {}

#[derive(Debug, Error)]
pub enum XiangtingError {
    #[error("hand contains an invalid pure hand: {0}")]
    Bingpai(#[from] BingpaiError),
}

#[derive(Debug, Error)]
pub enum BingpaiError {
    #[error("total tile count must be {max} or less but was {count}")]
    ExceedsMaxTileCount { max: u8, count: u8 },
    #[error("total tile count must be a multiple of 3 plus 1 or 2 but was {0}")]
    InvalidTileCount(u8),
}

pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let num_bingpai: u8 = bingpai.iter().sum();
    if num_bingpai > MAX_NUM_SHOUPAI {
        return Err(XiangtingError::Bingpai(BingpaiError::ExceedsMaxTileCount {
            max: MAX_NUM_SHOUPAI,
            count: num_bingpai,
        }));
    }
    if num_bingpai % 3 == 0 {
        return Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(
            num_bingpai,
        )));
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::Bingpai;
    use super::test_utils::BingpaiExtForTest;
    use super::*;

    #[test]
    fn calculate_replacement_number_err_empty_bingpai() {
        let bingpai = Bingpai::from_code("");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(0)))
        ));
    }

    #[test]
    fn calculate_replacement_number_ok_1_tile_bingpai() {
        let bingpai = Bingpai::from_code("1m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_ok_2_tiles_bingpai() {
        let bingpai = Bingpai::from_code("2p3s");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_err_3_tiles_bingpai() {
        let bingpai = Bingpai::from_code("2p3s7z");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(3)))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_15_tiles_bingpai() {
        let bingpai = Bingpai::from_code("123456789m123456z");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::ExceedsMaxTileCount {
                max: 14,
                count: 15
            }))
        ));
    }
}
