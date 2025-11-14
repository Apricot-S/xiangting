// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::{MAX_NUM_SHOUPAI, MAX_TILE_COPIES};
use crate::tile::{Tile, TileCounts};
use thiserror::Error;

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
    /// The pure hand contains tiles that are not used in 3-player mahjong (2m-8m).
    #[error("tile {0} cannot be used in 3-player mahjong")]
    InvalidTileFor3Player(Tile),
}

pub(crate) trait TileCountsExt {
    fn count(&self) -> Result<u8, BingpaiError>;
    fn count_3p(&self) -> Result<u8, BingpaiError>;
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
            n if n > MAX_NUM_SHOUPAI => Err(BingpaiError::TooManyTiles(n)),
            n if n % 3 == 0 => Err(BingpaiError::InvalidTileCount(n)),
            n => Ok(n),
        }
    }

    fn count_3p(&self) -> Result<u8, BingpaiError> {
        if let Some(i) = self[1..8].iter().position(|&t| t > 0) {
            return Err(BingpaiError::InvalidTileFor3Player((i + 1) as u8));
        }
        self.count()
    }
}
