// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use super::standard;
use crate::bingpai::{Bingpai, Bingpai3p};
use crate::config::PlayerCount;
use crate::error::XiangtingError;
use crate::tile::{TileCounts, TileFlags};
use std::cmp::Ordering;

#[inline]
pub fn calculate_necessary_tiles(
    bingpai: &TileCounts,
    player_count: &PlayerCount,
) -> Result<(u8, TileFlags), XiangtingError> {
    match player_count {
        PlayerCount::Four => calculate_necessary_tiles_4p(bingpai),
        PlayerCount::Three => calculate_necessary_tiles_3p(bingpai),
    }
}

fn calculate_necessary_tiles_4p(
    tile_counts: &TileCounts,
) -> Result<(u8, TileFlags), XiangtingError> {
    let bingpai = Bingpai::new(tile_counts)?;

    let (mut replacement_number, mut necessary_tiles) =
        standard::calculate_necessary_tiles(&bingpai);

    let (r1, n1) = qiduizi::calculate_necessary_tiles(&bingpai);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            necessary_tiles = n1;
        }
        Ordering::Equal => necessary_tiles |= n1,
        Ordering::Greater => (),
    }

    let (r2, n2) = shisanyao::calculate_necessary_tiles(&bingpai);
    match r2.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r2;
            necessary_tiles = n2;
        }
        Ordering::Equal => necessary_tiles |= n2,
        Ordering::Greater => (),
    }

    Ok((replacement_number, necessary_tiles))
}

fn calculate_necessary_tiles_3p(
    tile_counts: &TileCounts,
) -> Result<(u8, TileFlags), XiangtingError> {
    let bingpai_3p = Bingpai3p::new(tile_counts)?;

    let (mut replacement_number, mut necessary_tiles) =
        standard::calculate_necessary_tiles_3p(&bingpai_3p);

    let (r1, n1) = qiduizi::calculate_necessary_tiles_3p(&bingpai_3p);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            necessary_tiles = n1;
        }
        Ordering::Equal => necessary_tiles |= n1,
        Ordering::Greater => (),
    }

    let bingpai = bingpai_3p.into();

    let (r2, n2) = shisanyao::calculate_necessary_tiles(&bingpai);
    match r2.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r2;
            necessary_tiles = n2;
        }
        Ordering::Equal => necessary_tiles |= n2,
        Ordering::Greater => (),
    }

    Ok((replacement_number, necessary_tiles))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::BingpaiError;
    use crate::test_utils::FromTileCode;

    #[test]
    fn calculate_necessary_tiles_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("123m456p789s1122z");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("12z"));
    }

    #[test]
    fn calculate_necessary_tiles_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1188m288p55s1177z");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("2p"));
    }

    #[test]
    fn calculate_necessary_tiles_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_empty() {
        let bingpai = TileCounts::from_code("");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            ret,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(0)))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_ok_bingpai_1_tile() {
        let bingpai = TileCounts::from_code("1m");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(ret.is_ok());
    }

    #[test]
    fn calculate_necessary_tiles_ok_bingpai_2_tiles() {
        let bingpai = TileCounts::from_code("2p3s");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(ret.is_ok());
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_3_tiles() {
        let bingpai = TileCounts::from_code("2p3s7z");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            ret,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(3)))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_15_tiles() {
        let bingpai = TileCounts::from_code("111222333444555m");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            ret,
            Err(XiangtingError::Bingpai(BingpaiError::TooManyTiles(15)))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_5_same_tiles() {
        let bingpai = TileCounts::from_code("11111m");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            ret,
            Err(XiangtingError::Bingpai(BingpaiError::TooManyCopies {
                tile: 0,
                count: 5
            }))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("111m456p789s1122z");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Three);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("12z"));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1199m288p55s1177z");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Three);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("2p"));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Three);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_err_bingpai_2m() {
        let bingpai = TileCounts::from_code("2m");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Three);
        assert!(matches!(
            ret,
            Err(XiangtingError::Bingpai(
                BingpaiError::InvalidTileForThreePlayer(1)
            ))
        ));
    }
}
