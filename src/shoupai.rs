// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::{Bingpai, BingpaiExt, InvalidBingpaiError};
use crate::constants::{MAX_NUM_FULU_MIANZI, MAX_NUM_SAME_TILE, MAX_NUM_SHOUPAI, NUM_TILE_INDEX};
use crate::fulu_mianzi::{ClaimedTilePosition, FuluMianzi, InvalidFuluMianziError};
use thiserror::Error;

/// List of melds.
///
/// A element of array represents a meld in the hand.
/// Specify [`None`] for the element where there is no meld.
///
/// # Examples
///
/// ```
/// # use xiangting::{ClaimedTilePosition, FuluMianziList, FuluMianzi};
/// // 456p 7777s 111z
/// let melds: FuluMianziList = [
///     Some(FuluMianzi::Shunzi(12, ClaimedTilePosition::Low)),
///     Some(FuluMianzi::Gangzi(24)),
///     Some(FuluMianzi::Kezi(27)),
///     None,
/// ];
/// ```
pub type FuluMianziList = [Option<FuluMianzi>; MAX_NUM_FULU_MIANZI];

pub(crate) trait FuluMianziListExt {
    fn to_fulupai(&self) -> Bingpai;
}

impl FuluMianziListExt for FuluMianziList {
    fn to_fulupai(&self) -> Bingpai {
        self.iter()
            .flatten()
            .fold([0; NUM_TILE_INDEX], |mut fulupai, m| {
                match m {
                    FuluMianzi::Shunzi(tile, ClaimedTilePosition::Low) => {
                        fulupai[*tile as usize] += 1;
                        fulupai[(tile + 1) as usize] += 1;
                        fulupai[(tile + 2) as usize] += 1;
                    }
                    FuluMianzi::Shunzi(tile, ClaimedTilePosition::Middle) => {
                        fulupai[(tile - 1) as usize] += 1;
                        fulupai[*tile as usize] += 1;
                        fulupai[(tile + 1) as usize] += 1;
                    }
                    FuluMianzi::Shunzi(tile, ClaimedTilePosition::High) => {
                        fulupai[(tile - 2) as usize] += 1;
                        fulupai[(tile - 1) as usize] += 1;
                        fulupai[*tile as usize] += 1;
                    }
                    FuluMianzi::Kezi(tile) => {
                        fulupai[*tile as usize] += 3;
                    }
                    FuluMianzi::Gangzi(tile) => {
                        fulupai[*tile as usize] += 4;
                    }
                }
                fulupai
            })
    }
}

