// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

// #![warn(missing_docs)]

//! A library for calculating the deficiency number (a.k.a. xiangting number, 向聴数).
//!
//! This library is based on the algorithm in [Cryolite's Nyanten](https://github.com/Cryolite/nyanten).  
//! However, it introduces the following additional features:
//!
//! - Supports rules that include and exclude melded tiles when determining if a hand contains four identical tiles.
//! - Supports three-player mahjong.

#[cfg(not(feature = "build-file"))]
mod bingpai;
#[cfg(not(feature = "build-file"))]
mod calculate;
#[cfg(not(feature = "build-file"))]
mod constants;
#[cfg(not(feature = "build-file"))]
mod fulu_mianzi;
#[cfg(not(feature = "build-file"))]
mod qiduizi;
#[cfg(not(feature = "build-file"))]
mod shoupai;
#[cfg(not(feature = "build-file"))]
mod tile;

#[cfg(all(test, not(feature = "build-file")))]
mod test_utils;

pub use bingpai::{Bingpai, BingpaiError};
pub use calculate::{calculate_replacement_number, calculate_replacement_number_3_player};
pub use fulu_mianzi::{ClaimedTilePosition, FuluMianzi, FuluMianziError};
pub use shoupai::{ShoupaiError, XiangtingError};
pub use tile::Tile;
