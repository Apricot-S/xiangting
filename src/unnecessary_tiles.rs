// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use super::standard;
use crate::bingpai::{Bingpai, Bingpai3p, BingpaiError};
use crate::config::PlayerCount;
use crate::tile::{TileCounts, TileFlags};
use std::cmp::Ordering;

#[inline]
pub fn calculate_unnecessary_tiles(
    bingpai: &TileCounts,
    player_count: &PlayerCount,
) -> Result<(u8, TileFlags), BingpaiError> {
    match player_count {
        PlayerCount::Four => calculate_unnecessary_tiles_4p(bingpai),
        PlayerCount::Three => calculate_unnecessary_tiles_3p(bingpai),
    }
}

fn calculate_unnecessary_tiles_4p(
    tile_counts: &TileCounts,
) -> Result<(u8, TileFlags), BingpaiError> {
    let bingpai = Bingpai::new(tile_counts)?;

    let (mut replacement_number, mut unnecessary_tiles) =
        standard::calculate_unnecessary_tiles(&bingpai);

    let (r1, u1) = qiduizi::calculate_unnecessary_tiles(&bingpai);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            unnecessary_tiles = u1;
        }
        Ordering::Equal => unnecessary_tiles |= u1,
        Ordering::Greater => (),
    }

    let (r2, u2) = shisanyao::calculate_unnecessary_tiles(&bingpai);
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

fn calculate_unnecessary_tiles_3p(
    tile_counts: &TileCounts,
) -> Result<(u8, TileFlags), BingpaiError> {
    let bingpai_3p = Bingpai3p::new(tile_counts)?;

    let (mut replacement_number, mut unnecessary_tiles) =
        standard::calculate_unnecessary_tiles_3p(&bingpai_3p);

    let (r1, u1) = qiduizi::calculate_unnecessary_tiles_3p(&bingpai_3p);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            unnecessary_tiles = u1;
        }
        Ordering::Equal => unnecessary_tiles |= u1,
        Ordering::Greater => (),
    }

    let bingpai = bingpai_3p.into();

    let (r2, u2) = shisanyao::calculate_unnecessary_tiles(&bingpai);
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