/// Errors that occur when an invalid hand (手牌) is provided.
#[derive(Debug, Error)]
pub enum InvalidShoupaiError {
    /// Contains an invalid pure hand.
    #[error("hand contains an invalid pure hand ({0})")]
    InvalidBingpai(#[from] InvalidBingpaiError),
    /// Contains an invalid meld.
    #[error("hand contains an invalid meld ({0})")]
    InvalidFuluMianzi(#[from] InvalidFuluMianziError),
    /// Same tile count exceeds 4.
    #[error("same tile count must be 4 or less but was {0}")]
    ExceedsMaxNumSameTile(u8),
    /// Total tile count exceeds 14.
    #[error("total tile count must be 14 or less but was {0}")]
    ExceedsMaxNumShoupai(u8),
}

fn validate_fulu_mianzi_list(fulu_mianzi_list: &FuluMianziList) -> Result<(), InvalidShoupaiError> {
    fulu_mianzi_list
        .iter()
        .flatten()
        .try_for_each(|m| m.validate())?;
    Ok(())
}

fn validate_fulu_mianzi_list_3_player(
    fulu_mianzi_list: &FuluMianziList,
) -> Result<(), InvalidShoupaiError> {
    fulu_mianzi_list
        .iter()
        .flatten()
        .try_for_each(|m| m.validate_3_player())?;
    Ok(())
}

fn count_gangzi(fulu_mianzi_list: &FuluMianziList) -> u8 {
    fulu_mianzi_list
        .iter()
        .flatten()
        .filter(|m| matches!(*m, FuluMianzi::Gangzi(_)))
        .count() as u8
}

fn merge_bingpai_and_fulupai(bingpai: &Bingpai, fulupai: &Bingpai) -> Bingpai {
    std::array::from_fn(|i| bingpai[i] + fulupai[i])
}

fn validate_shoupai(shoupai: &Bingpai, num_gangzi: u8) -> Result<(), InvalidShoupaiError> {
    let num_shoupai = shoupai.iter().try_fold(0, |acc, &num_tile| {
        if num_tile > MAX_NUM_SAME_TILE {
            Err(InvalidShoupaiError::ExceedsMaxNumSameTile(num_tile))
        } else {
            Ok(acc + num_tile)
        }
    })?;

    if num_shoupai > (MAX_NUM_SHOUPAI + num_gangzi) {
        return Err(InvalidShoupaiError::ExceedsMaxNumShoupai(num_shoupai));
    }

    Ok(())
}

pub(crate) fn get_shoupai(
    bingpai: &Bingpai,
    fulu_mianzi_list: &FuluMianziList,
) -> Result<Bingpai, InvalidShoupaiError> {
    debug_assert!(bingpai.count().is_ok());

    validate_fulu_mianzi_list(fulu_mianzi_list)?;

    let fulupai = fulu_mianzi_list.to_fulupai();
    let shoupai = merge_bingpai_and_fulupai(bingpai, &fulupai);
    let num_gangzi = count_gangzi(fulu_mianzi_list);
    validate_shoupai(&shoupai, num_gangzi)?;

    Ok(shoupai)
}

pub(crate) fn get_shoupai_3_player(
    bingpai: &Bingpai,
    fulu_mianzi_list: &FuluMianziList,
) -> Result<Bingpai, InvalidShoupaiError> {
    debug_assert!(bingpai.count_3_player().is_ok());

    validate_fulu_mianzi_list_3_player(fulu_mianzi_list)?;

    let fulupai = fulu_mianzi_list.to_fulupai();
    let shoupai = merge_bingpai_and_fulupai(bingpai, &fulupai);
    let num_gangzi = count_gangzi(fulu_mianzi_list);
    validate_shoupai(&shoupai, num_gangzi)?;

    Ok(shoupai)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fulu_mianzi::FuluMianzi;

    #[test]
    fn to_fulupai_menqian() {
        let fulupai_menqian_1: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let fulupai_menqian_2 = menqian.to_fulupai();
        assert_eq!(fulupai_menqian_1, fulupai_menqian_2);
    }

    #[test]
    fn to_fulupai_3_fulu() {
        let fulupai_3_chi_1: Bingpai = [
            2, 2, 2, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let shunzi_3 = [
            Some(FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(9, ClaimedTilePosition::Low)),
            None,
        ];
        let fulupai_3_chi_2 = shunzi_3.to_fulupai();
        assert_eq!(fulupai_3_chi_1, fulupai_3_chi_2);

        let fulupai_3_peng_1: Bingpai = [
            0, 3, 3, 3, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let kezi_3 = [
            None,
            Some(FuluMianzi::Kezi(1)),
            Some(FuluMianzi::Kezi(2)),
            Some(FuluMianzi::Kezi(3)),
        ];
        let fulupai_3_peng_2 = kezi_3.to_fulupai();
        assert_eq!(fulupai_3_peng_1, fulupai_3_peng_2);

        let fulupai_3_gang_1: Bingpai = [
            4, 0, 4, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let gangzi_3 = [
            Some(FuluMianzi::Gangzi(0)),
            None,
            Some(FuluMianzi::Gangzi(2)),
            Some(FuluMianzi::Gangzi(3)),
        ];
        let fulupai_3_gang_2 = gangzi_3.to_fulupai();
        assert_eq!(fulupai_3_gang_1, fulupai_3_gang_2);
    }

    #[test]
    fn to_fulupai_4_fulu() {
        let fulupai_4_chi_1: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let shunzi_4 = [
            Some(FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(3, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(6, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(9, ClaimedTilePosition::Low)),
        ];
        let fulupai_4_chi_2 = shunzi_4.to_fulupai();
        assert_eq!(fulupai_4_chi_1, fulupai_4_chi_2);

        let fulupai_4_peng_1: Bingpai = [
            3, 3, 3, 3, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let kezi_4 = [
            Some(FuluMianzi::Kezi(0)),
            Some(FuluMianzi::Kezi(1)),
            Some(FuluMianzi::Kezi(2)),
            Some(FuluMianzi::Kezi(3)),
        ];
        let fulupai_4_peng_2 = kezi_4.to_fulupai();
        assert_eq!(fulupai_4_peng_1, fulupai_4_peng_2);

        let fulupai_4_gang_1: Bingpai = [
            4, 4, 4, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let gangzi_4 = [
            Some(FuluMianzi::Gangzi(0)),
            Some(FuluMianzi::Gangzi(1)),
            Some(FuluMianzi::Gangzi(2)),
            Some(FuluMianzi::Gangzi(3)),
        ];
        let fulupai_4_gang_2 = gangzi_4.to_fulupai();
        assert_eq!(fulupai_4_gang_1, fulupai_4_gang_2);
    }

    #[test]
    fn valid_shoupai_menqian() {
        let bingpai: Bingpai = [
            1, 1, 1, 1, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        assert_eq!(get_shoupai(&bingpai, &menqian).unwrap(), bingpai);
    }

    #[test]
    fn valid_shoupai_fulu() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];

        let kezi_4 = [
            Some(FuluMianzi::Kezi(0)),
            Some(FuluMianzi::Kezi(1)),
            Some(FuluMianzi::Kezi(2)),
            Some(FuluMianzi::Kezi(3)),
        ];
        let shoupai_kezi_4: Bingpai = [
            3, 3, 3, 3, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];
        assert_eq!(get_shoupai(&bingpai, &kezi_4).unwrap(), shoupai_kezi_4);

        let gangzi_4 = [
            Some(FuluMianzi::Gangzi(0)),
            Some(FuluMianzi::Gangzi(1)),
            Some(FuluMianzi::Gangzi(2)),
            Some(FuluMianzi::Gangzi(3)),
        ];
        let shoupai_gangzi_4: Bingpai = [
            4, 4, 4, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];
        assert_eq!(get_shoupai(&bingpai, &gangzi_4).unwrap(), shoupai_gangzi_4);
    }

    #[test]
    fn invalid_shoupai_fulu_too_many_tiles() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 0, 0, 0, // z
        ];

        let kezi_4 = [
            Some(FuluMianzi::Kezi(0)),
            Some(FuluMianzi::Kezi(1)),
            Some(FuluMianzi::Kezi(2)),
            Some(FuluMianzi::Kezi(3)),
        ];
        let result = get_shoupai(&bingpai, &kezi_4).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumShoupai(16),
        ));
    }

    #[test]
    fn invalid_shoupai_fulu_5th_tile() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];

        let gangzi_4 = [
            Some(FuluMianzi::Gangzi(0)),
            Some(FuluMianzi::Gangzi(1)),
            Some(FuluMianzi::Gangzi(2)),
            Some(FuluMianzi::Gangzi(2)),
        ];
        let result = get_shoupai(&bingpai, &gangzi_4).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumSameTile(8),
        ));
    }

