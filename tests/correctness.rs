// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "correctness")]
mod hand_generator;
#[cfg(feature = "correctness")]
mod nyanten;

#[cfg(feature = "correctness")]
mod tests {
    use crate::hand_generator::{NUM_HANDS, build_table, decode};
    use crate::nyanten::calculateReplacementNumber;
    use std::fs::File;
    use std::io::Write;
    use std::{env, thread};
    use xiangting::{PlayerCount, calculate_replacement_number};

    fn verify_correctness<const N: usize>() -> bool {
        let num_threads = env::var("NUM_THREADS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1);

        if NUM_HANDS[N - 1] % num_threads != 0 {
            panic!(
                "NUM_HANDS[{}] ({}) is not evenly divisible by NUM_THREADS ({}).",
                N - 1,
                NUM_HANDS[N - 1],
                num_threads,
            );
        }

        let chunk_size = NUM_HANDS[N - 1] / num_threads;
        let mut handles = Vec::new();

        let table = build_table::<N>();

        for i in 0..num_threads {
            let begin = i * chunk_size;
            let end = begin + chunk_size;

            let handle = thread::spawn(move || {
                for hash in begin..end {
                    let hand = decode(hash, &table);
                    let result_nyanten =
                        unsafe { calculateReplacementNumber(hand.as_ptr(), hand.as_ptr().add(34)) };
                    let result_xiangting =
                        calculate_replacement_number(&hand, &PlayerCount::Four).unwrap();

                    if result_nyanten != result_xiangting {
                        return Some(format!(
                            "Hand: {:?}, Nyanten: {}, xiangting: {}\n",
                            hand, result_nyanten, result_xiangting,
                        ));
                    }
                }

                None
            });

            handles.push(handle);
        }

        let results: Vec<_> = handles.into_iter().map(|handle| handle.join()).collect();
        if results.iter().any(|result| result.is_err()) {
            panic!("Test failed due to a thread panic.");
        }

        let mismatches: Vec<_> = results
            .into_iter()
            .filter_map(|result| result.ok())
            .flatten()
            .collect();

        if !mismatches.is_empty() {
            let file_name = format!("./mismatches_{}.txt", N - 1);
            let mut file = File::create(&file_name).expect("Failed to create file.");
            file.write_all(mismatches.join("").as_bytes())
                .expect("Failed to write to file.");
        }

        mismatches.is_empty()
    }

    #[test]
    #[ignore]
    fn verify_correctness_01() {
        assert!(verify_correctness::<2>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_02() {
        assert!(verify_correctness::<3>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_04() {
        assert!(verify_correctness::<5>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_05() {
        assert!(verify_correctness::<6>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_07() {
        assert!(verify_correctness::<8>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_08() {
        assert!(verify_correctness::<9>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_10() {
        assert!(verify_correctness::<11>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_11() {
        assert!(verify_correctness::<12>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_13() {
        assert!(verify_correctness::<14>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_14() {
        assert!(verify_correctness::<15>(), "There were mismatches.")
    }
}
