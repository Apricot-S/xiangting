// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/Apricot-S/xiangting

mod calsht;

use crate::calsht::Calsht;
use std::fs::OpenOptions;
use std::io::Write;
use xiangting::calculate_replacement_number;
use xiangting::common::HandEnumerator;

fn test_correctness(length: usize) -> bool {
    let enumerator = HandEnumerator::new(length).unwrap();
    let mut all_match = true;
    let file_name = format!("mismatches_{}.txt", length);

    let mut calculator0 = Calsht::new();
    calculator0.initialize("/workspaces/shanten-number");

    enumerator.into_iter().for_each(|hand| {
        let hand_i32: Vec<i32> = hand.iter().map(|&t| t as i32).collect();
        let m = (length / 3) as i32;
        let (result_shanten_number, _) = calculator0.operator(&hand_i32, m, 0b111);
        let result_xiangting = calculate_replacement_number(&hand, &None).unwrap();

        if result_xiangting != (result_shanten_number as u8) {
            all_match = false;

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&file_name)
                .unwrap();

            writeln!(
                file,
                "Hand: {:?}, Shanten Number: {}, xiangting: {}",
                hand, result_shanten_number, result_xiangting,
            )
            .unwrap();
        }
    });

    all_match
}

#[test]
#[ignore]
fn test_correctness_1() {
    assert!(test_correctness(1));
}

#[test]
#[ignore]
fn test_correctness_2() {
    assert!(test_correctness(2));
}
