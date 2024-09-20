// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::constants::{MAX_SHUPAI_INDEX, MAX_TILE_INDEX, NUM_TILE_INDEX};
use std::fmt;
use thiserror::Error;

type Tile = u8;

#[derive(Debug, Clone)]
pub enum ClaimedTilePosition {
    Low,
    Middle,
    High,
}

/// 面子: Meld.
///
/// # Examples
///
/// ```
/// # use xiangting::{ClaimedTilePosition, Mianzi};
/// // 4-56p (Chii 4m Low)
/// let shunzi = Mianzi::Shunzi(12, ClaimedTilePosition::Low);
///
/// // 1-11z (Pon 1z)
/// let kezi = Mianzi::Kezi(27);
///
/// // 7-777s (Kan 7s)
/// let gangzi = Mianzi::Gangzi(24);
/// ```
#[derive(Clone)]
pub enum Mianzi {
    /// 順子: Sequence.
    ///
    /// The first argument represents the index of the tile.
    /// The second argument represents the position of the claimed tile in the meld.
    /// The correspondence between the index and the tile is the same as [Bingpai](crate::Bingpai).
    ///
    /// # Examples
    ///
    /// ```
    /// # use xiangting::{ClaimedTilePosition, Mianzi};
    /// // 1-23m
    /// let shunzi_low = Mianzi::Shunzi(0, ClaimedTilePosition::Low);
    ///
    /// // 2-13m
    /// let shunzi_middle = Mianzi::Shunzi(1, ClaimedTilePosition::Middle);
    ///
    /// // 3-12m
    /// let shunzi_high = Mianzi::Shunzi(2, ClaimedTilePosition::High);
    /// ```
    Shunzi(Tile, ClaimedTilePosition),
    /// 刻子: Triplet.
    ///
    /// The argument represents the index of the tile.
    /// The correspondence between the index and the tile is the same as [Bingpai](crate::Bingpai).
    ///
    /// # Examples
    ///
    /// ```
    /// # use xiangting::Mianzi;
    /// // 1-11m
    /// let kezi = Mianzi::Kezi(0);
    /// ```
    Kezi(Tile),
    /// 槓子: Quad.
    ///
    /// The argument represents the index of the tile.
    /// The correspondence between the index and the tile is the same as [Bingpai](crate::Bingpai).
    ///
    /// # Examples
    ///
    /// ```
    /// # use xiangting::Mianzi;
    /// // 1-111m (Kan 1m)
    /// let gangzi = Mianzi::Gangzi(0);
    /// ```
    Gangzi(Tile),
}

#[derive(Debug, Error)]
pub enum InvalidMianziError {
    #[error("Invalid meld: Tile index must be between 0 and 33, but got {0}.")]
    IndexOutOfRange(Tile),
    #[error("Invalid meld: A sequence cannot be made with honors ({0}).")]
    ShunziWithZipai(Tile),
    #[error(
        "Invalid meld: A sequence cannot be made with the given tile and position ({0}, {1:?})."
    )]
    InvalidShunziCombination(Tile, ClaimedTilePosition),
}

impl Mianzi {
    pub fn validate(&self) -> Result<(), InvalidMianziError> {
        match self {
            Mianzi::Shunzi(tile, position) => {
                if *tile > MAX_SHUPAI_INDEX {
                    return Err(InvalidMianziError::ShunziWithZipai(*tile));
                }
                if !Mianzi::is_valid_shunzi_combination(tile, position) {
                    return Err(InvalidMianziError::InvalidShunziCombination(
                        *tile,
                        position.clone(),
                    ));
                }
                Ok(())
            }
            Mianzi::Kezi(tile) | Mianzi::Gangzi(tile) => {
                if *tile > MAX_TILE_INDEX {
                    return Err(InvalidMianziError::IndexOutOfRange(*tile));
                }
                Ok(())
            }
        }
    }

    #[inline]
    fn is_valid_shunzi_combination(tile: &Tile, position: &ClaimedTilePosition) -> bool {
        match position {
            // false: In case of
            // { claimed_tile: 8x, dazi: [9x, 10x] } or { claimed_tile: 9x, dazi: [10x, 11x] }
            ClaimedTilePosition::Low => !matches!(tile, 7 | 16 | 25 | 8 | 17 | 26),

            // false: In case of
            // { claimed_tile: 1x, dazi: [0x, 2x] } or { claimed_tile: 9x, dazi: [8x, 10x] }
            ClaimedTilePosition::Middle => !matches!(tile, 0 | 8 | 9 | 17 | 18 | 26),

            // false: In case of
            // { claimed_tile: 1x, dazi: [-1x, 0x] } or { claimed_tile: 2x, dazi: [0x, 1x] }
            ClaimedTilePosition::High => !matches!(tile, 0 | 9 | 18 | 1 | 10 | 19),
        }
    }
}

