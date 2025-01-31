// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::qiduizi;
use super::shisanyao;
use super::standard;
use crate::bingpai::{Bingpai, BingpaiExt};
use crate::shoupai::{get_shoupai, get_shoupai_3_player, FuluMianziList, InvalidShoupaiError};

/// Calculates the replacement number (= xiangting number + 1) for a given hand.
/// This function is for 4-player mahjong.
///
/// If the number of melds in the list is less than the required number of melds for the hand,
/// the missing melds are calculated as melds that do not overlap with the tiles in the hand.
/// For example, if the hand consists of 123p1s, three melds are required.
/// If only two melds are provided, such as \[444p, 777s], the missing third meld is calculated as
/// a meld that does not overlap with the tiles in the hand, such as 111z.
///
/// # Arguments
///
/// * `bingpai` - A reference to a hand excluding melds.
/// * `fulu_mianzi_list` - An optional reference to a list of melds.
///
/// # Errors
///
/// Returns [`Err`] if the hand is invalid.
///
/// # Examples
///
/// ```
/// # use xiangting::{calculate_replacement_number, ClaimedTilePosition, FuluMianzi};
/// # use xiangting::InvalidShoupaiError;
/// # fn main() -> Result<(), InvalidShoupaiError> {
/// // 123m456p789s11222z
/// let hand_14: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
///
/// let replacement_number = calculate_replacement_number(&hand_14, &None);
/// assert_eq!(replacement_number?, 0u8);
///
/// // 123m1z (3 melds required)
/// let hand_4: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 456p 7777s 111z (3 melds)
/// let melds_3 = [
///     Some(FuluMianzi::Shunzi(12, ClaimedTilePosition::Low)),
///     Some(FuluMianzi::Gangzi(24)),
///     Some(FuluMianzi::Kezi(27)),
///     None,
/// ];
///
/// let replacement_number_wo_melds = calculate_replacement_number(&hand_4, &None);
/// assert_eq!(replacement_number_wo_melds?, 1u8);
///
/// let replacement_number_w_melds = calculate_replacement_number(&hand_4, &Some(melds_3));
/// assert_eq!(replacement_number_w_melds?, 2u8);
///
/// // 456p 7777s (2 melds)
/// let melds_2 = [
///     Some(FuluMianzi::Shunzi(12, ClaimedTilePosition::Low)),
///     Some(FuluMianzi::Gangzi(24)),
///     None,
///     None,
/// ];
///
/// let replacement_number_w_missing_melds = calculate_replacement_number(&hand_4, &Some(melds_2));
/// assert_eq!(replacement_number_w_missing_melds?, 1u8);
/// # Ok(())
/// # }
/// ```
pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
) -> Result<u8, InvalidShoupaiError> {
    let num_bingpai = bingpai.count()?;

    debug_assert!(
        (4 - num_bingpai / 3)
            >= fulu_mianzi_list
                .as_ref()
                .map_or(0, |f| f.iter().flatten().count() as u8)
    );

    let shoupai = match fulu_mianzi_list {
        Some(f) => Some(get_shoupai(bingpai, f)?),
        None => None,
    };

    let r0 = standard::calculate_replacement_number(bingpai, &shoupai, num_bingpai);
    let r1 = qiduizi::calculate_replacement_number(bingpai, num_bingpai);
    let r2 = shisanyao::calculate_replacement_number(bingpai, num_bingpai);
    Ok([r0, r1, r2].into_iter().min().unwrap())
}

