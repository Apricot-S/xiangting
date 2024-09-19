// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

const MAX_NUM_SAME_TILE: u8 = 4;
const NUM_TILE_INDEX: usize = 3 * 9 + 4 + 3;
const MAX_NUM_HAND: usize = 14;

#[inline]
fn to_34_array(hand: &[usize]) -> [u8; NUM_TILE_INDEX] {
    let mut hand34 = [0u8; NUM_TILE_INDEX];
    hand.iter().for_each(|&t| hand34[t] += 1);
    hand34
}

pub struct HandEnumerator {
    tiles: [u8; NUM_TILE_INDEX],
    current_hand: Vec<usize>,
    stack: Vec<usize>,
    length: usize,
}

impl HandEnumerator {
    pub fn new(length: usize) -> Result<Self, String> {
        if length < 1 || length > MAX_NUM_HAND {
            return Err(format!("Hand length must be between 1 and 14.: {}", length));
        }

        Ok(Self {
            tiles: [MAX_NUM_SAME_TILE; NUM_TILE_INDEX],
            current_hand: Vec::new(),
            stack: vec![0],
            length,
        })
    }
}

impl Iterator for HandEnumerator {
    type Item = [u8; NUM_TILE_INDEX];

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(i) = self.stack.pop() {
            if self.current_hand.len() == self.length {
                let result = self.current_hand.clone();
                // Backtrack
                if let Some(last_tile) = self.current_hand.pop() {
                    self.tiles[last_tile] += 1;
                }
                return Some(to_34_array(&result));
            }

            if i >= NUM_TILE_INDEX {
                // End the loop at this level and backtrack
                if let Some(last_tile) = self.current_hand.pop() {
                    self.tiles[last_tile] += 1;
                }
                continue;
            }

            if self.tiles[i] > 0 {
                // Select tile
                self.tiles[i] -= 1;
                self.current_hand.push(i);

                // Update the index of the current frame
                self.stack.push(i + 1);
                // Push a new frame to the stack (simulate recursive call)
                self.stack.push(i);
            } else {
                // Move to the next tile
                self.stack.push(i + 1);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn valid_length() {
        (1..=MAX_NUM_HAND).into_iter().for_each(|i| {
            let result = HandEnumerator::new(i);
            assert!(result.is_ok())
        });
    }

    #[test]
    fn invalid_length() {
        let result0 = HandEnumerator::new(0);
        assert!(result0.is_err());
        let result15 = HandEnumerator::new(MAX_NUM_HAND + 1);
        assert!(result15.is_err());
    }

    fn correct_hands(
        length: usize,
        expected_first: [u8; NUM_TILE_INDEX],
        expected_last: [u8; NUM_TILE_INDEX],
        expected_count: usize,
    ) {
        let mut generator = HandEnumerator::new(length).unwrap();

        let start = Instant::now();

        let first_hand = generator.next().unwrap();
        let (last_hand, count) = generator.fold((first_hand.clone(), 1), |(_, count), hand| {
            (hand, count + 1)
        });

        let elapsed_time = start.elapsed();
        println!(
            "Time elapsed in generating hands (length = {}) is: {:?}",
            length, elapsed_time,
        );

        assert_eq!(first_hand, expected_first);
        assert_eq!(last_hand, expected_last);
        assert_eq!(count, expected_count);
    }

    #[test]
    #[ignore]
    fn correct_hands_1() {
        correct_hands(1, to_34_array(&[0]), to_34_array(&[33]), 34);
    }

    #[test]
    #[ignore]
    fn correct_hands_2() {
        correct_hands(2, to_34_array(&[0, 0]), to_34_array(&[33, 33]), 595);
    }

    #[test]
    #[ignore]
    fn correct_hands_3() {
        correct_hands(
            3,
            to_34_array(&[0, 0, 0]),
            to_34_array(&[33, 33, 33]),
            7_140,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_4() {
        correct_hands(
            4,
            to_34_array(&[0, 0, 0, 0]),
            to_34_array(&[33, 33, 33, 33]),
            66_045,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_5() {
        correct_hands(
            5,
            to_34_array(&[0, 0, 0, 0, 1]),
            to_34_array(&[32, 33, 33, 33, 33]),
            501_908,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_6() {
        correct_hands(
            6,
            to_34_array(&[0, 0, 0, 0, 1, 1]),
            to_34_array(&[32, 32, 33, 33, 33, 33]),
            3_261_467,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_7() {
        correct_hands(
            7,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1]),
            to_34_array(&[32, 32, 32, 33, 33, 33, 33]),
            18_623_330,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_8() {
        correct_hands(
            8,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1, 1]),
            to_34_array(&[32, 32, 32, 32, 33, 33, 33, 33]),
            95_305_485,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_9() {
        correct_hands(
            9,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1, 1, 2]),
            to_34_array(&[31, 32, 32, 32, 32, 33, 33, 33, 33]),
            443_646_280,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_10() {
        correct_hands(
            10,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2]),
            to_34_array(&[31, 31, 32, 32, 32, 32, 33, 33, 33, 33]),
            1_900_269_316,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_11() {
        correct_hands(
            11,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2]),
            to_34_array(&[31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]),
            7_558_429_024,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_12() {
        correct_hands(
            12,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2]),
            to_34_array(&[31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]),
            28_126_474_500,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_13() {
        correct_hands(
            13,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3]),
            to_34_array(&[30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]),
            98_521_596_000,
        );
    }

    #[test]
    #[ignore]
    fn correct_hands_14() {
        correct_hands(
            14,
            to_34_array(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3]),
            to_34_array(&[30, 30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]),
            326_520_504_500,
        );
    }
}
