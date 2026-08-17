// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(not(feature = "build-map"))]
mod common;
#[cfg(not(feature = "build-map"))]
mod core;
#[cfg(not(feature = "build-map"))]
mod hash;
#[cfg(not(feature = "build-map"))]
mod necessary_tiles;
#[cfg(not(feature = "build-map"))]
mod replacement_number;
#[cfg(not(feature = "build-map"))]
mod shupai_map;
#[cfg(not(feature = "build-map"))]
mod shupai_table;
#[cfg(not(feature = "build-map"))]
mod unnecessary_tiles;
#[cfg(not(feature = "build-map"))]
mod unpack;
#[cfg(not(feature = "build-map"))]
mod wanzi_19_map;
#[cfg(not(feature = "build-map"))]
mod wanzi_19_table;
#[cfg(not(feature = "build-map"))]
mod zipai_map;
#[cfg(not(feature = "build-map"))]
mod zipai_table;

#[cfg(not(feature = "build-map"))]
pub(super) use necessary_tiles::calculate_necessary_tiles;
#[cfg(not(feature = "build-map"))]
pub(super) use replacement_number::calculate_replacement_number;
#[cfg(not(feature = "build-map"))]
pub(super) use unnecessary_tiles::calculate_unnecessary_tiles;

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
