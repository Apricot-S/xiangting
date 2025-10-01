// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::{MAX_NUM_SHOUPAI, MAX_TILE_COPIES, NUM_TILE_INDEX};
use crate::tile::Tile;
use thiserror::Error;

/// 兵牌: Hand excluding melds (a.k.a. pure hand, 純手牌).
///
/// Each element of the array represents the count of a specific tile in the hand.
/// The correspondence between the index and the tile is the same as [`Tile`](crate::Tile).
///
/// See also [`BingpaiError`](crate::BingpaiError) for more information.
///
/// # Examples
///
/// ```
/// # use xiangting::Bingpai;
/// // 111m456p789s11222z
/// let hand: Bingpai = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
/// ```
pub type Bingpai = [u8; NUM_TILE_INDEX];

#[derive(Debug, Error)]
pub enum BingpaiError {
    #[error("tile {tile} count must be 4 or less but was {count}")]
    TooManyCopies { tile: Tile, count: u8 },
    #[error("total tile count must be 14 or less but was {0}")]
    TooManyTiles(u8),
    #[error("total tile count must be a multiple of 3 plus 1 or 2 but was {0}")]
    InvalidTileCount(u8),
    #[error("tile {0} cannot be used in 3-player mahjong")]
    InvalidTileFor3Player(Tile),
}

pub(crate) trait BingpaiExt {
    fn count(&self) -> Result<u8, BingpaiError>;
    fn count_3_player(&self) -> Result<u8, BingpaiError>;
}

impl BingpaiExt for Bingpai {
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

    fn count_3_player(&self) -> Result<u8, BingpaiError> {
        if let Some(i) = self[1..8].iter().position(|&t| t > 0) {
            return Err(BingpaiError::InvalidTileFor3Player((i + 1) as u8));
        }
        self.count()
    }
}
