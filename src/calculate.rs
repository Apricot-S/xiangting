// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::bingpai::{count_bingpai, count_bingpai_3_player, Bingpai, InvalidBingpaiError};
use super::qiduizi;
use super::shisanyao;
use super::shoupai::{
    validate_shoupai, validate_shoupai_3_player, FuluMianziList, InvalidShoupaiError,
};
use super::standard;
use thiserror::Error;

/// Error type of replacement number calculation
///
/// Indicates that the hand or melds are invalid.
#[derive(Debug, Error)]
pub enum XiangtingError {
    #[error("InvalidBingpaiError({0})")]
    InvalidBingpai(#[from] InvalidBingpaiError),
    #[error("InvalidShoupaiError({0})")]
    InvalidShoupai(#[from] InvalidShoupaiError),
}

/// Calculates the replacement number (= xiangting number + 1) for a given hand.
/// This function is for 4-player mahjong.
///
/// # Arguments
///
/// * `bingpai` - A reference to a hand excluding melds.
/// * `fulu_mianzi_list` - An optional reference to a list of melds.
///
/// # Returns
///
/// A [`Result`] containing the replacement number as [`u8`] or a [`XiangtingError`].
///
/// # Examples
///
/// ```
/// # use xiangting::{calculate_replacement_number, ClaimedTilePosition, FuluMianzi};
/// # use xiangting::XiangtingError;
/// # fn main() -> Result<(), XiangtingError> {
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
/// // 123m1z (3 melds)
/// let hand_4: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 456p 7777s 111z
/// let melds = [
///     Some(FuluMianzi::Shunzi(12, ClaimedTilePosition::Low)),
///     Some(FuluMianzi::Gangzi(24)),
///     Some(FuluMianzi::Kezi(27)),
///     None,
/// ];
///
/// let replacement_number_wo_melds = calculate_replacement_number(&hand_4, &None);
/// assert_eq!(replacement_number_wo_melds?, 1u8);
///
/// let replacement_number_w_melds = calculate_replacement_number(&hand_4, &Some(melds));
/// assert_eq!(replacement_number_w_melds?, 2u8);
/// # Ok(())
/// # }
/// ```
pub fn calculate_replacement_number(
    bingpai: &Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
) -> Result<u8, XiangtingError> {
    let num_bingpai = count_bingpai(bingpai)?;

    if let Some(f) = fulu_mianzi_list {
        validate_shoupai(bingpai, f)?;
    }

    let r0 = standard::calculate_replacement_number(*bingpai, fulu_mianzi_list, num_bingpai);
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
/// # Arguments
///
/// * `bingpai` - A reference to a hand excluding melds.
/// * `fulu_mianzi_list` - An optional reference to a list of melds.
///
/// # Returns
///
/// A [`Result`] containing the replacement number as [`u8`] or a [`XiangtingError`].
///
/// # Examples
///
/// ```
/// # use xiangting::{calculate_replacement_number_3_player, ClaimedTilePosition, FuluMianzi};
/// # use xiangting::XiangtingError;
/// # fn main() -> Result<(), XiangtingError> {
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
/// // 111m1z (3 melds)
/// let hand_4: [u8; 34] = [
///     3, 0, 0, 0, 0, 0, 0, 0, 0, // m
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // p
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // s
///     1, 0, 0, 0, 0, 0, 0, // z
/// ];
///
/// // 444p 7777s 111z
/// let melds = [
///     Some(FuluMianzi::Kezi(12)),
///     Some(FuluMianzi::Gangzi(24)),
///     Some(FuluMianzi::Kezi(27)),
///     None,
/// ];
///
/// let replacement_number_wo_melds = calculate_replacement_number_3_player(&hand_4, &None);
/// assert_eq!(replacement_number_wo_melds?, 1u8);
///
/// let replacement_number_w_melds = calculate_replacement_number_3_player(&hand_4, &Some(melds));
/// assert_eq!(replacement_number_w_melds?, 2u8);
/// # Ok(())
/// # }
/// ```
pub fn calculate_replacement_number_3_player(
    bingpai: &Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
) -> Result<u8, XiangtingError> {
    let num_bingpai = count_bingpai_3_player(bingpai)?;

    if let Some(f) = fulu_mianzi_list {
        validate_shoupai_3_player(bingpai, f)?;
    }

    let r0 =
        standard::calculate_replacement_number_3_player(*bingpai, fulu_mianzi_list, num_bingpai);
    let r1 = qiduizi::calculate_replacement_number(bingpai, num_bingpai);
    let r2 = shisanyao::calculate_replacement_number(bingpai, num_bingpai);
    Ok([r0, r1, r2].into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
