// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

/// The number of players.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerCount {
    /// Four-player mahjong (the standard rules).
    Four,
    /// Three-player mahjong.
    ///
    /// - Tiles from 2m (二萬) to 8m (八萬) are not used.
    Three,
}
