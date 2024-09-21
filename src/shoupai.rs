// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::bingpai::Bingpai;
use super::constants::{MAX_NUM_FULU_MIANZI, MAX_NUM_SAME_TILE, MAX_NUM_SHOUPAI, NUM_TILE_INDEX};
use super::mianzi::{ClaimedTilePosition, InvalidMianziError, Mianzi};
use thiserror::Error;

/// List of melds.
///
/// A element of array indicates a meld in the hand.
/// Specify [`None`] for the element where there is no meld.
///
/// # Examples
///
/// ```
/// # use xiangting::{ClaimedTilePosition, FuluMianziList, Mianzi};
/// // 456p 7777s 111z
/// let melds: FuluMianziList = [
///     Some(Mianzi::Shunzi(12, ClaimedTilePosition::Low)),
///     Some(Mianzi::Gangzi(24)),
///     Some(Mianzi::Kezi(27)),
///     None,
/// ];
/// ```
pub type FuluMianziList = [Option<Mianzi>; MAX_NUM_FULU_MIANZI];

pub(super) fn count_fulupai(fulu_mianzi_list: &FuluMianziList) -> Bingpai {
    fulu_mianzi_list
        .iter()
        .fold([0; NUM_TILE_INDEX], |mut fulupai, m| {
            match m {
                Some(Mianzi::Shunzi(tile, position)) => {
                    fulupai[*tile as usize] += 1;
                    match position {
                        ClaimedTilePosition::Low => {
                            fulupai[(*tile + 1) as usize] += 1;
                            fulupai[(*tile + 2) as usize] += 1;
                        }
                        ClaimedTilePosition::Middle => {
                            fulupai[(*tile - 1) as usize] += 1;
                            fulupai[(*tile + 1) as usize] += 1;
                        }
                        ClaimedTilePosition::High => {
                            fulupai[(*tile - 2) as usize] += 1;
                            fulupai[(*tile - 1) as usize] += 1;
                        }
                    };
                }
                Some(Mianzi::Kezi(tile)) => {
                    fulupai[*tile as usize] += 3;
                }
                Some(Mianzi::Gangzi(tile)) => {
                    fulupai[*tile as usize] += 4;
                }
                None => (),
            }
            fulupai
        })
}

