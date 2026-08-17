// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![no_std]
#![warn(missing_docs)]

//! A library for calculating the deficiency number (a.k.a. xiàngtīng number, 向聴数).
//!
//! # Example
//!
//! ```
//! # use xiangting::{PlayerCount, calculate_replacement_number};
//! # use xiangting::BingpaiError;
//! # fn main() -> Result<(), BingpaiError> {
//! // 123m456p789s11222z
//! let hand: [u8; 34] = [
//!     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
//!     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
//!     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
//!     2, 3, 0, 0, 0, 0, 0, // z
//! ];
//!
//! let replacement_number = calculate_replacement_number(&hand, PlayerCount::Four)?;
//! assert_eq!(replacement_number, 0u8);
//! # Ok(())
//! # }
//! ```

#[cfg(not(feature = "build-map"))]
mod bingpai;
#[cfg(not(feature = "build-map"))]
mod config;
#[cfg(not(feature = "build-map"))]
mod necessary_tiles;
#[cfg(not(feature = "build-map"))]
mod qiduizi;
#[cfg(not(feature = "build-map"))]
mod replacement_number;
#[cfg(not(feature = "build-map"))]
mod shisanyao;
#[cfg(not(feature = "build-map"))]
mod standard;
#[cfg(not(feature = "build-map"))]
mod tile;
#[cfg(not(feature = "build-map"))]
mod unnecessary_tiles;

#[cfg(all(test, not(feature = "build-map")))]
mod test_utils;

#[cfg(not(feature = "build-map"))]
pub use bingpai::BingpaiError;
#[cfg(not(feature = "build-map"))]
pub use config::PlayerCount;
#[cfg(not(feature = "build-map"))]
pub use necessary_tiles::calculate_necessary_tiles;
#[cfg(not(feature = "build-map"))]
pub use replacement_number::calculate_replacement_number;
#[cfg(not(feature = "build-map"))]
pub use tile::{Tile, TileCounts, TileFlags, TileFlagsExt};
#[cfg(not(feature = "build-map"))]
pub use unnecessary_tiles::calculate_unnecessary_tiles;

#[cfg(feature = "build-map")]
#[doc(hidden)]
pub mod standard;
