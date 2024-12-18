// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "correctness")]
mod hand_enumerator;
#[cfg(feature = "correctness")]
mod nyanten;

#[cfg(feature = "correctness")]
mod tests {
    use crate::hand_enumerator::HandEnumerator;
    use crate::nyanten::calculateReplacementNumber;
    use std::fs::OpenOptions;
    use std::io::Write;
    use xiangting::calculate_replacement_number;

    fn verify_correctness(length: usize) -> bool {
        let enumerator = HandEnumerator::new(length).unwrap();
        let file_name = format!("./mismatch_{}.txt", length);

        let result = enumerator.into_iter().try_for_each(|hand| {
            let result_nyanten =
                unsafe { calculateReplacementNumber(hand.as_ptr(), hand.as_ptr().add(34)) };
            let result_xiangting = calculate_replacement_number(&hand, &None).unwrap();

            if result_xiangting != result_nyanten {
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

                return Err(());
            }

            Ok(())
        });

        result.is_ok()
    }

    #[test]
    #[ignore]
    fn verify_correctness_01() {
        assert!(verify_correctness(1));
    }

    #[test]
    #[ignore]
    fn verify_correctness_02() {
        assert!(verify_correctness(2));
    }

    #[test]
    #[ignore]
    fn verify_correctness_04() {
        assert!(verify_correctness(4));
    }

    #[test]
    #[ignore]
    fn verify_correctness_05() {
        assert!(verify_correctness(5));
    }

    #[test]
    #[ignore]
    fn verify_correctness_07() {
        assert!(verify_correctness(7));
    }

    #[test]
    #[ignore]
    fn verify_correctness_08() {
        assert!(verify_correctness(8));
    }

    #[test]
    #[ignore]
    fn verify_correctness_10() {
        assert!(verify_correctness(10));
    }

    #[test]
    #[ignore]
    fn verify_correctness_11() {
        assert!(verify_correctness(11));
    }

    #[test]
    #[ignore]
    fn verify_correctness_13() {
        assert!(verify_correctness(13));
    }

    #[test]
    #[ignore]
    fn verify_correctness_14() {
        assert!(verify_correctness(14));
    }
}
