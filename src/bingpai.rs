// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::NUM_TILE_INDEX;

/// 兵牌: Hand excluding melds (a.k.a. pure hand, 純手牌).
///
/// Each element of the array represents the count of a specific tile in the hand.
/// The correspondence between the index and the tile is the same as [`Tile`](crate::Tile).
///
/// See also [`BingpaiError`](crate::BingpaiError) for more information.
///
/// # Examples
///
/// ```
/// # use xiangting::Bingpai;
/// // 111m456p789s11222z
/// let hand_14: Bingpai = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 111m1z (3 melds)
/// let hand_4: Bingpai = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
/// ```
pub type Bingpai = [u8; NUM_TILE_INDEX];
