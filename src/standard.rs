// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(not(feature = "build-file"))]
mod core;
#[cfg(not(feature = "build-file"))]
mod hash;
#[cfg(not(feature = "build-file"))]
mod necessary_tiles;
#[cfg(not(feature = "build-file"))]
mod replacement_number;
#[cfg(not(feature = "build-file"))]
mod shupai_map;
#[cfg(not(feature = "build-file"))]
mod shupai_table;
#[cfg(not(feature = "build-file"))]
mod unpack;
#[cfg(not(feature = "build-file"))]
mod wanzi_19_map;
#[cfg(not(feature = "build-file"))]
mod wanzi_19_table;
#[cfg(not(feature = "build-file"))]
mod zipai_map;
#[cfg(not(feature = "build-file"))]
mod zipai_table;

#[cfg(not(feature = "build-file"))]
pub(super) use necessary_tiles::{calculate_necessary_tiles, calculate_necessary_tiles_3_player};
#[cfg(not(feature = "build-file"))]
pub(super) use replacement_number::{
    calculate_replacement_number, calculate_replacement_number_3_player,
};

#[cfg(feature = "build-map")]
pub mod core;
#[cfg(feature = "build-map")]
pub mod hash;
#[cfg(feature = "build-map")]
pub mod shupai_table;
#[cfg(feature = "build-map")]
pub mod wanzi_19_table;
#[cfg(feature = "build-map")]
pub mod zipai_table;