const TILE_NAMES: [&str; NUM_TILE_INDEX] = [
    "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", // m
    "1p", "2p", "3p", "4p", "5p", "6p", "7p", "8p", "9p", // p
    "1s", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", // s
    "1z", "2z", "3z", "4z", "5z", "6z", "7z", // z
];

impl fmt::Display for Mianzi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mianzi::Shunzi(index, position) => {
                let position_str = match position {
                    ClaimedTilePosition::Low => "Low",
                    ClaimedTilePosition::Middle => "Middle",
                    ClaimedTilePosition::High => "High",
                };
                f.write_str(&format!(
                    "Chii {} {}",
                    TILE_NAMES[*index as usize], &position_str
                ))
            }
            Mianzi::Kezi(index) => f.write_str(&format!("Pon {}", TILE_NAMES[*index as usize])),
            Mianzi::Gangzi(index) => f.write_str(&format!("Kan {}", TILE_NAMES[*index as usize])),
        }
    }
}

impl fmt::Debug for Mianzi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_shunzi() {
        let shunzi_1m_low = Mianzi::Shunzi(0, ClaimedTilePosition::Low);
        let shunzi_7m_low = Mianzi::Shunzi(6, ClaimedTilePosition::Low);
        let shunzi_1p_low = Mianzi::Shunzi(0 + 9, ClaimedTilePosition::Low);
        let shunzi_7p_low = Mianzi::Shunzi(6 + 9, ClaimedTilePosition::Low);
        let shunzi_1s_low = Mianzi::Shunzi(0 + 18, ClaimedTilePosition::Low);
        let shunzi_7s_low = Mianzi::Shunzi(6 + 18, ClaimedTilePosition::Low);
        let shunzi_2m_middle = Mianzi::Shunzi(1, ClaimedTilePosition::Middle);
        let shunzi_8m_middle = Mianzi::Shunzi(7, ClaimedTilePosition::Middle);
        let shunzi_2p_middle = Mianzi::Shunzi(1 + 9, ClaimedTilePosition::Middle);
        let shunzi_8p_middle = Mianzi::Shunzi(7 + 9, ClaimedTilePosition::Middle);
        let shunzi_2s_middle = Mianzi::Shunzi(1 + 18, ClaimedTilePosition::Middle);
        let shunzi_8s_middle = Mianzi::Shunzi(7 + 18, ClaimedTilePosition::Middle);
        let shunzi_3m_high = Mianzi::Shunzi(2, ClaimedTilePosition::High);
        let shunzi_9m_high = Mianzi::Shunzi(8, ClaimedTilePosition::High);
        let shunzi_3p_high = Mianzi::Shunzi(2 + 9, ClaimedTilePosition::High);
        let shunzi_9p_high = Mianzi::Shunzi(8 + 9, ClaimedTilePosition::High);
        let shunzi_3s_high = Mianzi::Shunzi(2 + 18, ClaimedTilePosition::High);
        let shunzi_9s_high = Mianzi::Shunzi(8 + 18, ClaimedTilePosition::High);

