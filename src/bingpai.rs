// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::{MAX_NUM_SAME_TILE, MAX_NUM_SHOUPAI, NUM_TILE_INDEX};
use thiserror::Error;

/// 兵牌: Hand excluding melds (a.k.a. pure hand, 純手牌).
///
/// A element of array indicates the number of each tile in the hand.
/// The correspondence between the index and the tile is shown in the table below.
///
/// | Index | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   |
/// | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
/// | Tile  | 1m  | 2m  | 3m  | 4m  | 5m  | 6m  | 7m  | 8m  | 9m  |
///
/// | Index | 9   | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  |
/// | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
/// | Tile  | 1p  | 2p  | 3p  | 4p  | 5p  | 6p  | 7p  | 8p  | 9p  |
///
/// | Index | 18  | 19  | 20  | 21  | 22  | 23  | 24  | 25  | 26  |
/// | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
/// | Tile  | 1s  | 2s  | 3s  | 4s  | 5s  | 6s  | 7s  | 8s  | 9s  |
///
/// | Index | 27        | 28         | 29        | 30         | 31         | 32         | 33       |
/// | ----- | --------- | ---------- | --------- | ---------- | ---------- | ---------- | -------- |
/// | Tile  | East (1z) | South (2z) | West (3z) | North (4z) | White (5z) | Green (6z) | Red (7z) |
///
/// See also [`InvalidShoupaiError`](crate::InvalidShoupaiError) for more information.
///
/// # Examples
///
/// ```
/// # use xiangting::Bingpai;
/// // 111m456p789s11222z
/// let hand_14: Bingpai = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 111m1z (3 melds)
/// let hand_4: Bingpai = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
/// ```
pub type Bingpai = [u8; NUM_TILE_INDEX];

#[derive(Debug, Error)]
pub(crate) enum InvalidBingpaiError {
    #[error("same tile count must be 4 or less but was {0}")]
    ExceedsMaxNumSameTile(u8),
    #[error("total tile count must be 14 or less but was {0}")]
    ExceedsMaxNumBingpai(u8),
    #[error("hand is empty")]
    EmptyBingpai,
    #[error("total tile count must be a multiple of 3 plus 1 or 2 but was {0}")]
    InvalidNumBingpai(u8),
    #[error(
        "tile index {0} (must be outside 1 (2m) to 7 (8m)) cannot be used in 3-player mahjong"
    )]
    InvalidTileFor3Player(usize),
}

pub(super) fn count_bingpai(bingpai: &Bingpai) -> Result<u8, InvalidBingpaiError> {
    let num_bingpai = bingpai.iter().try_fold(0, |acc, &num_tile| {
        if num_tile > MAX_NUM_SAME_TILE {
            return Err(InvalidBingpaiError::ExceedsMaxNumSameTile(num_tile));
        }
        Ok(acc + num_tile)
    })?;

    if num_bingpai > MAX_NUM_SHOUPAI {
        return Err(InvalidBingpaiError::ExceedsMaxNumBingpai(num_bingpai));
    }
    if num_bingpai == 0 {
        return Err(InvalidBingpaiError::EmptyBingpai);
    }
    if num_bingpai % 3 == 0 {
        return Err(InvalidBingpaiError::InvalidNumBingpai(num_bingpai));
    }

    Ok(num_bingpai)
}

