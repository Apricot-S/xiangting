// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

//! A library for calculation of deficiency number (a.k.a. xiangting number, 向聴数).
//!
//! This is a port of [majiang-core/lib/xiangting.js](https://github.com/kobalab/majiang-core/blob/master/lib/xiangting.js) to Rust.
//! However, the following differences apply:
//!
//! - Calculates the replacement number, which is equal to the deficiency number (a.k.a. xiangting number, 向聴数) plus one.
//! - Supports both calculations that include and exclude melds (副露) when considering the four tiles in a hand.
//! - The original algorithm miscalculated the deficiency number in some cases. These errors have been corrected in this port, but the calculation speed has deteriorated as a result.
//! - Supports three-player mahjong.
//! - Does not support short hand or long hand.

#![warn(missing_docs)]

mod bingpai;
mod calculate;
mod constants;
mod mianzi;
mod qiduizi;
mod shisanyao;
mod shoupai;
mod standard;

#[doc(hidden)]
pub mod common;

pub use bingpai::Bingpai;
pub use calculate::{
    calculate_replacement_number, calculate_replacement_number_3_player, XiangtingError,
};
pub use mianzi::{ClaimedTilePosition, Mianzi};
pub use shoupai::FuluMianziList;
