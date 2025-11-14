// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use super::standard;
use crate::error::XiangtingError;
use crate::fulu_mianzi::FuluMianzi;
use crate::shoupai::{Shoupai, Shoupai3p};
use crate::tile::TileCounts;

/// Calculates the replacement number (= xiangting number + 1) for a given hand.
/// This function is for 4-player mahjong.
///
/// In some rulesets, melded tiles are excluded when checking whether a hand contains
/// four identical tiles. In others, melded tiles are included in the calculation.
/// This function allows you to control that behavior via the `fulu_mianzi_list` argument:
///
/// - Use `None` if melds are excluded in the ruleset (e.g., Tenhou, Mahjong Soul).
/// - Provide `Some(&[..])` if melds are included (e.g., World Riichi Championship, M.LEAGUE).
///
/// If fewer melds are provided than required for a complete hand,
/// the missing ones are treated as melds that do not overlap with the tiles in the hand.
/// For example, with the hand 123p1s, three melds are required.
/// If only two are given (e.g., \[444p, 777s]), the third is considered to be
/// a non-overlapping meld, such as 111z.
///
/// # Arguments
///
/// * `bingpai` - 兵牌: A reference to a hand excluding melds (a.k.a. pure hand, 純手牌).
/// * `fulu_mianzi_list` - An `Option` containing a reference to a slice of melds.
///
/// # Errors
///
/// Returns [`Err`] if the hand is invalid.
///
/// # Examples
///
/// ```
/// # use xiangting::{ClaimedTilePosition, FuluMianzi, calculate_replacement_number};
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
/// let replacement_number = calculate_replacement_number(&hand, None);
/// assert_eq!(replacement_number?, 0u8);
///
/// // 123m1z
/// let hand: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 456p 7777s 111z
/// let melds_3 = [
///     FuluMianzi::Shunzi(12, ClaimedTilePosition::Low),
///     FuluMianzi::Gangzi(24),
///     FuluMianzi::Kezi(27),
/// ];
///
/// let replacement_number_wo_melds = calculate_replacement_number(&hand, None);
/// assert_eq!(replacement_number_wo_melds?, 1u8);
///
/// let replacement_number_w_melds = calculate_replacement_number(&hand, Some(&melds_3));
/// assert_eq!(replacement_number_w_melds?, 2u8);
///
/// // 456p 7777s
/// let melds_2 = [
///     FuluMianzi::Shunzi(12, ClaimedTilePosition::Low),
///     FuluMianzi::Gangzi(24),
/// ];
///
/// let replacement_number_w_missing_melds = calculate_replacement_number(&hand, Some(&melds_2));
/// assert_eq!(replacement_number_w_missing_melds?, 1u8);
/// # Ok(())
/// # }
/// ```
pub fn calculate_replacement_number(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let shoupai = Shoupai::new(bingpai, fulu_mianzi_list)?;

    let r0 = standard::calculate_replacement_number(&shoupai);
    let r1 = qiduizi::calculate_replacement_number(&shoupai);
    let r2 = shisanyao::calculate_replacement_number(&shoupai);

    Ok([r0, r1, r2].into_iter().min().unwrap())
}

