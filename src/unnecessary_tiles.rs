// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use crate::error::XiangtingError;
use crate::fulu_mianzi::FuluMianzi;
use crate::shoupai::{Shoupai, Shoupai3p};
use crate::tile::{TileCounts, TileFlags};
use std::cmp::Ordering;

pub fn calculate_unnecessary_tiles(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<(u8, TileFlags), XiangtingError> {
    let shoupai = Shoupai::new(bingpai, fulu_mianzi_list)?;

    let (mut replacement_number, mut unnecessary_tiles) = (u8::MAX, 0u64);

    let (r1, u1) = qiduizi::calculate_unnecessary_tiles(&shoupai);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            unnecessary_tiles = u1;
        }
        Ordering::Equal => unnecessary_tiles |= u1,
        Ordering::Greater => (),
    }

    let (r2, u2) = shisanyao::calculate_unnecessary_tiles(&shoupai);
    match r2.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r2;
            unnecessary_tiles = u2;
        }
        Ordering::Equal => unnecessary_tiles |= u2,
        Ordering::Greater => (),
    }

    Ok((replacement_number, unnecessary_tiles))
}

pub fn calculate_unnecessary_tiles_3_player(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<(u8, TileFlags), XiangtingError> {
    let shoupai_3p = Shoupai3p::new(bingpai, fulu_mianzi_list)?;

    let (mut replacement_number, mut unnecessary_tiles) = (u8::MAX, 0u64);

    let (r1, u1) = qiduizi::calculate_unnecessary_tiles_3_player(&shoupai_3p);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            unnecessary_tiles = u1;
        }
        Ordering::Equal => unnecessary_tiles |= u1,
        Ordering::Greater => (),
    }

    let shoupai = shoupai_3p.into();

    let (r2, u2) = shisanyao::calculate_unnecessary_tiles(&shoupai);
    match r2.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r2;
            unnecessary_tiles = u2;
        }
        Ordering::Equal => unnecessary_tiles |= u2,
        Ordering::Greater => (),
    }

    Ok((replacement_number, unnecessary_tiles))
}
