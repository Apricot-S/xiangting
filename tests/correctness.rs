// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "correctness")]
mod nyanten;

#[cfg(feature = "correctness")]
mod tests {
    use crate::nyanten::calculateReplacementNumber;
    use std::fs::OpenOptions;
    use std::io::Write;
    use xiangting::calculate_replacement_number;
    use xiangting::common::HandEnumerator;

    fn verify_correctness(length: usize) -> bool {
        let enumerator = HandEnumerator::new(length).unwrap();
        let mut all_match = true;
        let file_name = format!("./mismatches_{}.txt", length);

        enumerator.into_iter().for_each(|hand| {
            let result_nyanten =
                unsafe { calculateReplacementNumber(hand.as_ptr(), hand.as_ptr().add(33)) };
            let result_xiangting = calculate_replacement_number(&hand, &None).unwrap();

            if result_xiangting != result_nyanten {
                all_match = false;

                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&file_name)
                    .unwrap();

                writeln!(
                    file,
                    "Hand: {:?}, Nyanten: {}, xiangting: {}",
                    hand, result_nyanten, result_xiangting,
                )
                .unwrap();
            }
        });

        all_match
    }

    #[test]
    #[ignore]
    fn verify_correctness_1() {
        assert!(verify_correctness(1));
    }

    #[test]
    #[ignore]
    fn verify_correctness_2() {
        assert!(verify_correctness(2));
    }
}