pub(super) fn count_bingpai_3_player(bingpai: &Bingpai) -> Result<u8, InvalidBingpaiError> {
    bingpai[1..8].iter().enumerate().try_for_each(|(i, &t)| {
        if t > 0 {
            return Err(InvalidBingpaiError::InvalidTileFor3Player(i + 1));
        }
        Ok(())
    })?;

    count_bingpai(bingpai)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_bingpai() {
        let bingpai_1: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let bingpai_2: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 1, 1, 1, 1, 1, // z
        ];
        let bingpai_3: Bingpai = [
            4, 4, 4, 2, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];

        let num_bingpai_1 = count_bingpai(&bingpai_1).unwrap();
        let num_bingpai_2 = count_bingpai(&bingpai_2).unwrap();
        let num_bingpai_3 = count_bingpai(&bingpai_3).unwrap();

        assert_eq!(num_bingpai_1, bingpai_1.iter().sum());
        assert_eq!(num_bingpai_2, bingpai_2.iter().sum());
        assert_eq!(num_bingpai_3, bingpai_3.iter().sum());
    }

    #[test]
    fn invalid_bingpai_empty() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let result = count_bingpai(&bingpai).unwrap_err();
        assert!(matches!(result, InvalidBingpaiError::EmptyBingpai));
    }

    #[test]
    fn invalid_bingpai_too_many_tiles() {
        let bingpai: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 1, 1, 1, 1, 1, 1, // z
        ];
        let result = count_bingpai(&bingpai).unwrap_err();
        assert!(matches!(
            result,
            InvalidBingpaiError::ExceedsMaxNumBingpai(15)
        ));
    }

    #[test]
    fn invalid_bingpai_5th_tile() {
        let bingpai_1: Bingpai = [
            5, 4, 4, 2, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let result_1 = count_bingpai(&bingpai_1).unwrap_err();
        assert!(matches!(
            result_1,
            InvalidBingpaiError::ExceedsMaxNumSameTile(5)
        ));

        let bingpai_2: Bingpai = [
            5, 4, 4, 4, 2, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let result_2 = count_bingpai(&bingpai_2).unwrap_err();
        assert!(matches!(
            result_2,
            InvalidBingpaiError::ExceedsMaxNumSameTile(5)
        ));
    }

    #[test]
    fn invalid_bingpai_incomplete_hand() {
        let bingpai: Bingpai = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 1, 1, 1, // z
        ];
        let result = count_bingpai(&bingpai).unwrap_err();
        assert!(matches!(result, InvalidBingpaiError::InvalidNumBingpai(12)));
    }

    #[test]
    fn valid_bingpai_3_player() {
        let bingpai_1: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let bingpai_2: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 1, 1, 1, 1, 1, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 1, 1, 1, 1, 1, // z
        ];
        let bingpai_3: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 4, // m
            4, 2, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];

        let num_bingpai_1 = count_bingpai_3_player(&bingpai_1).unwrap();
        let num_bingpai_2 = count_bingpai_3_player(&bingpai_2).unwrap();
        let num_bingpai_3 = count_bingpai_3_player(&bingpai_3).unwrap();

        assert_eq!(num_bingpai_1, bingpai_1.iter().sum());
        assert_eq!(num_bingpai_2, bingpai_2.iter().sum());
        assert_eq!(num_bingpai_3, bingpai_3.iter().sum());
    }

    #[test]
    fn invalid_bingpai_3_player_empty() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let result = count_bingpai_3_player(&bingpai).unwrap_err();
        assert!(matches!(result, InvalidBingpaiError::EmptyBingpai));
    }

    #[test]
    fn invalid_bingpai_3_player_too_many_tiles() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 1, 1, 1, 1, 1, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 1, 1, 1, 1, 1, 1, // z
        ];
        let result = count_bingpai_3_player(&bingpai).unwrap_err();
        assert!(matches!(
            result,
            InvalidBingpaiError::ExceedsMaxNumBingpai(15)
        ));
    }

    #[test]
    fn invalid_bingpai_3_player_5th_tile() {
        let bingpai_1: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            5, 4, 4, 2, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let result_1 = count_bingpai_3_player(&bingpai_1).unwrap_err();
        assert!(matches!(
            result_1,
            InvalidBingpaiError::ExceedsMaxNumSameTile(5)
        ));

        let bingpai_2: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            5, 4, 4, 4, 2, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let result_2 = count_bingpai_3_player(&bingpai_2).unwrap_err();
        assert!(matches!(
            result_2,
            InvalidBingpaiError::ExceedsMaxNumSameTile(5)
        ));
    }

    #[test]
    fn invalid_bingpai_3_player_incomplete_hand() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 1, 1, 1, 1, 1, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 1, 1, 1, // z
        ];
        let result = count_bingpai_3_player(&bingpai).unwrap_err();
        assert!(matches!(result, InvalidBingpaiError::InvalidNumBingpai(12)));
    }

    #[test]
    fn invalid_bingpai_3_player_2m_8m() {
        let bingpai_2m: Bingpai = [
            0, 1, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let bingpai_8m: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 1, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];

        let result_2m = count_bingpai_3_player(&bingpai_2m).unwrap_err();
        let result_8m = count_bingpai_3_player(&bingpai_8m).unwrap_err();

        assert!(matches!(
            result_2m,
            InvalidBingpaiError::InvalidTileFor3Player(1)
        ));
        assert!(matches!(
            result_8m,
            InvalidBingpaiError::InvalidTileFor3Player(7)
        ));
    }
}
