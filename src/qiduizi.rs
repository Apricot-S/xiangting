// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::Bingpai;
use crate::constants::MAX_NUM_SHOUPAI;

pub(super) fn calculate_replacement_number(bingpai: &Bingpai, num_bingpai: u8) -> u8 {
    if (num_bingpai != (MAX_NUM_SHOUPAI - 1)) && (num_bingpai != MAX_NUM_SHOUPAI) {
        return u8::MAX;
    }

    let (num_kinds, num_duizi) = bingpai
        .iter()
        .filter(|&&count| count > 0)
        .fold((0, 0), |(num_kinds, num_duizi), &count| {
            (num_kinds + 1, num_duizi + (count >= 2) as u8)
        });

    7 - num_duizi + 7u8.saturating_sub(num_kinds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::BingpaiExtForTest;

    #[test]
    fn calculate_replacement_number_without_pair() {
        let bingpai = Bingpai::from_code("19m19p19s1234567z");
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 7);
    }

    #[test]
    fn calculate_replacement_number_with_quadruple() {
        let bingpai = Bingpai::from_code("1188m288p55s1111z");
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_with_triplet() {
        let bingpai = Bingpai::from_code("1188m2388p55s111z");
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_with_2_triplets() {
        let bingpai = Bingpai::from_code("1188m288p555s111z");
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai = Bingpai::from_code("1188m288p55s1177z");
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai = Bingpai::from_code("1188m2288p55s1177z");
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand() {
        let bingpai = Bingpai::from_code("1188m55s1122z");
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(&bingpai, num_bingpai);
        assert_eq!(replacement_number, u8::MAX);
    }
}