        assert_eq!(shunzi_1m_low.validate().unwrap(), ());
        assert_eq!(shunzi_7m_low.validate().unwrap(), ());
        assert_eq!(shunzi_1p_low.validate().unwrap(), ());
        assert_eq!(shunzi_7p_low.validate().unwrap(), ());
        assert_eq!(shunzi_1s_low.validate().unwrap(), ());
        assert_eq!(shunzi_7s_low.validate().unwrap(), ());
        assert_eq!(shunzi_2m_middle.validate().unwrap(), ());
        assert_eq!(shunzi_8m_middle.validate().unwrap(), ());
        assert_eq!(shunzi_2p_middle.validate().unwrap(), ());
        assert_eq!(shunzi_8p_middle.validate().unwrap(), ());
        assert_eq!(shunzi_2s_middle.validate().unwrap(), ());
        assert_eq!(shunzi_8s_middle.validate().unwrap(), ());
        assert_eq!(shunzi_3m_high.validate().unwrap(), ());
        assert_eq!(shunzi_9m_high.validate().unwrap(), ());
        assert_eq!(shunzi_3p_high.validate().unwrap(), ());
        assert_eq!(shunzi_9p_high.validate().unwrap(), ());
        assert_eq!(shunzi_3s_high.validate().unwrap(), ());
        assert_eq!(shunzi_9s_high.validate().unwrap(), ());
    }

    #[test]
    fn invalid_shunzi() {
        let shunzi_1z_low = Mianzi::Shunzi(MAX_SHUPAI_INDEX + 1, ClaimedTilePosition::Low);
        let shunzi_8m_low = Mianzi::Shunzi(7, ClaimedTilePosition::Low);
        let shunzi_9m_low = Mianzi::Shunzi(8, ClaimedTilePosition::Low);
        let shunzi_1m_middle = Mianzi::Shunzi(0, ClaimedTilePosition::Middle);
        let shunzi_9m_middle = Mianzi::Shunzi(8, ClaimedTilePosition::Middle);
        let shunzi_1m_high = Mianzi::Shunzi(0, ClaimedTilePosition::High);
        let shunzi_2m_high = Mianzi::Shunzi(1, ClaimedTilePosition::High);

        assert!(matches!(
            shunzi_1z_low.validate().unwrap_err(),
            InvalidMianziError::ShunziWithZipai(27)
        ));
        assert!(matches!(
            shunzi_8m_low.validate().unwrap_err(),
            InvalidMianziError::InvalidShunziCombination(7, ClaimedTilePosition::Low)
        ));
        assert!(matches!(
            shunzi_9m_low.validate().unwrap_err(),
            InvalidMianziError::InvalidShunziCombination(8, ClaimedTilePosition::Low)
        ));
        assert!(matches!(
            shunzi_1m_middle.validate().unwrap_err(),
            InvalidMianziError::InvalidShunziCombination(0, ClaimedTilePosition::Middle)
        ));
        assert!(matches!(
            shunzi_9m_middle.validate().unwrap_err(),
            InvalidMianziError::InvalidShunziCombination(8, ClaimedTilePosition::Middle)
        ));
        assert!(matches!(
            shunzi_1m_high.validate().unwrap_err(),
            InvalidMianziError::InvalidShunziCombination(0, ClaimedTilePosition::High)
        ));
        assert!(matches!(
            shunzi_2m_high.validate().unwrap_err(),
            InvalidMianziError::InvalidShunziCombination(1, ClaimedTilePosition::High)
        ));
    }

    #[test]
    fn valid_kezi() {
        let kezi_1 = Mianzi::Kezi(0);
        let kezi_2 = Mianzi::Kezi(MAX_TILE_INDEX);

        assert_eq!(kezi_1.validate().unwrap(), ());
        assert_eq!(kezi_2.validate().unwrap(), ());
    }

    #[test]
    fn invalid_kezi() {
        let kezi_1 = Mianzi::Kezi(MAX_TILE_INDEX + 1);

        assert!(matches!(
            kezi_1.validate().unwrap_err(),
            InvalidMianziError::IndexOutOfRange(34)
        ));
    }

    #[test]
    fn valid_gangzi() {
        let gangzi_1 = Mianzi::Gangzi(0);
        let gangzi_2 = Mianzi::Gangzi(MAX_TILE_INDEX);

        assert_eq!(gangzi_1.validate().unwrap(), ());
        assert_eq!(gangzi_2.validate().unwrap(), ());
    }

    #[test]
    fn invalid_gangzi() {
        let gangzi_1 = Mianzi::Gangzi(MAX_TILE_INDEX + 1);

        assert!(matches!(
            gangzi_1.validate().unwrap_err(),
            InvalidMianziError::IndexOutOfRange(34)
        ));
    }

    #[test]
    fn shunzi_display() {
        let shunzi_low = Mianzi::Shunzi(0, ClaimedTilePosition::Low);
        assert_eq!(format!("{}", shunzi_low), "Chii 1m Low");

        let shunzi_middle = Mianzi::Shunzi(1, ClaimedTilePosition::Middle);
        assert_eq!(format!("{}", shunzi_middle), "Chii 2m Middle");

        let shunzi_high = Mianzi::Shunzi(2, ClaimedTilePosition::High);
        assert_eq!(format!("{}", shunzi_high), "Chii 3m High");
    }

    #[test]
    fn kezi_display() {
        let kezi = Mianzi::Kezi(0);
        assert_eq!(format!("{}", kezi), "Pon 1m");
    }

    #[test]
    fn gangzi_display() {
        let gangzi = Mianzi::Gangzi(0);
        assert_eq!(format!("{}", gangzi), "Kan 1m");
    }
}
