// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![warn(missing_docs)]

//! A library for calculating the deficiency number (a.k.a. the xiangting number, 向聴数).
//!
//! This library is based on the algorithm in [Nyanten](https://github.com/Cryolite/nyanten).  
//! However, it introduces the following additional features:
//!
//! - Supports rules that include and exclude melded tiles when determining if a hand contains four identical tiles.
//! - Supports three-player mahjong.
//!
//! # Example
//!
//! ```
//! # use xiangting::calculate_replacement_number;
//! # use xiangting::InvalidShoupaiError;
//! # fn main() -> Result<(), InvalidShoupaiError> {
//! // 123m456p789s11222z
//! let hand_14: [u8; 34] = [
//!     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
//!     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
//!     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
//!     2, 3, 0, 0, 0, 0, 0, // z
//! ];
//!
//! let replacement_number = calculate_replacement_number(&hand_14, &None);
//! assert_eq!(replacement_number?, 0u8);
//! # Ok(())
//! # }
//! ```

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
mod shisanyao;
#[cfg(not(feature = "build-file"))]
mod shoupai;
#[cfg(not(feature = "build-file"))]
mod standard;

#[cfg(not(feature = "build-file"))]
pub use bingpai::Bingpai;
#[cfg(not(feature = "build-file"))]
pub use calculate::{calculate_replacement_number, calculate_replacement_number_3_player};
#[cfg(not(feature = "build-file"))]
pub use fulu_mianzi::{ClaimedTilePosition, FuluMianzi, InvalidFuluMianziError};
#[cfg(not(feature = "build-file"))]
pub use shoupai::{FuluMianziList, InvalidShoupaiError};

#[cfg(feature = "build-map")]
#[doc(hidden)]
pub mod standard;
