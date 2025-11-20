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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::BingpaiError;
    use crate::test_utils::FromTileCode;

    #[test]
    fn calculate_unnecessary_tiles_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("123m456p789s1122z");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        let (replacement_number, unnecessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1188m288p55s1177z");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        let (replacement_number, unnecessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        let (replacement_number, unnecessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_err_bingpai_empty() {
        let bingpai = TileCounts::from_code("");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(ret, Err(BingpaiError::InvalidTileCount(0))));
    }

    #[test]
    fn calculate_unnecessary_tiles_ok_bingpai_1_tile() {
        let bingpai = TileCounts::from_code("1m");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(ret.is_ok());
    }

    #[test]
    fn calculate_unnecessary_tiles_ok_bingpai_2_tiles() {
        let bingpai = TileCounts::from_code("2p3s");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(ret.is_ok());
    }

    #[test]
    fn calculate_unnecessary_tiles_err_bingpai_3_tiles() {
        let bingpai = TileCounts::from_code("2p3s7z");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(ret, Err(BingpaiError::InvalidTileCount(3))));
    }

    #[test]
    fn calculate_unnecessary_tiles_err_bingpai_15_tiles() {
        let bingpai = TileCounts::from_code("111222333444555m");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(ret, Err(BingpaiError::TooManyTiles(15))));
    }

    #[test]
    fn calculate_unnecessary_tiles_err_bingpai_5_same_tiles() {
        let bingpai = TileCounts::from_code("11111m");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            ret,
            Err(BingpaiError::TooManyCopies { tile: 0, count: 5 })
        ));
    }

    #[test]
    fn calculate_unnecessary_tiles_3_player_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("111m456p789s1122z");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Three);
        let (replacement_number, unnecessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_3_player_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1199m288p55s1177z");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Three);
        let (replacement_number, unnecessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_3_player_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Three);
        let (replacement_number, unnecessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_3_player_err_bingpai_2m() {
        let bingpai = TileCounts::from_code("2m");
        let ret = calculate_unnecessary_tiles(&bingpai, &PlayerCount::Three);
        assert!(matches!(
            ret,
            Err(BingpaiError::InvalidTileForThreePlayer(1))
        ));
    }
}
