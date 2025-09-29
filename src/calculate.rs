// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::{Bingpai, BingpaiError, BingpaiExt};
use crate::fulu_mianzi::{FuluMianzi, FuluMianziError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XiangtingError {
    #[error("hand contains an invalid pure hand: {0}")]
    Bingpai(#[from] BingpaiError),
    #[error("hand contains an invalid meld: {0}")]
    FuluMianzi(#[from] FuluMianziError),
}

pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let num_bingpai = bingpai.count()?;
    if let Some(fl) = fulu_mianzi_list {
        for f in fl {
            f.to_tile_counts()?;
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bingpai;
    use crate::FuluMianzi;
    use crate::test_utils::BingpaiExtForTest;

    #[test]
    fn calculate_replacement_number_err_bingpai_empty() {
        let bingpai = Bingpai::from_code("");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(0)))
        ));
    }

    #[test]
    fn calculate_replacement_number_ok_bingpai_1_tile() {
        let bingpai = Bingpai::from_code("1m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_ok_bingpai_2_tiles() {
        let bingpai = Bingpai::from_code("2p3s");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_3_tiles() {
        let bingpai = Bingpai::from_code("2p3s7z");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(3)))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_5_same_tiles() {
        let bingpai = Bingpai::from_code("11111m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::TooManyCopies {
                tile: 0,
                count: 5
            }))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_fulu_index_out_of_range() {
        let bingpai = Bingpai::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(34)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::FuluMianzi(
                FuluMianziError::IndexOutOfRange(34)
            ))
        ));
    }
}
