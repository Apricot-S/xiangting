// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use crate::fulu_mianzi::FuluMianzi;
use crate::shoupai::{Shoupai, XiangtingError};
use crate::tile::{TileCounts, TileFlags};

pub fn calculate_unnecessary_tiles(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<(u8, TileFlags), XiangtingError> {
    let shoupai = Shoupai::new(bingpai, fulu_mianzi_list)?;

    let (r1, u1) = qiduizi::calculate_unnecessary_tiles(&shoupai);

    unimplemented!("")
}
