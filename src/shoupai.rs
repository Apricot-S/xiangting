// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::{BingpaiError, TileCountsExt};
use crate::tile::TileCounts;
use thiserror::Error;

/// Errors that occur when an invalid hand (手牌) is provided.
#[derive(Debug, Error)]
pub enum ShoupaiError {
    /// The hand contains an invalid pure hand.
    #[error("hand contains an invalid pure hand: {0}")]
    Bingpai(#[from] BingpaiError),
}

pub(crate) struct Shoupai<'a> {
    bingpai: &'a TileCounts,
    num_required_bingpai_mianzi: u8,
}

impl<'a> Shoupai<'a> {
    pub(crate) fn new(bingpai: &'a TileCounts) -> Result<Self, ShoupaiError> {
        let num_bingpai = bingpai.count()?;
        let num_required_bingpai_mianzi = num_bingpai / 3;

        Ok(Self {
            bingpai,
            num_required_bingpai_mianzi,
        })
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn bingpai(&self) -> &'a TileCounts {
        self.bingpai
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn num_required_bingpai_mianzi(&self) -> u8 {
        self.num_required_bingpai_mianzi
    }
}

pub(crate) struct Shoupai3p<'a> {
    bingpai: &'a TileCounts,
    num_required_bingpai_mianzi: u8,
}

impl<'a> Shoupai3p<'a> {
    pub(crate) fn new(bingpai: &'a TileCounts) -> Result<Self, ShoupaiError> {
        let num_bingpai = bingpai.count_3p()?;
        let num_required_bingpai_mianzi = num_bingpai / 3;

        Ok(Self {
            bingpai,
            num_required_bingpai_mianzi,
        })
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn bingpai(&self) -> &'a TileCounts {
        self.bingpai
    }

    #[inline(always)]
    #[must_use]
    pub(crate) fn num_required_bingpai_mianzi(&self) -> u8 {
        self.num_required_bingpai_mianzi
    }
}

impl<'a> From<Shoupai3p<'a>> for Shoupai<'a> {
    fn from(value: Shoupai3p<'a>) -> Self {
        Self {
            bingpai: value.bingpai,
            num_required_bingpai_mianzi: value.num_required_bingpai_mianzi,
        }
    }
}
