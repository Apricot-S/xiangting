// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod block_count;
mod calculate;
mod tile_flag;

#[cfg(not(feature = "build-map"))]
mod core;
#[cfg(not(feature = "build-map"))]
mod hash;
#[cfg(not(feature = "build-map"))]
mod shupai_map;
#[cfg(not(feature = "build-map"))]
mod shupai_table;
#[cfg(not(feature = "build-map"))]
mod zipai_map;
#[cfg(not(feature = "build-map"))]
mod zipai_table;

#[cfg(feature = "build-map")]
pub mod core;
#[cfg(feature = "build-map")]
pub mod hash;
#[cfg(feature = "build-map")]
pub mod shupai_table;
#[cfg(feature = "build-map")]
pub mod zipai_table;

pub(super) use calculate::{calculate_replacement_number, calculate_replacement_number_3_player};
