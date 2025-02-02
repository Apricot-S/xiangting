// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::{MAX_SHUPAI_INDEX, MAX_TILE_INDEX, NUM_TILE_INDEX};
use crate::tile::Tile;
use std::fmt;
use thiserror::Error;

/// Position of the claimed tile in the melded sequence.
/// Used in [`FuluMianzi::Shunzi`](self::FuluMianzi::Shunzi).
#[derive(Debug, Clone)]
pub enum ClaimedTilePosition {
    /// The claimed tile is the lowest in the sequence.
    /// For example, claiming a 3 to form a sequence of 3-4-5.
    Low,
    /// The claimed tile is the middle in the sequence.
    /// For example, claiming a 4 to form a sequence of 3-4-5.
    Middle,
    /// The claimed tile is the highest in the sequence.
    /// For example, claiming a 5 to form a sequence of 3-4-5.
    High,
}

/// 副露面子: Meld.
///
/// # Examples
///
/// ```
/// # use xiangting::{ClaimedTilePosition, FuluMianzi};
/// // 4-56p (Chii 4p Low)
/// let shunzi = FuluMianzi::Shunzi(12, ClaimedTilePosition::Low);
///
/// // 1-11z (Pon 1z)
/// let kezi = FuluMianzi::Kezi(27);
///
/// // 7-777s (Kan 7s)
/// let gangzi = FuluMianzi::Gangzi(24);
/// ```
#[derive(Clone)]
pub enum FuluMianzi {
    /// 順子: Sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// # use xiangting::{ClaimedTilePosition, FuluMianzi};
    /// // 1-23m (Chii 1m Low)
    /// let shunzi_low = FuluMianzi::Shunzi(0, ClaimedTilePosition::Low);
    ///
    /// // 2-13m (Chii 2m Middle)
    /// let shunzi_middle = FuluMianzi::Shunzi(1, ClaimedTilePosition::Middle);
    ///
    /// // 3-12m (Chii 3m High)
    /// let shunzi_high = FuluMianzi::Shunzi(2, ClaimedTilePosition::High);
    /// ```
    Shunzi(Tile, ClaimedTilePosition),
    /// 刻子: Triplet.
    ///
    /// # Examples
    ///
    /// ```
    /// # use xiangting::FuluMianzi;
    /// // 1-11m (Pon 1m)
    /// let kezi = FuluMianzi::Kezi(0);
    /// ```
    Kezi(Tile),
    /// 槓子: Quad.
    ///
    /// # Examples
    ///
    /// ```
    /// # use xiangting::FuluMianzi;
    /// // 1-111m (Kan 1m)
    /// let gangzi = FuluMianzi::Gangzi(0);
    /// ```
    Gangzi(Tile),
}

/// Errors that occur when an invalid meld is provided.
#[derive(Debug, Error)]
pub enum InvalidFuluMianziError {
    /// The tile index is outside the range of 0 to 33.
    #[error("tile index must be between 0 and 33 but was {0}")]
    IndexOutOfRange(Tile),
    /// An attempt was made to create a sequence using honors (字牌).
    #[error("a sequence cannot be made with honors ({0})")]
    ShunziWithZipai(Tile),
    /// The tile and position combination cannot form a valid sequence.
    #[error("a sequence cannot be made with {0} and {1:?}")]
    InvalidShunziCombination(Tile, ClaimedTilePosition),
    /// This meld cannot be used in 3-player mahjong (2m to 8m or sequence).
    #[error("{0} cannot be used in 3-player mahjong")]
    InvalidFuluMianziFor3Player(FuluMianzi),
}

impl FuluMianzi {
    pub(crate) fn validate(&self) -> Result<(), InvalidFuluMianziError> {
        match self {
            FuluMianzi::Shunzi(tile, position) => {
                if *tile > MAX_SHUPAI_INDEX {
                    if *tile > MAX_TILE_INDEX {
                        return Err(InvalidFuluMianziError::IndexOutOfRange(*tile));
                    }
                    return Err(InvalidFuluMianziError::ShunziWithZipai(*tile));
                }
                if !FuluMianzi::is_valid_shunzi_combination(tile, position) {
                    return Err(InvalidFuluMianziError::InvalidShunziCombination(
                        *tile,
                        position.clone(),
                    ));
                }
                Ok(())
            }
            FuluMianzi::Kezi(tile) | FuluMianzi::Gangzi(tile) => {
                if *tile > MAX_TILE_INDEX {
                    return Err(InvalidFuluMianziError::IndexOutOfRange(*tile));
                }
                Ok(())
            }
        }
    }

