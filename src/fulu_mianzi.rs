// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::constants::{MAX_SHUPAI_INDEX, MAX_TILE_INDEX, NUM_TILE_INDEX};
use crate::tile::Tile;
use std::fmt;
use thiserror::Error;

/// Position of the claimed tile in the melded sequence.
/// Used in [`FuluMianzi::Shunzi`](self::FuluMianzi::Shunzi).
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Clone, PartialEq, Eq)]
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
pub enum FuluMianziError {
    /// The tile index is outside the range of 0 to 33.
    #[error("tile index must be between 0 and 33 but was {0}")]
    IndexOutOfRange(Tile),
    /// An attempt was made to create a sequence using honors (字牌).
    #[error("a sequence cannot be made with honors: {0}")]
    ShunziWithZipai(Tile),
    /// The tile and position combination cannot form a valid sequence.
    #[error("a sequence cannot be made with {0} and {1:?}")]
    InvalidShunziCombination(Tile, ClaimedTilePosition),
    /// This meld cannot be used in 3-player mahjong (2m to 8m or sequence).
    #[error("{0} cannot be used in 3-player mahjong")]
    InvalidFuluMianziFor3Player(FuluMianzi),
}

impl FuluMianzi {
    pub(crate) fn validate(&self) -> Result<(), FuluMianziError> {
        match self {
            FuluMianzi::Shunzi(t, p) => {
                if *t > MAX_SHUPAI_INDEX {
                    if *t > MAX_TILE_INDEX {
                        return Err(FuluMianziError::IndexOutOfRange(*t));
                    }
                    return Err(FuluMianziError::ShunziWithZipai(*t));
                }
                if !FuluMianzi::is_valid_shunzi_combination(t, p) {
                    return Err(FuluMianziError::InvalidShunziCombination(*t, p.clone()));
                }
            }
            FuluMianzi::Kezi(t) | FuluMianzi::Gangzi(t) => {
                if *t > MAX_TILE_INDEX {
                    return Err(FuluMianziError::IndexOutOfRange(*t));
                }
            }
        }
        Ok(())
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
    fn validate_ok_shunzi_1m_23m() {
        assert!(matches!(
            FuluMianzi::Shunzi(0, ClaimedTilePosition::Low).validate(),
            Ok(())
        ));
    }

    #[test]
    fn validate_ok_shunzi_7m_89m() {
        assert!(matches!(
            FuluMianzi::Shunzi(6, ClaimedTilePosition::Low).validate(),
            Ok(())
        ));
    }

    #[test]
    fn validate_ok_shunzi_2m_13m() {
        assert!(matches!(
            FuluMianzi::Shunzi(1, ClaimedTilePosition::Middle).validate(),
            Ok(())
        ));
    }

    #[test]
    fn validate_ok_shunzi_8m_79m() {
        assert!(matches!(
            FuluMianzi::Shunzi(7, ClaimedTilePosition::Middle).validate(),
            Ok(())
        ));
    }

    #[test]
    fn validate_ok_shunzi_3m_12m() {
        assert!(matches!(
            FuluMianzi::Shunzi(2, ClaimedTilePosition::High).validate(),
            Ok(())
        ));
    }

    #[test]
    fn validate_ok_shunzi_9m_78m() {
        assert!(matches!(
            FuluMianzi::Shunzi(8, ClaimedTilePosition::High).validate(),
            Ok(())
        ));
    }

    #[test]
    fn validate_ok_kezi() {
        assert!(matches!(FuluMianzi::Kezi(33).validate(), Ok(())));
    }

    #[test]
    fn validate_ok_gangzi() {
        assert!(matches!(FuluMianzi::Gangzi(33).validate(), Ok(())));
    }

    #[test]
    fn validate_err_shunzi_8z() {
        assert!(matches!(
            FuluMianzi::Shunzi(34, ClaimedTilePosition::Low).validate(),
            Err(FuluMianziError::IndexOutOfRange(34))
        ));
    }

    #[test]
    fn validate_err_shunzi_1z() {
        assert!(matches!(
            FuluMianzi::Shunzi(27, ClaimedTilePosition::Low).validate(),
            Err(FuluMianziError::ShunziWithZipai(27))
        ));
    }

    #[test]
    fn validate_err_shunzi_8m_910m() {
        assert!(matches!(
            FuluMianzi::Shunzi(7, ClaimedTilePosition::Low).validate(),
            Err(FuluMianziError::InvalidShunziCombination(
                7,
                ClaimedTilePosition::Low
            ))
        ));
    }

    #[test]
    fn validate_err_shunzi_9m_1011m() {
        assert!(matches!(
            FuluMianzi::Shunzi(8, ClaimedTilePosition::Low).validate(),
            Err(FuluMianziError::InvalidShunziCombination(
                8,
                ClaimedTilePosition::Low
            ))
        ));
    }

    #[test]
    fn validate_err_shunzi_1m_02m() {
        assert!(matches!(
            FuluMianzi::Shunzi(0, ClaimedTilePosition::Middle).validate(),
            Err(FuluMianziError::InvalidShunziCombination(
                0,
                ClaimedTilePosition::Middle
            ))
        ));
    }

    #[test]
    fn validate_err_shunzi_9m_810m() {
        assert!(matches!(
            FuluMianzi::Shunzi(8, ClaimedTilePosition::Middle).validate(),
            Err(FuluMianziError::InvalidShunziCombination(
                8,
                ClaimedTilePosition::Middle
            ))
        ));
    }

    #[test]
    fn validate_err_shunzi_1m_minus10m() {
        assert!(matches!(
            FuluMianzi::Shunzi(0, ClaimedTilePosition::High).validate(),
            Err(FuluMianziError::InvalidShunziCombination(
                0,
                ClaimedTilePosition::High
            ))
        ));
    }

    #[test]
    fn validate_err_shunzi_2m_01m() {
        assert!(matches!(
            FuluMianzi::Shunzi(1, ClaimedTilePosition::High).validate(),
            Err(FuluMianziError::InvalidShunziCombination(
                1,
                ClaimedTilePosition::High
            ))
        ));
    }

    #[test]
    fn validate_err_kezi_8z() {
        assert!(matches!(
            FuluMianzi::Kezi(34).validate(),
            Err(FuluMianziError::IndexOutOfRange(34))
        ));
    }

    #[test]
    fn validate_err_gangzi_8z() {
        assert!(matches!(
            FuluMianzi::Gangzi(34).validate(),
            Err(FuluMianziError::IndexOutOfRange(34))
        ));
    }

    #[test]
    fn shunzi_display_low() {
        let shunzi_low = FuluMianzi::Shunzi(0, ClaimedTilePosition::Low);
        assert_eq!(format!("{}", shunzi_low), "Chii-1m-Low");
    }

    #[test]
    fn shunzi_display_middle() {
        let shunzi_middle = FuluMianzi::Shunzi(1, ClaimedTilePosition::Middle);
        assert_eq!(format!("{}", shunzi_middle), "Chii-2m-Middle");
    }

    #[test]
    fn shunzi_display_high() {
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
