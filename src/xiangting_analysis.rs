// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::analysis::XiangtingAnalysis;
use crate::fulu_mianzi::FuluMianzi;
use crate::shoupai::{Shoupai, XiangtingError};
use crate::tile::TileCounts;

pub fn analyze_xiangting(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<XiangtingAnalysis, XiangtingError> {
    let shoupai = Shoupai::new(bingpai, fulu_mianzi_list)?;
    unimplemented!("")
}
