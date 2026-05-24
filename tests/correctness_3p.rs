// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "correctness")]
mod hand_generator_3p;

#[cfg(feature = "correctness")]
mod tests {
    use crate::hand_generator_3p::{NUM_HANDS_3P, build_table_3p, decode_3p};
    use shanten_dp::{calc_shanten, make_tile_limits};
    use std::fs::File;
    use std::io::Write;
    use std::{env, thread};
    use xiangting::{PlayerCount, calculate_replacement_number};

    fn verify_correctness_3p<const N: usize>() -> bool {
        let num_threads = env::var("NUM_THREADS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1);

        if NUM_HANDS_3P[N - 1] % num_threads != 0 {
            panic!(
                "NUM_HANDS_3P[{}] ({}) is not evenly divisible by NUM_THREADS ({}).",
                N - 1,
                NUM_HANDS_3P[N - 1],
                num_threads,
            );
        }

        let chunk_size = NUM_HANDS_3P[N - 1] / num_threads;
        let mut handles = Vec::new();

        let table = build_table_3p::<N>();

        for i in 0..num_threads {
            let begin = i * chunk_size;
            let end = begin + chunk_size;

            let handle = thread::spawn(move || {
                let tile_limits = make_tile_limits(true);

                for hash in begin..end {
                    let hand = decode_3p(hash, &table);
                    let hand_i8 = hand.map(|count| count as i8);
                    let result_shanten_dp =
                        calc_shanten(&hand_i8, &tile_limits, (N - 1) / 3, false)
                            .unwrap()
                            .unwrap()
                            + 1;
                    let result_xiangting =
                        calculate_replacement_number(&hand, &PlayerCount::Three).unwrap();

                    if result_shanten_dp as u8 != result_xiangting {
                        return Some(format!(
                            "Hand: {:?}, shanten-dp: {}, xiangting: {}\n",
                            hand, result_shanten_dp, result_xiangting,
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
            let file_name = format!("./mismatches_3p_{}.txt", N - 1);
            let mut file = File::create(&file_name).expect("Failed to create file.");
            file.write_all(mismatches.join("").as_bytes())
                .expect("Failed to write to file.");
        }

        mismatches.is_empty()
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_01() {
        assert!(verify_correctness_3p::<2>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_02() {
        assert!(verify_correctness_3p::<3>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_04() {
        assert!(verify_correctness_3p::<5>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_05() {
        assert!(verify_correctness_3p::<6>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_07() {
        assert!(verify_correctness_3p::<8>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_08() {
        assert!(verify_correctness_3p::<9>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_10() {
        assert!(verify_correctness_3p::<11>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_11() {
        assert!(verify_correctness_3p::<12>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_13() {
        assert!(verify_correctness_3p::<14>(), "There were mismatches.")
    }

    #[test]
    #[ignore]
    fn verify_correctness_3p_14() {
        assert!(verify_correctness_3p::<15>(), "There were mismatches.")
    }
}
