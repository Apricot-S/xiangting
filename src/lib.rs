// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![warn(missing_docs)]

//! A library for calculation of deficiency number (a.k.a. xiangting number, 向聴数).
//!
//! This library is based on the algorithm in [majiang-core/lib/xiangting.js](https://github.com/kobalab/majiang-core/blob/master/lib/xiangting.js).
//! However, the following differences apply:
//!
//! - Calculates the replacement number, which is equal to the deficiency number (a.k.a. xiangting number, 向聴数) plus one.
//! - Supports both calculations that include and exclude melds (副露) when considering the four tiles in a hand.
//! - The original algorithm miscalculated the deficiency number in some cases. These errors have been corrected in this library, but the calculation speed has deteriorated as a result.
//! - Supports three-player mahjong.
//! - Does not support short hand or long hand.
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

mod bingpai;
mod calculate;
mod constants;
mod fulu_mianzi;
mod qiduizi;
mod shisanyao;
mod shoupai;

#[cfg(not(feature = "build-map"))]
mod standard;

#[cfg(feature = "build-map")]
#[doc(hidden)]
pub mod standard;

#[doc(hidden)]
pub mod common;

pub use bingpai::Bingpai;
pub use calculate::{calculate_replacement_number, calculate_replacement_number_3_player};
pub use fulu_mianzi::{ClaimedTilePosition, FuluMianzi, InvalidFuluMianziError};
pub use shoupai::{FuluMianziList, InvalidShoupaiError};