/// Calculates the replacement number (= xiangting number + 1) for a given hand.
/// This function is for 3-player mahjong.
///
/// Tiles from 2m (二萬) to 8m (八萬) are not used.
/// In addition, melded sequences (明順子) are not allowed.
///
/// In some rulesets, melded tiles are excluded when checking whether a hand contains
/// four identical tiles. In others, melded tiles are included in the calculation.
/// This function allows you to control that behavior via the `fulu_mianzi_list` argument:
///
/// - Use `None` if melds are excluded in the ruleset (e.g., Tenhou, Mahjong Soul).
/// - Provide `Some(&[..])` if melds are included (e.g., World Riichi Championship, M.LEAGUE).
///
/// If fewer melds are provided than required for a complete hand,
/// the missing ones are treated as melds that do not overlap with the tiles in the hand.
/// For example, with the hand 123p1s, three melds are required.
/// If only two are given (e.g., \[444p, 777s]), the third is considered to be
/// a non-overlapping meld, such as 111z.
///
/// # Arguments
///
/// * `bingpai` - 兵牌: A reference to a hand excluding melds (a.k.a. pure hand, 純手牌).
/// * `fulu_mianzi_list` - An `Option` containing a reference to a slice of melds.
///
/// # Errors
///
/// Returns [`Err`] if the hand is invalid.
///
/// # Examples
///
/// ```
/// # use xiangting::{ClaimedTilePosition, FuluMianzi, calculate_replacement_number_3_player};
/// # use xiangting::XiangtingError;
/// # fn main() -> Result<(), XiangtingError> {
/// // 111m456p789s11222z
/// let hand: [u8; 34] = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
///
/// let replacement_number = calculate_replacement_number_3_player(&hand, None);
/// assert_eq!(replacement_number?, 0u8);
///
/// // 111m1z
/// let hand: [u8; 34] = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 444p 7777s 111z
/// let melds_3 = [
///     FuluMianzi::Kezi(12),
///     FuluMianzi::Gangzi(24),
///     FuluMianzi::Kezi(27),
/// ];
///
/// let replacement_number_wo_melds = calculate_replacement_number_3_player(&hand, None);
/// assert_eq!(replacement_number_wo_melds?, 1u8);
///
/// let replacement_number_w_melds = calculate_replacement_number_3_player(&hand, Some(&melds_3));
/// assert_eq!(replacement_number_w_melds?, 2u8);
///
/// // 444p 7777s
/// let melds_2 = [FuluMianzi::Kezi(12), FuluMianzi::Gangzi(24)];
///
/// let replacement_number_w_missing_melds =
///     calculate_replacement_number_3_player(&hand, Some(&melds_2));
/// assert_eq!(replacement_number_w_missing_melds?, 1u8);
/// # Ok(())
/// # }
/// ```
pub fn calculate_replacement_number_3_player(
    bingpai: &TileCounts,
    fulu_mianzi_list: Option<&[FuluMianzi]>,
) -> Result<u8, XiangtingError> {
    let shoupai_3p = Shoupai3p::new(bingpai, fulu_mianzi_list)?;

    let r0 = standard::calculate_replacement_number_3_player(&shoupai_3p);

    let shoupai = shoupai_3p.into();

    let r1 = qiduizi::calculate_replacement_number(&shoupai);
    let r2 = shisanyao::calculate_replacement_number(&shoupai);

    Ok([r0, r1, r2].into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::BingpaiError;
    use crate::fulu_mianzi::{ClaimedTilePosition, FuluMianziError};
    use crate::shoupai::ShoupaiError;
    use crate::test_utils::FromTileCode;

    #[test]
    fn calculate_replacement_number_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("123m456p789s1122z");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1188m288p55s1177z");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_empty() {
        let bingpai = TileCounts::from_code("");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::InvalidTileCount(0)
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_ok_bingpai_1_tile() {
        let bingpai = TileCounts::from_code("1m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_ok_bingpai_2_tiles() {
        let bingpai = TileCounts::from_code("2p3s");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(replacement_number.is_ok());
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_3_tiles() {
        let bingpai = TileCounts::from_code("2p3s7z");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::InvalidTileCount(3)
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_15_tiles() {
        let bingpai = TileCounts::from_code("111222333444555m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::TooManyTiles(15)
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_bingpai_5_same_tiles() {
        let bingpai = TileCounts::from_code("11111m");
        let replacement_number = calculate_replacement_number(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::TooManyCopies { tile: 0, count: 5 }
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_fulu_index_out_of_range() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(34)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::IndexOutOfRange(34)
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_fulu_shunzi_with_zipai() {
        let bingpai = TileCounts::from_code("1p");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(27, ClaimedTilePosition::Low)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::ShunziWithZipai(27)
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_fulu_invalid_shunzi_combination() {
        let bingpai = TileCounts::from_code("1p");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(0, ClaimedTilePosition::Middle)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::InvalidShunziCombination(0, ClaimedTilePosition::Middle)
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_shoupai_too_many_fulu_mianzi() {
        let bingpai = TileCounts::from_code("11122233344455m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(5)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::TooManyFuluMianzi {
                max: 0,
                count: 1
            }))
        ));
    }

    #[test]
    fn calculate_replacement_number_err_shoupai_5_same_tiles() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(0)];
        let replacement_number = calculate_replacement_number(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::TooManyCopies {
                tile: 0,
                count: 5
            }))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_ok_standard_tenpai() {
        let bingpai = TileCounts::from_code("111m456p789s1122z");
        let replacement_number = calculate_replacement_number_3_player(&bingpai, None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_ok_qiduizi_tenpai() {
        let bingpai = TileCounts::from_code("1199m288p55s1177z");
        let replacement_number = calculate_replacement_number_3_player(&bingpai, None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_ok_shisanyao_tenpai() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let replacement_number = calculate_replacement_number_3_player(&bingpai, None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_err_bingpai_2m() {
        let bingpai = TileCounts::from_code("2m");
        let replacement_number = calculate_replacement_number_3_player(&bingpai, None);
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::Bingpai(
                BingpaiError::InvalidTileFor3Player(1)
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_err_fulu_123p() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Shunzi(9, ClaimedTilePosition::Low)];
        let replacement_number =
            calculate_replacement_number_3_player(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Shunzi(
                    9,
                    ClaimedTilePosition::Low
                ))
            )))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_err_fulu_222m() {
        let bingpai = TileCounts::from_code("1m");
        let fulu_mianzi_list = [FuluMianzi::Kezi(1)];
        let replacement_number =
            calculate_replacement_number_3_player(&bingpai, Some(&fulu_mianzi_list));
        assert!(matches!(
            replacement_number,
            Err(XiangtingError::Shoupai(ShoupaiError::FuluMianzi(
                FuluMianziError::InvalidFuluMianziFor3Player(FuluMianzi::Kezi(1))
            )))
        ));
    }
}