/// Calculates the replacement number (= xiangting number + 1) for a given hand.
/// This function is for 3-player mahjong.
///
/// Tiles from 2m (二萬) to 8m (八萬) cannot be used.
/// Additionally, melded sequences (明順子) cannot be used.
///
/// If the number of melds in the list is less than the required number of melds for the hand,
/// the missing melds are calculated as melds that do not overlap with the tiles in the hand.
/// For example, if the hand consists of 123p1s, three melds are required.
/// If only two melds are provided, such as \[444p, 777s], the missing third meld is calculated as
/// a meld that does not overlap with the tiles in the hand, such as 111z.
///
/// # Arguments
///
/// * `bingpai` - A reference to a hand excluding melds.
/// * `fulu_mianzi_list` - An optional reference to a list of melds.
///
/// # Errors
///
/// Returns [`Err`] if the hand is invalid.
///
/// # Examples
///
/// ```
/// # use xiangting::{calculate_replacement_number_3_player, ClaimedTilePosition, FuluMianzi};
/// # use xiangting::InvalidShoupaiError;
/// # fn main() -> Result<(), InvalidShoupaiError> {
/// // 111m456p789s11222z
/// let hand_14: [u8; 34] = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 1, 1, 1, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 1, 1, 1, // s
///     2, 3, 0, 0, 0, 0, 0, // z
/// ];
///
/// let replacement_number = calculate_replacement_number_3_player(&hand_14, &None);
/// assert_eq!(replacement_number?, 0u8);
///
/// // 111m1z (3 melds required)
/// let hand_4: [u8; 34] = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 444p 7777s 111z (3 melds)
/// let melds_3 = [
///     Some(FuluMianzi::Kezi(12)),
///     Some(FuluMianzi::Gangzi(24)),
///     Some(FuluMianzi::Kezi(27)),
///     None,
/// ];
///
/// let replacement_number_wo_melds = calculate_replacement_number_3_player(&hand_4, &None);
/// assert_eq!(replacement_number_wo_melds?, 1u8);
///
/// let replacement_number_w_melds = calculate_replacement_number_3_player(&hand_4, &Some(melds_3));
/// assert_eq!(replacement_number_w_melds?, 2u8);
///
/// // 444p 7777s (2 melds)
/// let melds_2 = [
///     Some(FuluMianzi::Kezi(12)),
///     Some(FuluMianzi::Gangzi(24)),
///     None,
///     None,
/// ];
///
/// let replacement_number_w_missing_melds
///     = calculate_replacement_number_3_player(&hand_4, &Some(melds_2));
/// assert_eq!(replacement_number_w_missing_melds?, 1u8);
/// # Ok(())
/// # }
/// ```
pub fn calculate_replacement_number_3_player(
    bingpai: &Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
) -> Result<u8, InvalidShoupaiError> {
    let num_bingpai = bingpai.count_3_player()?;

    debug_assert!(
        (4 - num_bingpai / 3)
            >= fulu_mianzi_list
                .as_ref()
                .map_or(0, |f| f.iter().flatten().count() as u8)
    );

    let shoupai = match fulu_mianzi_list {
        Some(f) => Some(get_shoupai_3_player(bingpai, f)?),
        None => None,
    };

    let r0 = standard::calculate_replacement_number_3_player(bingpai, &shoupai, num_bingpai);
    let r1 = qiduizi::calculate_replacement_number(bingpai, num_bingpai);
    let r2 = shisanyao::calculate_replacement_number(bingpai, num_bingpai);
    Ok([r0, r1, r2].into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::InvalidBingpaiError;

    #[test]
    fn calculate_replacement_number_standard_tenpai() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 2, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_shisanyao_tenpai() {
        let bingpai: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_qiduizi_tenpai() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 1, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 2, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_empty_bingpai() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert!(matches!(
            replacement_number.unwrap_err(),
            InvalidShoupaiError::InvalidBingpai(InvalidBingpaiError::EmptyBingpai)
        ));
    }

    #[test]
    fn calculate_replacement_number_too_many_tiles() {
        let bingpai: Bingpai = [
            1, 1, 1, 1, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert!(matches!(
            replacement_number.unwrap_err(),
            InvalidShoupaiError::InvalidBingpai(InvalidBingpaiError::ExceedsMaxNumBingpai(15))
        ));
    }

    #[test]
    fn calculate_replacement_number_5th_tile() {
        let bingpai: Bingpai = [
            5, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 1, 0, 0, 0, 0, 0, // p
            1, 1, 1, 1, 0, 0, 0, 0, 0, // s
            1, 0, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert!(matches!(
            replacement_number.unwrap_err(),
            InvalidShoupaiError::InvalidBingpai(InvalidBingpaiError::ExceedsMaxNumSameTile(5))
        ));
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand() {
        let bingpai: Bingpai = [
            4, 4, 4, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number(&bingpai, &None);
        assert!(matches!(
            replacement_number.unwrap_err(),
            InvalidShoupaiError::InvalidBingpai(InvalidBingpaiError::InvalidNumBingpai(12))
        ));
    }

    #[test]
    fn calculate_replacement_number_3_player_standard_tenpai() {
        let bingpai: Bingpai = [
            3, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 2, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number_3_player(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_shisanyao_tenpai() {
        let bingpai: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let replacement_number = calculate_replacement_number_3_player(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_qiduizi_tenpai() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 0, 2, // m
            0, 1, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 2, // z
        ];
        let replacement_number = calculate_replacement_number_3_player(&bingpai, &None);
        assert_eq!(replacement_number.unwrap(), 1);
    }

    #[test]
    fn calculate_replacement_number_3_player_empty_bingpai() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let replacement_number = calculate_replacement_number_3_player(&bingpai, &None);
        assert!(matches!(
            replacement_number.unwrap_err(),
            InvalidShoupaiError::InvalidBingpai(InvalidBingpaiError::EmptyBingpai)
        ));
    }
}
