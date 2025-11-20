// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::BingpaiError;
use thiserror::Error;

/// Errors that can occur when calculating the deficiency number for a hand.
#[derive(Debug, Error)]
pub enum XiangtingError {
    /// The pure hand is invalid.
    #[error("pure hand is invalid: {0}")]
    Bingpai(#[from] BingpaiError),
}
