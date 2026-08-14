// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "correctness")]
mod hand_generator;

#[cfg(feature = "correctness")]
mod tests {
    use crate::hand_generator::{NUM_HANDS, build_table, decode};
    use shanten_dp::{Mode, calc_shanten, make_tile_limits};
    use std::fs::File;
    use std::io::Write;
    use std::{env, thread};
    use xiangting::{PlayerCount, calculate_replacement_number};

    fn verify_correctness<const N: usize>() -> bool {
        let num_threads = env::var("NUM_THREADS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1);

        assert!(num_threads > 0, "NUM_THREADS must be greater than 0.");

        let chunk_size = NUM_HANDS[N - 1] / num_threads;
        let remainder = NUM_HANDS[N - 1] % num_threads;
        let mut handles = Vec::new();

        let table = build_table::<N>();
        let tile_limits = make_tile_limits(false);

        for i in 0..num_threads {
            let extra = u64::from(i < remainder);
            let begin = i * chunk_size + i.min(remainder);
            let end = begin + chunk_size + extra;

            let table = table;
            let tile_limits = tile_limits;

            let handle = thread::spawn(move || {
                for hash in begin..end {
                    let hand = decode(hash, &table);
                    let result_shanten_dp =
                        calc_shanten(&hand, &tile_limits, (N - 1) / 3, Mode::all(), false, false)
                            .unwrap()
                            .unwrap()
                            + 1;
                    let result_xiangting =
                        calculate_replacement_number(&hand, PlayerCount::Four).unwrap();

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