    #[test]
    fn invalid_shoupai_fulu_invalid_mianzi() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 0, 0, 0, // z
        ];

        let shunzi_3 = [
            Some(FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(27, ClaimedTilePosition::Low)),
            None,
        ];
        let result = get_shoupai(&bingpai, &shunzi_3).unwrap_err();
        assert!(matches!(result, InvalidShoupaiError::InvalidFuluMianzi(_)));
    }

    #[test]
    fn valid_shoupai_3_player_fulu() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 0, // z
        ];

        let kezi_4 = [
            Some(FuluMianzi::Kezi(0)),
            Some(FuluMianzi::Kezi(8)),
            Some(FuluMianzi::Kezi(9)),
            Some(FuluMianzi::Kezi(33)),
        ];
        let shoupai_kezi_4: Bingpai = [
            3, 0, 0, 0, 0, 0, 0, 0, 3, // m
            3, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 3, // z
        ];
        assert_eq!(
            get_shoupai_3_player(&bingpai, &kezi_4).unwrap(),
            shoupai_kezi_4
        );

        let gangzi_4 = [
            Some(FuluMianzi::Gangzi(0)),
            Some(FuluMianzi::Gangzi(8)),
            Some(FuluMianzi::Gangzi(9)),
            Some(FuluMianzi::Gangzi(33)),
        ];
        let shoupai_gangzi_4: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 4, // m
            4, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 0, 0, 0, 0, 4, // z
        ];
        assert_eq!(
            get_shoupai_3_player(&bingpai, &gangzi_4).unwrap(),
            shoupai_gangzi_4
        );
    }

    #[test]
    fn invalid_shoupai_3_player_fulu_shunzi() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            3, 3, 3, 2, 0, 0, 0, // z
        ];

        let shunzi_1 = [
            Some(FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)),
            None,
            None,
            None,
        ];
        let result = get_shoupai_3_player(&bingpai, &shunzi_1).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidFuluMianzi(
                InvalidFuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Shunzi(
                    0,
                    ClaimedTilePosition::Low
                ))
            )
        ));
    }

    #[test]
    fn invalid_shoupai_3_player_fulu_invalid_kezi() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            3, 3, 3, 2, 0, 0, 0, // z
        ];

        let kezi_2m = [Some(FuluMianzi::Kezi(1)), None, None, None];
        let result = get_shoupai_3_player(&bingpai, &kezi_2m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidFuluMianzi(
                InvalidFuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Kezi(1))
            )
        ));

        let kezi_8m = [Some(FuluMianzi::Kezi(7)), None, None, None];
        let result = get_shoupai_3_player(&bingpai, &kezi_8m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidFuluMianzi(
                InvalidFuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Kezi(7))
            )
        ));
    }

    #[test]
    fn invalid_shoupai_3_player_fulu_invalid_gangzi() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            3, 3, 3, 2, 0, 0, 0, // z
        ];

        let gangzi_2m = [Some(FuluMianzi::Gangzi(1)), None, None, None];
        let result = get_shoupai_3_player(&bingpai, &gangzi_2m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidFuluMianzi(
                InvalidFuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Gangzi(1))
            )
        ));

        let gangzi_8m = [Some(FuluMianzi::Gangzi(7)), None, None, None];
        let result = get_shoupai_3_player(&bingpai, &gangzi_8m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidFuluMianzi(
                InvalidFuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Gangzi(7))
            )
        ));
    }
}
