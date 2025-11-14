// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

/// The number of players.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerCount {
    /// Four-player mahjong.
    Four,
    /// Three-player mahjong.
    Three,
}
