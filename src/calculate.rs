// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::Bingpai;
use crate::fulu_mianzi::FuluMianzi;
use crate::shoupai::{Shoupai, Shoupai3Player, XiangtingError};

pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let shoupai = Shoupai::new(bingpai, fulu_mianzi_list)?;

    Ok(0)
}

pub fn calculate_replacement_number_3_player(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let shoupai = Shoupai3Player::new(bingpai, fulu_mianzi_list)?;

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::BingpaiError;
    use crate::fulu_mianzi::{ClaimedTilePosition, FuluMianziError};
    use crate::shoupai::{ShoupaiError, XiangtingError};
    use crate::test_utils::BingpaiExtForTest;

    #[test]
    fn calculate_replacement_number_err_bingpai_empty() {
        let bingpai = Bingpai::from_code("");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidBingpai(
                BingpaiError::InvalidTileCount(0)
            ))
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
            Err(XiangtingError::InvalidBingpai(
                BingpaiError::InvalidTileCount(3)
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_15_tiles() {
        let bingpai = Bingpai::from_code("111222333444555m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidBingpai(BingpaiError::TooManyTiles(
                15
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_5_same_tiles() {
        let bingpai = Bingpai::from_code("11111m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidBingpai(
                BingpaiError::TooManyCopies { tile: 0, count: 5 }
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_fulu_index_out_of_range() {
        let bingpai = Bingpai::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(34)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidFuluMianzi(
                FuluMianziError::IndexOutOfRange(34)
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_fulu_shunzi_with_zipai() {
        let bingpai = Bingpai::from_code("1p");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(27, ClaimedTilePosition::Low)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidFuluMianzi(
                FuluMianziError::ShunziWithZipai(27)
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_fulu_invalid_shunzi_combination() {
        let bingpai = Bingpai::from_code("1p");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(0, ClaimedTilePosition::Middle)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidFuluMianzi(
                FuluMianziError::InvalidShunziCombination(0, ClaimedTilePosition::Middle)
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_shoupai_too_many_fulu_mianzi() {
        let bingpai = Bingpai::from_code("11122233344455m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(5)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidShoupai(
                ShoupaiError::TooManyFuluMianzi { max: 0, count: 1 }
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_shoupai_5_same_tiles() {
        let bingpai = Bingpai::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(0)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidShoupai(
                ShoupaiError::TooManyCopies { tile: 0, count: 5 }
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_err_bingpai_2m() {
        let bingpai = Bingpai::from_code("2m");
        let replacement_number = calculate_replacement_number_3_player(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidBingpai(
                BingpaiError::InvalidTileFor3Player(1)
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_err_fulu_123p() {
        let bingpai = Bingpai::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(9, ClaimedTilePosition::Low)];
        let replacement_number =
            calculate_replacement_number_3_player(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidFuluMianzi(
                FuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Shunzi(
                    9,
                    ClaimedTilePosition::Low
                ))
            ))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_err_fulu_222m() {
        let bingpai = Bingpai::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(1)];
        let replacement_number =
            calculate_replacement_number_3_player(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::InvalidFuluMianzi(
                FuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Kezi(1))
            ))
        ));
    }
}
