// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::{Bingpai, BingpaiError, BingpaiExt};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuluMianzi {}

#[derive(Debug, Error)]
pub enum XiangtingError {
    #[error("hand contains an invalid pure hand: {0}")]
    Bingpai(#[from] BingpaiError),
}

pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let num_bingpai = bingpai.count()?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bingpai;
    use crate::test_utils::BingpaiExtForTest;

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
            Err(XiangtingError::Bingpai(BingpaiError::TooManyTiles {
                max: 14,
                count: 15
            }))
        ));
    }
}