#[derive(Debug, Error)]
pub enum InvalidShoupaiError {
    #[error("Invalid hand: Same tile count exceeds 4 ({0}).")]
    ExceedsMaxNumSameTile(u8),
    #[error("Invalid hand: Total tile count exceeds 14 ({0}).")]
    ExceedsMaxNumShoupai(u8),
    #[error("Invalid hand: Total tile count is not a multiple of 3 plus 1 or 2 ({0}).")]
    InvalidNumShoupai(u8),
    #[error("InvalidMianziError({0})")]
    InvalidMianzi(#[from] InvalidMianziError),
    #[error("Invalid hand: {0} cannot be used in 3-player mahjong.")]
    InvalidMianziFor3Player(Mianzi),
}

pub(super) fn validate_shoupai(
    bingpai: &Bingpai,
    fulu_mianzi_list: &FuluMianziList,
) -> Result<(), InvalidShoupaiError> {
    fulu_mianzi_list
        .iter()
        .flatten()
        .try_for_each(|m| m.validate())?;

    let num_gangzi = fulu_mianzi_list
        .iter()
        .flatten()
        .filter(|m| matches!(*m, Mianzi::Gangzi(_)))
        .count() as u8;

    let mut shoupai = *bingpai;
    let fulupai = count_fulupai(fulu_mianzi_list);
    shoupai
        .iter_mut()
        .zip(fulupai.iter())
        .for_each(|(s, &f)| *s += f);

    let num_shoupai = shoupai.iter().try_fold(0, |acc, &num_tile| {
        if num_tile > MAX_NUM_SAME_TILE {
            return Err(InvalidShoupaiError::ExceedsMaxNumSameTile(num_tile));
        }
        Ok(acc + num_tile)
    })?;

    if num_shoupai > (MAX_NUM_SHOUPAI + num_gangzi) {
        return Err(InvalidShoupaiError::ExceedsMaxNumShoupai(num_shoupai));
    }
    if (num_shoupai - num_gangzi) % 3 == 0 {
        return Err(InvalidShoupaiError::InvalidNumShoupai(num_shoupai));
    }

    Ok(())
}

pub(super) fn validate_shoupai_3_player(
    bingpai: &Bingpai,
    fulu_mianzi_list: &FuluMianziList,
) -> Result<(), InvalidShoupaiError> {
    fulu_mianzi_list
        .iter()
        .flatten()
        .try_for_each(|m| match m {
            Mianzi::Shunzi(_, _) => Err(InvalidShoupaiError::InvalidMianziFor3Player(m.clone())),
            Mianzi::Kezi(t) | Mianzi::Gangzi(t) => {
                if (1..8).contains(t) {
                    Err(InvalidShoupaiError::InvalidMianziFor3Player(m.clone()))
                } else {
                    Ok(())
                }
            }
        })?;

    validate_shoupai(bingpai, fulu_mianzi_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mianzi::Mianzi;

    #[test]
    fn count_fulupai_menqian() {
        let fulupai_menqian_1: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let fulupai_menqian_2 = count_fulupai(&menqian);
        assert_eq!(fulupai_menqian_1, fulupai_menqian_2);
    }

    #[test]
    fn count_fulupai_3_fulu() {
        let fulupai_3_chi_1: Bingpai = [
            2, 2, 2, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let shunzi_3 = [
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(9, ClaimedTilePosition::Low)),
            None,
        ];
        let fulupai_3_chi_2 = count_fulupai(&shunzi_3);
        assert_eq!(fulupai_3_chi_1, fulupai_3_chi_2);

        let fulupai_3_peng_1: Bingpai = [
            0, 3, 3, 3, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let kezi_3 = [
            None,
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        let fulupai_3_peng_2 = count_fulupai(&kezi_3);
        assert_eq!(fulupai_3_peng_1, fulupai_3_peng_2);

        let fulupai_3_gang_1: Bingpai = [
            4, 0, 4, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let gangzi_3 = [
            Some(Mianzi::Gangzi(0)),
            None,
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(3)),
        ];
        let fulupai_3_gang_2 = count_fulupai(&gangzi_3);
        assert_eq!(fulupai_3_gang_1, fulupai_3_gang_2);
    }

    #[test]
    fn count_fulupai_4_fulu() {
        let fulupai_4_chi_1: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let shunzi_4 = [
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(3, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(6, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(9, ClaimedTilePosition::Low)),
        ];
        let fulupai_4_chi_2 = count_fulupai(&shunzi_4);
        assert_eq!(fulupai_4_chi_1, fulupai_4_chi_2);

        let fulupai_4_peng_1: Bingpai = [
            3, 3, 3, 3, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let kezi_4 = [
            Some(Mianzi::Kezi(0)),
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        let fulupai_4_peng_2 = count_fulupai(&kezi_4);
        assert_eq!(fulupai_4_peng_1, fulupai_4_peng_2);

        let fulupai_4_gang_1: Bingpai = [
            4, 4, 4, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let gangzi_4 = [
            Some(Mianzi::Gangzi(0)),
            Some(Mianzi::Gangzi(1)),
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(3)),
        ];
        let fulupai_4_gang_2 = count_fulupai(&gangzi_4);
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
        assert_eq!(validate_shoupai(&bingpai, &menqian).unwrap(), ());
    }

    #[test]
    fn invalid_shoupai_menqian_too_many_tiles() {
        let bingpai: Bingpai = [
            1, 1, 1, 1, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let result = validate_shoupai(&bingpai, &menqian).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumShoupai(15)
        ));
    }

    #[test]
    fn invalid_shoupai_menqian_5th_tile() {
        let bingpai: Bingpai = [
            5, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 0, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let result = validate_shoupai(&bingpai, &menqian).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumSameTile(5)
        ));
    }

    #[test]
    fn invalid_shoupai_menqian_incomplete_hand() {
        let bingpai: Bingpai = [
            4, 4, 4, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let menqian = [None, None, None, None];
        let result = validate_shoupai(&bingpai, &menqian).unwrap_err();
        assert!(matches!(result, InvalidShoupaiError::InvalidNumShoupai(12)));
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
            Some(Mianzi::Kezi(0)),
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        assert_eq!(validate_shoupai(&bingpai, &kezi_4).unwrap(), ());

        let gangzi_4 = [
            Some(Mianzi::Gangzi(0)),
            Some(Mianzi::Gangzi(1)),
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(3)),
        ];
        assert_eq!(validate_shoupai(&bingpai, &gangzi_4).unwrap(), ());
    }

    #[test]
    fn invalid_shoupai_fulu_too_many_tiles() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];

        let kezi_4 = [
            Some(Mianzi::Kezi(0)),
            Some(Mianzi::Kezi(1)),
            Some(Mianzi::Kezi(2)),
            Some(Mianzi::Kezi(3)),
        ];
        let result = validate_shoupai(&bingpai, &kezi_4).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumShoupai(15),
        ));
    }

    #[test]
    fn invalid_shoupai_fulu_5th_tile() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];

