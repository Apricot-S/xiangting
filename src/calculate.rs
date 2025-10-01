// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::Tile;
use crate::bingpai::{Bingpai, BingpaiError, BingpaiExt};
use crate::fulu_mianzi::{ClaimedTilePosition, FuluMianzi, FuluMianziError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShoupaiError {
    #[error("tile {tile} count must be 4 or less but was {count}")]
    TooManyCopies { tile: Tile, count: u8 },
}

#[derive(Debug, Error)]
pub enum XiangtingError {
    #[error("hand contains an invalid pure hand: {0}")]
    InvalidBingpai(#[from] BingpaiError),
    #[error("hand contains an invalid meld: {0}")]
    InvalidFuluMianzi(#[from] FuluMianziError),
    #[error("hand contains an invalid combination of pure hand and melds: {0}")]
    InvalidShoupai(#[from] ShoupaiError),
}

pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let num_bingpai = bingpai.count()?;
    if let Some(fl) = fulu_mianzi_list {
        fl.iter().try_for_each(|f| f.validate())?;
    }

    let fulupai = fulu_mianzi_list.map(|fl| {
        fl.iter().fold([0u8; 34], |mut fulupai, m| {
            match m {
                FuluMianzi::Shunzi(t, ClaimedTilePosition::Low) => {
                    fulupai[*t as usize] += 1;
                    fulupai[(t + 1) as usize] += 1;
                    fulupai[(t + 2) as usize] += 1;
                }
                FuluMianzi::Shunzi(t, ClaimedTilePosition::Middle) => {
                    fulupai[(t - 1) as usize] += 1;
                    fulupai[*t as usize] += 1;
                    fulupai[(t + 1) as usize] += 1;
                }
                FuluMianzi::Shunzi(t, ClaimedTilePosition::High) => {
                    fulupai[(t - 2) as usize] += 1;
                    fulupai[(t - 1) as usize] += 1;
                    fulupai[*t as usize] += 1;
                }
                FuluMianzi::Kezi(t) => {
                    fulupai[*t as usize] += 3;
                }
                FuluMianzi::Gangzi(t) => {
                    fulupai[*t as usize] += 4;
                }
            }
            fulupai
        })
    });

    let shoupai: Bingpai = fulupai.map_or_else(
        || *bingpai,
        |fp| std::array::from_fn(|i| bingpai[i] + fp[i]),
    );
    shoupai
        .iter()
        .enumerate()
        .find(|(_, c)| **c > 4)
        .map(|(i, &c)| ShoupaiError::TooManyCopies {
            tile: i as Tile,
            count: c,
        })
        .map_or(Ok(()), Err)?;

    Ok(0)
}

pub fn calculate_replacement_number_3_player(
    bingpai: &Bingpai,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let num_bingpai = bingpai.count_3_player()?;
    if let Some(fl) = fulu_mianzi_list {
        fl.iter().try_for_each(|f| f.validate_3_player())?;
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bingpai;
    use crate::ClaimedTilePosition;
    use crate::FuluMianzi;
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

    // #[test]
    // fn calculate_replacement_number_err_shoupai_15_tiles() {
    //     // TODO: 副露が手牌に対して多くないかのエラーに変更する
    //     let bingpai = Bingpai::from_code("11122233344455m");
    //     let fulu_mianzi_list = [FuluMianzi::Kezi(5)];
    //     let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
    //     assert!(matches!(
    //         replacement_number,
    //         Err(XiangtingError::Shoupai(ShoupaiError::TooManyTiles {
    //             max: 14,
    //             count: 15
    //         }))
    //     ));
    // }

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
