// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::{MAX_NUM_SAME_TILE, MAX_NUM_SHOUPAI, NUM_TILE_INDEX};
use thiserror::Error;

pub type Bingpai = [u8; NUM_TILE_INDEX];

#[derive(Debug, Error)]
pub enum InvalidBingpaiError {
    #[error("Invalid hand: Same tile count exceeds 4 ({0}).")]
    ExceedsMaxNumSameTile(u8),
    #[error("Invalid hand: Total tile count exceeds 14 ({0}).")]
    ExceedsMaxNumBingpai(u8),
    #[error("Invalid hand: Hand is empty.")]
    EmptyBingpai,
    #[error("Invalid hand: Total tile count is not a multiple of 3 plus 1 or 2 ({0}).")]
    InvalidNumBingpai(u8),
    #[error("Invalid hand: 2m to 8m cannot be used in 3-player mahjong ({0}).")]
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
