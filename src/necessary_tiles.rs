// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use super::standard;
use crate::error::XiangtingError;
use crate::fulu_mianzi::FuluMianzi;
use crate::shoupai::{Shoupai, Shoupai3p};
use crate::tile::{TileCounts, TileFlags};
use std::cmp::Ordering;

pub fn calculate_necessary_tiles(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<(u8, TileFlags), XiangtingError> {
    let shoupai = Shoupai::new(bingpai, fulu_mianzi_list)?;

    let (mut replacement_number, mut necessary_tiles) =
        standard::calculate_necessary_tiles(&shoupai);

    let (r1, n1) = qiduizi::calculate_necessary_tiles(&shoupai);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            necessary_tiles = n1;
        }
        Ordering::Equal => necessary_tiles |= n1,
        Ordering::Greater => (),
    }

    let (r2, n2) = shisanyao::calculate_necessary_tiles(&shoupai);
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

pub fn calculate_necessary_tiles_3_player(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<(u8, TileFlags), XiangtingError> {
    let shoupai_3p = Shoupai3p::new(bingpai, fulu_mianzi_list)?;

    let (mut replacement_number, mut necessary_tiles) =
        standard::calculate_necessary_tiles_3p(&shoupai_3p);

    let (r1, n1) = qiduizi::calculate_necessary_tiles_3p(&shoupai_3p);
    match r1.cmp(&replacement_number) {
        Ordering::Less => {
            replacement_number = r1;
            necessary_tiles = n1;
        }
        Ordering::Equal => necessary_tiles |= n1,
        Ordering::Greater => (),
    }

    let shoupai = shoupai_3p.into();

    let (r2, n2) = shisanyao::calculate_necessary_tiles(&shoupai);
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
    use crate::fulu_mianzi::{ClaimedTilePosition, FuluMianziError};
    use crate::shoupai::ShoupaiError;
    use crate::test_utils::FromTileCode;

    #[test]
    fn calculate_necessary_tiles_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("123m456p789s1122z");
        let ret = calculate_necessary_tiles(&bingpai, None);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("12z"));
    }

    #[test]
    fn calculate_necessary_tiles_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1188m288p55s1177z");
        let ret = calculate_necessary_tiles(&bingpai, None);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("2p"));
    }

    #[test]
    fn calculate_necessary_tiles_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let ret = calculate_necessary_tiles(&bingpai, None);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_empty() {
        let bingpai = TileCounts::from_code("");
        let replacement_number = calculate_necessary_tiles(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::InvalidTileCount(0)
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_ok_bingpai_1_tile() {
        let bingpai = TileCounts::from_code("1m");
        let replacement_number = calculate_necessary_tiles(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_necessary_tiles_ok_bingpai_2_tiles() {
        let bingpai = TileCounts::from_code("2p3s");
        let replacement_number = calculate_necessary_tiles(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_3_tiles() {
        let bingpai = TileCounts::from_code("2p3s7z");
        let replacement_number = calculate_necessary_tiles(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::InvalidTileCount(3)
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_15_tiles() {
        let bingpai = TileCounts::from_code("111222333444555m");
        let replacement_number = calculate_necessary_tiles(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::TooManyTiles(15)
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_5_same_tiles() {
        let bingpai = TileCounts::from_code("11111m");
        let replacement_number = calculate_necessary_tiles(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::TooManyCopies { tile: 0, count: 5 }
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_fulu_index_out_of_range() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(34)];
        let replacement_number = calculate_necessary_tiles(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::IndexOutOfRange(34)
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_fulu_shunzi_with_zipai() {
        let bingpai = TileCounts::from_code("1p");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(27, ClaimedTilePosition::Low)];
        let replacement_number = calculate_necessary_tiles(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::ShunziWithZipai(27)
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_fulu_invalid_shunzi_combination() {
        let bingpai = TileCounts::from_code("1p");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(0, ClaimedTilePosition::Middle)];
        let replacement_number = calculate_necessary_tiles(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::InvalidShunziCombination(0, ClaimedTilePosition::Middle)
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_shoupai_too_many_fulu_mianzi() {
        let bingpai = TileCounts::from_code("11122233344455m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(5)];
        let replacement_number = calculate_necessary_tiles(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::TooManyFuluMianzi {
                max: 0,
                count: 1
            }))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_err_shoupai_5_same_tiles() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(0)];
        let replacement_number = calculate_necessary_tiles(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::TooManyCopies {
                tile: 0,
                count: 5
            }))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("111m456p789s1122z");
        let ret = calculate_necessary_tiles_3_player(&bingpai, None);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("12z"));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1199m288p55s1177z");
        let ret = calculate_necessary_tiles_3_player(&bingpai, None);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("2p"));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let ret = calculate_necessary_tiles_3_player(&bingpai, None);
        let (replacement_number, necessary_tiles) = ret.unwrap();
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_err_bingpai_2m() {
        let bingpai = TileCounts::from_code("2m");
        let replacement_number = calculate_necessary_tiles_3_player(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::InvalidTileForThreePlayer(1)
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_err_fulu_123p() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(9, ClaimedTilePosition::Low)];
        let replacement_number =
            calculate_necessary_tiles_3_player(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::InvalidFuluMianziForThreePlayer(FuluMianzi::Shunzi(
                    9,
                    ClaimedTilePosition::Low
                ))
            )))
        ));
    }

    #[test]
    fn calculate_necessary_tiles_3_player_err_fulu_222m() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(1)];
        let replacement_number =
            calculate_necessary_tiles_3_player(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::InvalidFuluMianziForThreePlayer(FuluMianzi::Kezi(1))
            )))
        ));
    }
}
