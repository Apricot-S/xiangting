// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use xiangting::{TileCounts, calculate_replacement_number};

pub fn calculate_necessary_tiles(bingpai: &TileCounts) -> u64 {
    let mut bingpai = bingpai.clone();

    let replacement_number = calculate_replacement_number(&bingpai, None).unwrap();
    if replacement_number == 0 {
        return 0;
    }

    let mut necessary_tiles = 0u64;

    match bingpai.iter().sum::<u8>() {
        n if n % 3 == 1 => {
            for tile in 0..34 {
                if bingpai[tile] >= 4 {
                    continue;
                }

                bingpai[tile] += 1;
                let new_replacement_number = calculate_replacement_number(&bingpai, None).unwrap();
                if new_replacement_number < replacement_number {
                    necessary_tiles |= 1 << tile;
                }
                bingpai[tile] -= 1;
            }
        }
        n if n % 3 == 2 => {
            for tile in 0..34 {
                if bingpai[tile] >= 4 {
                    continue;
                }

                let new_replacement_number = calculate_replacement_number(&bingpai, None).unwrap();
                if new_replacement_number < replacement_number {
                    necessary_tiles |= 1 << tile;
                }
            }
        }
        _ => panic!("invalid hand"),
    }

    necessary_tiles
}

pub fn calculate_unnecessary_tiles(bingpai: &TileCounts) -> u64 {
    let mut bingpai = bingpai.clone();

    let replacement_number = calculate_replacement_number(&bingpai, None).unwrap();
    if replacement_number == 0 {
        return 0;
    }

    let mut unnecessary_tiles = 0u64;
    match bingpai.iter().sum::<u8>() {
        n if n % 3 == 1 => {
            for tile in 0..34 {
                if bingpai[tile] > 0 {
                    let new_replacement_number =
                        calculate_replacement_number(&bingpai, None).unwrap();
                    if new_replacement_number == replacement_number {
                        unnecessary_tiles |= 1 << tile;
                    }
                }
            }
        }
        n if n % 3 == 2 => {
            for tile in 0..34 {
                if bingpai[tile] > 0 {
                    bingpai[tile] -= 1;
                    let new_replacement_number =
                        calculate_replacement_number(&bingpai, None).unwrap();
                    if new_replacement_number == replacement_number {
                        unnecessary_tiles |= 1 << tile;
                    }
                    bingpai[tile] += 1;
                }
            }
        }
        _ => panic!("invalid hand"),
    }
    unnecessary_tiles
}
