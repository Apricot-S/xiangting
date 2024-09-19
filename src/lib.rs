// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod bingpai;
mod calculate;
mod constants;
mod mianzi;
mod qiduizi;
mod shisanyao;
mod shoupai;
mod standard;

pub mod common;

pub use bingpai::Bingpai;
pub use calculate::{
    calculate_replacement_number, calculate_replacement_number_3_player, XiangtingError,
};
pub use mianzi::{ClaimedTilePosition, Mianzi};
