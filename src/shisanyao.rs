// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::shoupai::Shoupai;

pub(super) fn calculate_replacement_number(shoupai: &Shoupai) -> u8 {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return u8::MAX;
    }

    const YAOJIUPAI_INDICES: [usize; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];
    let (num_kinds, has_jiangpai) = YAOJIUPAI_INDICES
        .iter()
        .map(|&i| &shoupai.bingpai[i])
        .filter(|&&count| count > 0)
        .fold((0, false), |(num_kinds, has_jiangpai), &count| {
            (num_kinds + 1, has_jiangpai || count >= 2)
        });

    14 - num_kinds - (has_jiangpai as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::Bingpai;
    use crate::test_utils::BingpaiExtForTest;

    #[test]
    fn calculate_replacement_number_no_terminals_and_honors() {
        let bingpai = Bingpai::from_code("23455m345p45678s");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 14);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        let bingpai = Bingpai::from_code("189m12p249s12345z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_with_pair() {
        let bingpai = Bingpai::from_code("119m12p299s12345z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai = Bingpai::from_code("11m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_tenpai_13_wait() {
        let bingpai = Bingpai::from_code("19m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai = Bingpai::from_code("119m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand() {
        let bingpai = Bingpai::from_code("19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, u8::MAX);
    }
}
