// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![warn(missing_docs)]

//! A library for calculating the deficiency number (a.k.a. xiangting number, 向聴数).
//!
//! This library is based on the algorithm in [Cryolite's Nyanten](https://github.com/Cryolite/nyanten).  
//! However, it introduces the following additional features:
//!
//! - Supports rules that include and exclude melded tiles when determining if a hand contains four identical tiles.
//! - Supports three-player mahjong.

#[cfg(not(feature = "build-file"))]
mod tile;
