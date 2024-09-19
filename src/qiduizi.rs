// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::bingpai::Bingpai;
use super::constants::MAX_NUM_SHOUPAI;

pub(crate) fn calculate_replacement_number(bingpai: &Bingpai, num_bingpai: u8) -> u8 {
    if (num_bingpai != (MAX_NUM_SHOUPAI - 1)) && (num_bingpai != MAX_NUM_SHOUPAI) {
        return u8::MAX;
    }

    let mut num_kinds: u8 = 0;
    let mut num_duizi: u8 = 0;

    bingpai.iter().for_each(|&num_tile| {
        if num_tile >= 1 {
            num_kinds += 1;
        }
        if num_tile >= 2 {
            num_duizi += 1;
        }
    });

    7 - num_duizi + (if num_kinds < 7 { 7 - num_kinds } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_replacement_number_without_pair() {
        let bingpai: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 7);
    }

    #[test]
    fn calculate_replacement_number_with_quadruple() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 1, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            4, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_with_triplet() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 1, 1, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            3, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_with_2_triplets() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 1, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 3, 0, 0, 0, 0, // s
            3, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 1, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 2, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 2, 0, 0, 0, 0, 0, 2, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 2, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand() {
        let bingpai: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 2, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 2, 0, 0, 0, 0, // s
            2, 2, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, u8::MAX);
    }
}
