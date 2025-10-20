// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::{MAX_SHUPAI_INDEX, MAX_TILE_INDEX, NUM_TILE_INDEX};

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

pub(crate) trait TileExt {
    fn is_valid(&self) -> bool;
    fn is_shupai(&self) -> bool;
}

impl TileExt for Tile {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        *self <= MAX_TILE_INDEX
    }

    #[inline(always)]
    fn is_shupai(&self) -> bool {
        *self <= MAX_SHUPAI_INDEX
    }
}

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

/// A type representing tiles as a bit flag set.
///
/// Each bit corresponds to a tile index, following the same mapping as [`Tile`](crate::Tile).
/// The least significant bit (bit 0) represents 1m, bit 1 represents 2m, ...,
/// and bit 33 represents Red (7z).
///
/// This allows efficient representation of sets of tiles, such as
/// necessary tiles or unnecessary tiles.
///
/// # Examples
///
/// ```
/// # use xiangting::TileFlags;
/// // 1m456p789s12z
/// let tiles: TileFlags = 0b0000011_111000000_000111000_000000001;
/// ```
pub type TileFlags = u64;

/// Extension utilities for working with [`TileFlags`](crate::TileFlags).
///
/// This trait provides convenience methods to interpret and transform bit flag sets
/// that represent tiles.
pub trait TileFlagsExt {
    /// Converts the bit flag set into a boolean array.
    ///
    /// Each element indicates whether the corresponding tile index is present:
    /// `true` if the bit for that index is set, `false` otherwise.
    ///
    /// The order of elements is identical to [`TileCounts`](crate::TileCounts).
    ///
    /// # Examples
    ///
    /// ```
    /// # use xiangting::{TileFlags, TileFlagsExt};
    /// // 1m456p789s12z
    /// let flags: TileFlags = 0b0000011_111000000_000111000_000000001;
    /// let arr = flags.to_array();
    ///
    /// assert!(arr[0]);  // 1m
    /// assert!(arr[12]); // 4p
    /// assert!(arr[13]); // 5p
    /// assert!(arr[14]); // 6p
    /// assert!(arr[24]); // 7s
    /// assert!(arr[25]); // 8s
    /// assert!(arr[26]); // 9s
    /// assert!(arr[27]); // 1z (East)
    /// assert!(arr[28]); // 2z (South)
    ///
    /// // A tile not in the set:
    /// assert!(!arr[4]); // 5m
    /// ```
    fn to_array(&self) -> [bool; NUM_TILE_INDEX];
}

impl TileFlagsExt for TileFlags {
    fn to_array(&self) -> [bool; NUM_TILE_INDEX] {
        let mut arr = [false; NUM_TILE_INDEX];
        for (i, t) in arr.iter_mut().enumerate() {
            *t = (self & (1u64 << i)) != 0;
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_7z() {
        let t_7z: Tile = 33;
        assert!(t_7z.is_valid());
    }

    #[test]
    fn is_invalid_8z() {
        let t_8z: Tile = 34;
        assert!(!t_8z.is_valid());
    }

    #[test]
    fn is_shupai_9s() {
        let t_9s: Tile = 26;
        assert!(t_9s.is_shupai());
    }

    #[test]
    fn is_not_shupai_1z() {
        let t_1z: Tile = 27;
        assert!(!t_1z.is_shupai());
    }

    #[test]
    fn is_not_shupai_8z() {
        let t_8z: Tile = 34;
        assert!(!t_8z.is_shupai());
    }

    #[test]
    fn to_array_empty() {
        let flags: TileFlags = 0b0000000_000000000_000000000_000000000;
        assert_eq!(flags.to_array(), [false; 34]);
    }

    #[test]
    fn to_array_all() {
        let flags: TileFlags = 0b1111111_111111111_111111111_111111111;
        assert_eq!(flags.to_array(), [true; 34]);
    }

    #[test]
    fn to_array_1m456p789s12z() {
        let flags: TileFlags = 0b0000011_111000000_000111000_000000001;
        let arr = [
            true, false, false, false, false, false, false, false, false, // m
            false, false, false, true, true, true, false, false, false, // p
            false, false, false, false, false, false, true, true, true, // s
            true, true, false, false, false, false, false, // z
        ];
        assert_eq!(flags.to_array(), arr);
    }
}
