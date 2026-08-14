// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::tile::{Tile, TileCounts};
use core::marker::PhantomData;
use thiserror::Error;

const MAX_TILE_COPIES: u8 = 4;
const MAX_NUM_BINGPAI: u8 = 14;

pub(crate) trait PlayerRule {
    const IS_THREE_PLAYER: bool;

    fn validate(tile_counts: &TileCounts) -> Result<(), BingpaiError>;
}

pub(crate) enum FourPlayer {}

pub(crate) enum ThreePlayer {}

impl PlayerRule for FourPlayer {
    const IS_THREE_PLAYER: bool = false;

    #[inline(always)]
    fn validate(_tile_counts: &TileCounts) -> Result<(), BingpaiError> {
        Ok(())
    }
}

impl PlayerRule for ThreePlayer {
    const IS_THREE_PLAYER: bool = true;

    #[inline(always)]
    fn validate(tile_counts: &TileCounts) -> Result<(), BingpaiError> {
        if let Some(i) = tile_counts[1..8].iter().position(|&count| count != 0) {
            return Err(BingpaiError::InvalidTileForThreePlayer((i + 1) as Tile));
        }

        Ok(())
    }
}

pub(crate) struct Bingpai<'a, R: PlayerRule> {
    tile_counts: &'a TileCounts,
    num_required_bingpai_mianzi: u8,
    rule: PhantomData<fn() -> R>,
}

/// Errors that occur when an invalid pure hand (純手牌) is provided.
#[derive(Debug, Error)]
pub enum BingpaiError {
    /// A specific tile count in the pure hand exceeds 4.
    #[error("tile {tile} count must be 4 or less but was {count}")]
    TooManyCopies {
        /// The tile that appears too many times.
        tile: Tile,
        /// The actual number of copies found in the pure hand.
        count: u8,
    },
    /// Total tile count in the pure hand exceeds 14.
    #[error("total tile count must be 14 or less but was {0}")]
    TooManyTiles(u8),
    /// Total tile count in the pure hand is not of the form 3n+1 or 3n+2.
    #[error("total tile count must be a multiple of 3 plus 1 or 2 but was {0}")]
    InvalidTileCount(u8),
    /// The pure hand contains tiles that are not used in three-player mahjong (2m-8m).
    #[error("tile {0} cannot be used in three-player mahjong")]
    InvalidTileForThreePlayer(Tile),
}

pub(crate) trait TileCountsExt {
    fn count(&self) -> Result<u8, BingpaiError>;
}

impl TileCountsExt for TileCounts {
    fn count(&self) -> Result<u8, BingpaiError> {
        self.iter()
            .enumerate()
            .find(|(_, c)| **c > MAX_TILE_COPIES)
            .map(|(i, &c)| BingpaiError::TooManyCopies {
                tile: i as Tile,
                count: c,
            })
            .map_or(Ok(()), Err)?;

        let num_bingpai: u8 = self.iter().sum();
        match num_bingpai {
            n if n > MAX_NUM_BINGPAI => Err(BingpaiError::TooManyTiles(n)),
            n if n % 3 == 0 => Err(BingpaiError::InvalidTileCount(n)),
            n => Ok(n),
        }
    }
}

impl<'a, R: PlayerRule> Bingpai<'a, R> {
    pub(crate) fn new(tile_counts: &'a TileCounts) -> Result<Self, BingpaiError> {
        R::validate(tile_counts)?;
        let num_bingpai = tile_counts.count()?;
        let num_required_bingpai_mianzi = num_bingpai / 3;

        Ok(Self {
            tile_counts,
            num_required_bingpai_mianzi,
            rule: PhantomData,
        })
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn tile_counts(&self) -> &'a TileCounts {
        self.tile_counts
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn num_required_bingpai_mianzi(&self) -> u8 {
        self.num_required_bingpai_mianzi
    }
}