        let gangzi_4 = [
            Some(Mianzi::Gangzi(0)),
            Some(Mianzi::Gangzi(1)),
            Some(Mianzi::Gangzi(2)),
            Some(Mianzi::Gangzi(2)),
        ];
        let result = validate_shoupai(&bingpai, &gangzi_4).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::ExceedsMaxNumSameTile(8),
        ));
    }

    #[test]
    fn invalid_shoupai_fulu_incomplete_hand() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];

        let shunzi_3 = [
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            None,
        ];
        let result = validate_shoupai(&bingpai, &shunzi_3).unwrap_err();
        assert!(matches!(result, InvalidShoupaiError::InvalidNumShoupai(12)));
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
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            Some(Mianzi::Shunzi(27, ClaimedTilePosition::Low)),
            None,
        ];
        let result = validate_shoupai(&bingpai, &shunzi_3).unwrap_err();
        assert!(matches!(result, InvalidShoupaiError::InvalidMianzi(_)));
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
            Some(Mianzi::Kezi(0)),
            Some(Mianzi::Kezi(8)),
            Some(Mianzi::Kezi(9)),
            Some(Mianzi::Kezi(33)),
        ];
        assert_eq!(validate_shoupai_3_player(&bingpai, &kezi_4).unwrap(), ());

        let gangzi_4 = [
            Some(Mianzi::Gangzi(0)),
            Some(Mianzi::Gangzi(8)),
            Some(Mianzi::Gangzi(9)),
            Some(Mianzi::Gangzi(33)),
        ];
        assert_eq!(validate_shoupai_3_player(&bingpai, &gangzi_4).unwrap(), ());
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
            Some(Mianzi::Shunzi(0, ClaimedTilePosition::Low)),
            None,
            None,
            None,
        ];
        let result = validate_shoupai_3_player(&bingpai, &shunzi_1).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidMianziFor3Player(Mianzi::Shunzi(
                0,
                ClaimedTilePosition::Low
            ))
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

        let kezi_2m = [Some(Mianzi::Kezi(1)), None, None, None];
        let result = validate_shoupai_3_player(&bingpai, &kezi_2m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidMianziFor3Player(Mianzi::Kezi(1))
        ));

        let kezi_8m = [Some(Mianzi::Kezi(7)), None, None, None];
        let result = validate_shoupai_3_player(&bingpai, &kezi_8m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidMianziFor3Player(Mianzi::Kezi(7))
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

        let gangzi_2m = [Some(Mianzi::Gangzi(1)), None, None, None];
        let result = validate_shoupai_3_player(&bingpai, &gangzi_2m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidMianziFor3Player(Mianzi::Gangzi(1))
        ));

        let gangzi_8m = [Some(Mianzi::Gangzi(7)), None, None, None];
        let result = validate_shoupai_3_player(&bingpai, &gangzi_8m).unwrap_err();
        assert!(matches!(
            result,
            InvalidShoupaiError::InvalidMianziFor3Player(Mianzi::Gangzi(7))
        ));
    }
}
