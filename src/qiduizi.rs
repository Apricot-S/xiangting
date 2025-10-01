// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::shoupai::Shoupai;

pub(super) fn calculate_replacement_number(shoupai: &Shoupai) -> u8 {
    if shoupai.num_required_melds < 4 {
        return u8::MAX;
    }

    let (num_kinds, num_duizi) = shoupai
        .bingpai
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
    use crate::bingpai::Bingpai;
    use crate::test_utils::BingpaiExtForTest;

    #[test]
    fn calculate_replacement_number_without_pair() {
        let bingpai = Bingpai::from_code("19m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 7);
    }

    #[test]
    fn calculate_replacement_number_with_quadruple() {
        let bingpai = Bingpai::from_code("1188m288p55s1111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_with_triplet() {
        let bingpai = Bingpai::from_code("1188m2388p55s111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_with_2_triplets() {
        let bingpai = Bingpai::from_code("1188m288p555s111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai = Bingpai::from_code("1188m288p55s1177z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai = Bingpai::from_code("1188m2288p55s1177z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand() {
        let bingpai = Bingpai::from_code("1188m55s1122z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, u8::MAX);
    }
}