    pub(crate) fn validate_3_player(&self) -> Result<(), InvalidFuluMianziError> {
        match self {
            FuluMianzi::Shunzi(_, _) => {
                return Err(InvalidFuluMianziError::InvalidFuluMianziFor3Player(
                    self.clone(),
                ));
            }
            FuluMianzi::Kezi(t) | FuluMianzi::Gangzi(t) => {
                if (1..8).contains(t) {
                    return Err(InvalidFuluMianziError::InvalidFuluMianziFor3Player(
                        self.clone(),
                    ));
                }
            }
        }

        self.validate()
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

impl fmt::Display for FuluMianzi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FuluMianzi::Shunzi(index, position) => {
                let position_str = match position {
                    ClaimedTilePosition::Low => "Low",
                    ClaimedTilePosition::Middle => "Middle",
                    ClaimedTilePosition::High => "High",
                };
                f.write_str(&format!(
                    "Chii-{}-{}",
                    TILE_NAMES[*index as usize], &position_str
                ))
            }
            FuluMianzi::Kezi(index) => f.write_str(&format!("Pon-{}", TILE_NAMES[*index as usize])),
            FuluMianzi::Gangzi(index) => {
                f.write_str(&format!("Kan-{}", TILE_NAMES[*index as usize]))
            }
        }
    }
}

impl fmt::Debug for FuluMianzi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_shunzi() {
        let shunzi_1m_low = FuluMianzi::Shunzi(0, ClaimedTilePosition::Low);
        let shunzi_7m_low = FuluMianzi::Shunzi(6, ClaimedTilePosition::Low);
        let shunzi_1p_low = FuluMianzi::Shunzi(0 + 9, ClaimedTilePosition::Low);
        let shunzi_7p_low = FuluMianzi::Shunzi(6 + 9, ClaimedTilePosition::Low);
        let shunzi_1s_low = FuluMianzi::Shunzi(0 + 18, ClaimedTilePosition::Low);
        let shunzi_7s_low = FuluMianzi::Shunzi(6 + 18, ClaimedTilePosition::Low);
        let shunzi_2m_middle = FuluMianzi::Shunzi(1, ClaimedTilePosition::Middle);
        let shunzi_8m_middle = FuluMianzi::Shunzi(7, ClaimedTilePosition::Middle);
        let shunzi_2p_middle = FuluMianzi::Shunzi(1 + 9, ClaimedTilePosition::Middle);
        let shunzi_8p_middle = FuluMianzi::Shunzi(7 + 9, ClaimedTilePosition::Middle);
        let shunzi_2s_middle = FuluMianzi::Shunzi(1 + 18, ClaimedTilePosition::Middle);
        let shunzi_8s_middle = FuluMianzi::Shunzi(7 + 18, ClaimedTilePosition::Middle);
        let shunzi_3m_high = FuluMianzi::Shunzi(2, ClaimedTilePosition::High);
        let shunzi_9m_high = FuluMianzi::Shunzi(8, ClaimedTilePosition::High);
        let shunzi_3p_high = FuluMianzi::Shunzi(2 + 9, ClaimedTilePosition::High);
        let shunzi_9p_high = FuluMianzi::Shunzi(8 + 9, ClaimedTilePosition::High);
        let shunzi_3s_high = FuluMianzi::Shunzi(2 + 18, ClaimedTilePosition::High);
        let shunzi_9s_high = FuluMianzi::Shunzi(8 + 18, ClaimedTilePosition::High);

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
    fn invalid_shunzi_out_of_range() {
        let shunzi_8z_low = FuluMianzi::Shunzi(MAX_TILE_INDEX + 1, ClaimedTilePosition::Low);

        assert!(matches!(
            shunzi_8z_low.validate().unwrap_err(),
            InvalidFuluMianziError::IndexOutOfRange(34)
        ));
    }

    #[test]
    fn invalid_shunzi_zipai() {
        let shunzi_1z_low = FuluMianzi::Shunzi(MAX_SHUPAI_INDEX + 1, ClaimedTilePosition::Low);
        let shunzi_7z_high = FuluMianzi::Shunzi(MAX_TILE_INDEX, ClaimedTilePosition::High);

        assert!(matches!(
            shunzi_1z_low.validate().unwrap_err(),
            InvalidFuluMianziError::ShunziWithZipai(27)
        ));
        assert!(matches!(
            shunzi_7z_high.validate().unwrap_err(),
            InvalidFuluMianziError::ShunziWithZipai(33)
        ));
    }

