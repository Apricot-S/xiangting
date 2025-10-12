// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::NUM_TILE_INDEX;

/// 牌: Tile.
///
/// The value represents the index of the tile.
/// The correspondence between the index and the tile is shown in the table below.
///
/// | Index | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   |
/// | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
/// | Tile  | 1m  | 2m  | 3m  | 4m  | 5m  | 6m  | 7m  | 8m  | 9m  |
///
/// | Index | 9   | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  |
/// | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
/// | Tile  | 1p  | 2p  | 3p  | 4p  | 5p  | 6p  | 7p  | 8p  | 9p  |
///
/// | Index | 18  | 19  | 20  | 21  | 22  | 23  | 24  | 25  | 26  |
/// | ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
/// | Tile  | 1s  | 2s  | 3s  | 4s  | 5s  | 6s  | 7s  | 8s  | 9s  |
///
/// | Index | 27        | 28         | 29        | 30         | 31         | 32         | 33       |
/// | ----- | --------- | ---------- | --------- | ---------- | ---------- | ---------- | -------- |
/// | Tile  | East (1z) | South (2z) | West (3z) | North (4z) | White (5z) | Green (6z) | Red (7z) |
pub type Tile = u8;

/// A type representing the number of tiles for each kind.
///
/// Each element of the array represents the count of a specific tile in the hand.
/// The correspondence between the index and the tile is the same as [`Tile`](crate::Tile).
///
/// # Examples
///
/// ```
/// # use xiangting::TileCounts;
/// // 111m456p789s11222z
/// let hand: TileCounts = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
/// ```
pub type TileCounts = [u8; NUM_TILE_INDEX];
