// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use super::standard;
use crate::bingpai::{Bingpai, Bingpai3p};
use crate::config::PlayerCount;
use crate::error::XiangtingError;
use crate::tile::TileCounts;

/// Calculates the replacement number (= xiangting number + 1) for a given hand.
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
/// # use xiangting::{PlayerCount, calculate_replacement_number};
/// # use xiangting::XiangtingError;
/// # fn main() -> Result<(), XiangtingError> {
/// // 123m456p789s11222z
/// let hand: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
///
/// let replacement_number = calculate_replacement_number(&hand, &PlayerCount::Four);
/// assert_eq!(replacement_number?, 0u8);
/// # Ok(())
/// # }
/// ```
///
/// In three-player mahjong, the tiles from 2m (二萬) to 8m (八萬) are not used.
///
/// ```
/// # use xiangting::{PlayerCount, calculate_replacement_number};
/// # use xiangting::XiangtingError;
/// # fn main() -> Result<(), XiangtingError> {
/// // 1111m111122233z
/// let hand: [u8; 34] = [
///     4, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     4, 3, 2, 0, 0, 0, 0, // z
/// ];
///
/// let replacement_number_4p = calculate_replacement_number(&hand, &PlayerCount::Four);
/// assert_eq!(replacement_number_4p.unwrap(), 2u8);
///
/// let replacement_number_3p = calculate_replacement_number(&hand, &PlayerCount::Three);
/// assert_eq!(replacement_number_3p.unwrap(), 3u8);
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn calculate_replacement_number(
    bingpai: &TileCounts,
    player_count: &PlayerCount,
) -> Result<u8, XiangtingError> {
    match player_count {
        PlayerCount::Four => calculate_replacement_number_4p(bingpai),
        PlayerCount::Three => calculate_replacement_number_3p(bingpai),
    }
}

fn calculate_replacement_number_4p(tile_counts: &TileCounts) -> Result<u8, XiangtingError> {
    let bingpai = Bingpai::new(tile_counts)?;

    let r0 = standard::calculate_replacement_number(&bingpai);
    let r1 = qiduizi::calculate_replacement_number(&bingpai);
    let r2 = shisanyao::calculate_replacement_number(&bingpai);

    Ok([r0, r1, r2].into_iter().min().unwrap())
}

fn calculate_replacement_number_3p(tile_counts: &TileCounts) -> Result<u8, XiangtingError> {
    let bingpai_3p = Bingpai3p::new(tile_counts)?;

    let r0 = standard::calculate_replacement_number_3p(&bingpai_3p);

    let bingpai = bingpai_3p.into();

    let r1 = qiduizi::calculate_replacement_number(&bingpai);
    let r2 = shisanyao::calculate_replacement_number(&bingpai);

    Ok([r0, r1, r2].into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::BingpaiError;
    use crate::test_utils::FromTileCode;

    #[test]
    fn calculate_replacement_number_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("123m456p789s1122z");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1188m288p55s1177z");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_empty() {
        let bingpai = TileCounts::from_code("");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(0)))
        ));
    }

    #[test]
    fn calculate_replacement_number_ok_bingpai_1_tile() {
        let bingpai = TileCounts::from_code("1m");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_ok_bingpai_2_tiles() {
        let bingpai = TileCounts::from_code("2p3s");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_3_tiles() {
        let bingpai = TileCounts::from_code("2p3s7z");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::InvalidTileCount(3)))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_15_tiles() {
        let bingpai = TileCounts::from_code("111222333444555m");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::TooManyTiles(15)))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_5_same_tiles() {
        let bingpai = TileCounts::from_code("11111m");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Four);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(BingpaiError::TooManyCopies {
                tile: 0,
                count: 5
            }))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("111m456p789s1122z");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Three);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1199m288p55s1177z");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Three);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Three);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_err_bingpai_2m() {
        let bingpai = TileCounts::from_code("2m");
        let replacement_number = calculate_replacement_number(&bingpai, &PlayerCount::Three);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Bingpai(
                BingpaiError::InvalidTileForThreePlayer(1)
            ))
        ));
    }
}
