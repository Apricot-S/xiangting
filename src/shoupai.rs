// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::{BingpaiError, TileCountsExt};
use crate::constants::MAX_NUM_MIANZI;
use crate::fulu_mianzi::{ClaimedTilePosition, FuluMianzi, FuluMianziError};
use crate::tile::{Tile, TileCounts};
use thiserror::Error;

/// Errors that occur when an invalid hand (手牌) is provided.
#[derive(Debug, Error)]
pub enum ShoupaiError {
    /// The hand contains an invalid pure hand.
    #[error("hand contains an invalid pure hand: {0}")]
    InvalidBingpai(#[from] BingpaiError),
    /// The hand contains an invalid meld.
    #[error("hand contains an invalid meld: {0}")]
    InvalidFuluMianzi(#[from] FuluMianziError),
    /// The number of melds in the hand exceeds the allowed maximum.
    #[error("the number of melds must be at most {max}, but was {count}")]
    TooManyFuluMianzi {
        /// The maximum number of melds permitted in this hand.
        max: u8,
        /// The actual number of melds found in the hand.
        count: u8,
    },
    /// A specific tile count in the hand exceeds 4.
    #[error("tile {tile} count must be 4 or less but was {count}")]
    TooManyCopies {
        /// The tile that appears too many times.
        tile: Tile,
        /// The actual number of copies found in the hand.
        count: u8,
    },
}

type FuluMianziList = [FuluMianzi];

trait FuluMianziListExt {
    fn to_tile_counts(&self) -> TileCounts;
}

impl FuluMianziListExt for FuluMianziList {
    fn to_tile_counts(&self) -> TileCounts {
        self.iter().fold([0u8; 34], |mut fulupai, m| {
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
    }
}

pub(crate) struct Shoupai<'a> {
    bingpai: &'a TileCounts,
    tile_counts: Option<TileCounts>,
    num_required_bingpai_mianzi: u8,
}

impl<'a> Shoupai<'a> {
    pub(crate) fn new(
        bingpai: &'a TileCounts,
        fulu_mianzi_list: Option<&[FuluMianzi]>,
    ) -> Result<Self, ShoupaiError> {
        let num_bingpai = bingpai.count()?;
        let num_required_bingpai_mianzi = num_bingpai / 3;

        let max_num_fulu = MAX_NUM_MIANZI - num_required_bingpai_mianzi;
        let num_fulu = fulu_mianzi_list.map(|fl| fl.len() as u8);
        validate_num_fulu(num_fulu, max_num_fulu)?;

        fulu_mianzi_list
            .map(|fl| fl.iter().try_for_each(|f| f.validate()))
            .transpose()?;

        let tile_counts = get_tile_counts(bingpai, fulu_mianzi_list);
        validate_tile_counts(tile_counts)?;

        Ok(Self {
            bingpai,
            tile_counts,
            num_required_bingpai_mianzi,
        })
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn bingpai(&self) -> &'a TileCounts {
        self.bingpai
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn tile_counts(&self) -> Option<&TileCounts> {
        self.tile_counts.as_ref()
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn num_required_bingpai_mianzi(&self) -> u8 {
        self.num_required_bingpai_mianzi
    }
}

pub(crate) struct Shoupai3Player<'a> {
    bingpai: &'a TileCounts,
    tile_counts: Option<TileCounts>,
    num_required_bingpai_mianzi: u8,
}

impl<'a> Shoupai3Player<'a> {
    pub(crate) fn new(
        bingpai: &'a TileCounts,
        fulu_mianzi_list: Option<&[FuluMianzi]>,
    ) -> Result<Self, ShoupaiError> {
        let num_bingpai = bingpai.count_3_player()?;
        let num_required_bingpai_mianzi = num_bingpai / 3;

        let max_num_fulu = MAX_NUM_MIANZI - num_required_bingpai_mianzi;
        let num_fulu = fulu_mianzi_list.map(|fl| fl.len() as u8);
        validate_num_fulu(num_fulu, max_num_fulu)?;

        fulu_mianzi_list
            .map(|fl| fl.iter().try_for_each(|f| f.validate_3_player()))
            .transpose()?;

        let tile_counts = get_tile_counts(bingpai, fulu_mianzi_list);
        validate_tile_counts(tile_counts)?;

        Ok(Self {
            bingpai,
            tile_counts,
            num_required_bingpai_mianzi,
        })
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn bingpai(&self) -> &'a TileCounts {
        self.bingpai
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn tile_counts(&self) -> Option<&TileCounts> {
        self.tile_counts.as_ref()
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn num_required_bingpai_mianzi(&self) -> u8 {
        self.num_required_bingpai_mianzi
    }
}

impl<'a> From<Shoupai3Player<'a>> for Shoupai<'a> {
    fn from(value: Shoupai3Player<'a>) -> Self {
        Self {
            bingpai: value.bingpai,
            tile_counts: value.tile_counts,
            num_required_bingpai_mianzi: value.num_required_bingpai_mianzi,
        }
    }
}

fn validate_num_fulu(num_fulu: Option<u8>, max_num_fulu: u8) -> Result<(), ShoupaiError> {
    num_fulu
        .filter(|&n| n > max_num_fulu)
        .map(|n| ShoupaiError::TooManyFuluMianzi {
            max: max_num_fulu,
            count: n,
        })
        .map_or(Ok(()), Err)
}

fn get_tile_counts(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Option<TileCounts> {
    let fulupai = fulu_mianzi_list.map(|fl| fl.to_tile_counts());
    fulupai.map(|fp| std::array::from_fn(|i| bingpai[i] + fp[i]))
}

fn validate_tile_counts(tile_counts: Option<TileCounts>) -> Result<(), ShoupaiError> {
    tile_counts.map_or(Ok(()), |tc| {
        tc.iter()
            .enumerate()
            .find(|(_, c)| **c > 4)
            .map(|(i, &c)| ShoupaiError::TooManyCopies {
                tile: i as Tile,
                count: c,
            })
            .map_or(Ok(()), Err)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;

    #[test]
    fn to_tile_counts_1m_23m() {
        let fulu_mianzi_list = [FuluMianzi::Shunzi(0, ClaimedTilePosition::Low)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("123m")
        );
    }

    #[test]
    fn to_tile_counts_4p_35p() {
        let fulu_mianzi_list = [FuluMianzi::Shunzi(12, ClaimedTilePosition::Middle)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("435p")
        );
    }

    #[test]
    fn to_tile_counts_9s_78s() {
        let fulu_mianzi_list = [FuluMianzi::Shunzi(26, ClaimedTilePosition::High)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("978s")
        );
    }

    #[test]
    fn to_tile_counts_111z() {
        let fulu_mianzi_list = [FuluMianzi::Kezi(27)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("111z")
        );
    }

    #[test]
    fn to_tile_counts_7777z() {
        let fulu_mianzi_list = [FuluMianzi::Gangzi(33)];
        assert_eq!(
            fulu_mianzi_list.to_tile_counts(),
            TileCounts::from_code("7777z")
        );
    }

    #[test]
    #[should_panic]
    fn to_tile_counts_111z_not_consider_invalid_fulu() {
        let fulu_mianzi_list = [FuluMianzi::Kezi(34)];
        fulu_mianzi_list.to_tile_counts();
    }
}
