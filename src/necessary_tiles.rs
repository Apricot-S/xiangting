// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use super::standard;
use crate::bingpai::{Bingpai, Bingpai3p, BingpaiError};
use crate::config::PlayerCount;
use crate::tile::{TileCounts, TileFlags};
use core::cmp::Ordering;

/// Calculates the replacement number (= xiàngtīng number + 1) and necessary tiles for a given hand.
///
/// # Arguments
///
/// * `bingpai` - 兵牌: A reference to a hand excluding melds (a.k.a. pure hand, 純手牌).
/// * `player_count` - A reference to the number of players.
///
/// # Errors
///
/// Returns [`Err`] if the hand is invalid.
///
/// # Examples
///
/// ```
/// # use xiangting::{PlayerCount, calculate_necessary_tiles};
/// # use xiangting::BingpaiError;
/// # fn main() -> Result<(), BingpaiError> {
/// // 199m146779p12s246z
/// let hand: [u8; 34] = [
///     1, 0, 0, 0, 0, 0, 0, 0, 2, // m
///     1, 0, 0, 1, 0, 1, 2, 0, 1, // p
///     1, 1, 0, 0, 0, 0, 0, 0, 0, // s
///     0, 1, 0, 1, 0, 1, 0, // z
/// ];
///
/// let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&hand, &PlayerCount::Four)?;
/// assert_eq!(replacement_number, 5u8);
/// assert_eq!(necessary_tiles, 0b1111111_100000111_111111111_100000111); // 1239m123456789p1239s1234567z
/// # Ok(())
/// # }
/// ```
///
/// In three-player mahjong, the tiles from 2m (二萬) to 8m (八萬) are not used.
///
/// ```
/// # use xiangting::{PlayerCount, calculate_necessary_tiles};
/// # use xiangting::BingpaiError;
/// # fn main() -> Result<(), BingpaiError> {
/// // 1111m111122233z
/// let hand: [u8; 34] = [
///     4, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     4, 3, 2, 0, 0, 0, 0, // z
/// ];
///
/// let (replacement_number_4p, necessary_tiles_4p) =
///     calculate_necessary_tiles(&hand, &PlayerCount::Four)?;
/// assert_eq!(replacement_number_4p, 2u8);
/// assert_eq!(necessary_tiles_4p, 0b0000000_000000000_000000000_000000110); // 23m
///
/// let (replacement_number_3p, necessary_tiles_3p) =
///     calculate_necessary_tiles(&hand, &PlayerCount::Three)?;
/// assert_eq!(replacement_number_3p, 3u8);
/// assert_eq!(necessary_tiles_3p, 0b1111100_111111111_111111111_100000000); // 9m123456789p123456789s34567z
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn calculate_necessary_tiles(
    bingpai: &TileCounts,
    player_count: &PlayerCount,
) -> Result<(u8, TileFlags), BingpaiError> {
    match player_count {
        PlayerCount::Four => calculate_necessary_tiles_4p(bingpai),
        PlayerCount::Three => calculate_necessary_tiles_3p(bingpai),
    }
}

fn calculate_necessary_tiles_4p(tile_counts: &TileCounts) -> Result<(u8, TileFlags), BingpaiError> {
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

fn calculate_necessary_tiles_3p(tile_counts: &TileCounts) -> Result<(u8, TileFlags), BingpaiError> {
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
        assert!(matches!(ret, Err(BingpaiError::InvalidTileCount(0))));
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
        assert!(matches!(ret, Err(BingpaiError::InvalidTileCount(3))));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_15_tiles() {
        let bingpai = TileCounts::from_code("111222333444555m");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(ret, Err(BingpaiError::TooManyTiles(15))));
    }

    #[test]
    fn calculate_necessary_tiles_err_bingpai_5_same_tiles() {
        let bingpai = TileCounts::from_code("11111m");
        let ret = calculate_necessary_tiles(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            ret,
            Err(BingpaiError::TooManyCopies { tile: 0, count: 5 })
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
            Err(BingpaiError::InvalidTileForThreePlayer(1))
        ));
    }
}
