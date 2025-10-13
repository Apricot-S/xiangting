// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XiangtingAnalysis {
    pub replacement_number: u8,
    pub necessary_tiles: u64,
    pub unnecessary_tiles: u64,
}