    #[test]
    fn invalid_shunzi_combination() {
        let shunzi_8m_low = FuluMianzi::Shunzi(7, ClaimedTilePosition::Low);
        let shunzi_9m_low = FuluMianzi::Shunzi(8, ClaimedTilePosition::Low);
        let shunzi_1m_middle = FuluMianzi::Shunzi(0, ClaimedTilePosition::Middle);
        let shunzi_9m_middle = FuluMianzi::Shunzi(8, ClaimedTilePosition::Middle);
        let shunzi_1m_high = FuluMianzi::Shunzi(0, ClaimedTilePosition::High);
        let shunzi_2m_high = FuluMianzi::Shunzi(1, ClaimedTilePosition::High);

        assert!(matches!(
            shunzi_8m_low.validate().unwrap_err(),
            InvalidFuluMianziError::InvalidShunziCombination(7, ClaimedTilePosition::Low)
        ));
        assert!(matches!(
            shunzi_9m_low.validate().unwrap_err(),
            InvalidFuluMianziError::InvalidShunziCombination(8, ClaimedTilePosition::Low)
        ));
        assert!(matches!(
            shunzi_1m_middle.validate().unwrap_err(),
            InvalidFuluMianziError::InvalidShunziCombination(0, ClaimedTilePosition::Middle)
        ));
        assert!(matches!(
            shunzi_9m_middle.validate().unwrap_err(),
            InvalidFuluMianziError::InvalidShunziCombination(8, ClaimedTilePosition::Middle)
        ));
        assert!(matches!(
            shunzi_1m_high.validate().unwrap_err(),
            InvalidFuluMianziError::InvalidShunziCombination(0, ClaimedTilePosition::High)
        ));
        assert!(matches!(
            shunzi_2m_high.validate().unwrap_err(),
            InvalidFuluMianziError::InvalidShunziCombination(1, ClaimedTilePosition::High)
        ));
    }

    #[test]
    fn valid_kezi() {
        let kezi_1 = FuluMianzi::Kezi(0);
        let kezi_2 = FuluMianzi::Kezi(MAX_TILE_INDEX);

        assert_eq!(kezi_1.validate().unwrap(), ());
        assert_eq!(kezi_2.validate().unwrap(), ());
    }

    #[test]
    fn invalid_kezi() {
        let kezi_1 = FuluMianzi::Kezi(MAX_TILE_INDEX + 1);

        assert!(matches!(
            kezi_1.validate().unwrap_err(),
            InvalidFuluMianziError::IndexOutOfRange(34)
        ));
    }

    #[test]
    fn valid_gangzi() {
        let gangzi_1 = FuluMianzi::Gangzi(0);
        let gangzi_2 = FuluMianzi::Gangzi(MAX_TILE_INDEX);

        assert_eq!(gangzi_1.validate().unwrap(), ());
        assert_eq!(gangzi_2.validate().unwrap(), ());
    }

    #[test]
    fn invalid_gangzi() {
        let gangzi_1 = FuluMianzi::Gangzi(MAX_TILE_INDEX + 1);

        assert!(matches!(
            gangzi_1.validate().unwrap_err(),
            InvalidFuluMianziError::IndexOutOfRange(34)
        ));
    }

    #[test]
    fn shunzi_display() {
        let shunzi_low = FuluMianzi::Shunzi(0, ClaimedTilePosition::Low);
        assert_eq!(format!("{}", shunzi_low), "Chii-1m-Low");

        let shunzi_middle = FuluMianzi::Shunzi(1, ClaimedTilePosition::Middle);
        assert_eq!(format!("{}", shunzi_middle), "Chii-2m-Middle");

        let shunzi_high = FuluMianzi::Shunzi(2, ClaimedTilePosition::High);
        assert_eq!(format!("{}", shunzi_high), "Chii-3m-High");
    }

    #[test]
    fn kezi_display() {
        let kezi = FuluMianzi::Kezi(0);
        assert_eq!(format!("{}", kezi), "Pon-1m");
    }

    #[test]
    fn gangzi_display() {
        let gangzi = FuluMianzi::Gangzi(0);
        assert_eq!(format!("{}", gangzi), "Kan-1m");
    }
}
