// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::hash::{hash_19m, hash_shupai, hash_zipai};
use super::shupai_map::{SHUPAI_NECESSARY_TILES_MAP, SHUPAI_REPLACEMENT_NUMBER_MAP};
use super::unpack::{UnpackedNumbers, unpack_necessary_tiles, unpack_replacement_number};
use super::wanzi_19_map::{WANZI_19_NECESSARY_TILES_MAP, WANZI_19_REPLACEMENT_NUMBER_MAP};
use super::zipai_map::{ZIPAI_NECESSARY_TILES_MAP, ZIPAI_REPLACEMENT_NUMBER_MAP};
use crate::shoupai::{Shoupai, Shoupai3Player};
use crate::tile::TileFlags;
use std::cmp::Ordering;

struct Entry {
    numbers: UnpackedNumbers,
    tiles: [TileFlags; 10],
}

#[inline]
fn update_min(
    lhs_number: &mut u8,
    lhs_tiles: &mut TileFlags,
    rhs_number: u8,
    rhs_tiles: TileFlags,
) {
    match (*lhs_number).cmp(&rhs_number) {
        Ordering::Less => (),
        Ordering::Equal => *lhs_tiles |= rhs_tiles,
        Ordering::Greater => {
            *lhs_number = rhs_number;
            *lhs_tiles = rhs_tiles;
        }
    }
}

fn update_dp(lhs: &mut Entry, rhs: &Entry) {
    for i in (5..10).rev() {
        // The original expression is
        // ```
        // let mut number = lhs.numbers[i] + rhs.numbers[0];
        // let mut tiles = lhs.tiles[i] | rhs.tiles[0];
        // update_min(
        //     &mut number,
        //     &mut tiles,
        //     lhs.numbers[0] + rhs.numbers[i],
        //     rhs.tiles[i] | rhs.tiles[i],
        // );
        // ```
        // However, since lhs[0] and rhs[0] are always 0, the calculation can be omitted.
        let mut number = lhs.numbers[i];
        let mut tiles = lhs.tiles[i];
        update_min(&mut number, &mut tiles, rhs.numbers[i], rhs.tiles[i]);

        for j in 5..i {
            update_min(
                &mut number,
                &mut tiles,
                lhs.numbers[j] + rhs.numbers[i - j],
                lhs.tiles[j] | rhs.tiles[i - j],
            );
            update_min(
                &mut number,
                &mut tiles,
                lhs.numbers[i - j] + rhs.numbers[j],
                lhs.tiles[i - j] | rhs.tiles[j],
            );
        }

        lhs.numbers[i] = number;
        lhs.tiles[i] = tiles;
    }

    // Skip the case when i = 0, as the inner loop would not run, leading to redundant assignments.
    for i in (1..5).rev() {
        // The original expression is
        // ```
        // let mut number = lhs.numbers[i] + rhs.numbers[0];
        // let mut tiles = lhs.tiles[i] | rhs.tiles[0];
        // ```
        // However, since rhs[0] is always 0, the calculation can be omitted.
        let mut number = lhs.numbers[i];
        let mut tiles = lhs.tiles[i];

        for j in 0..i {
            update_min(
                &mut number,
                &mut tiles,
                lhs.numbers[j] + rhs.numbers[i - j],
                lhs.tiles[j] | rhs.tiles[i - j],
            );
        }

        lhs.numbers[i] = number;
        lhs.tiles[i] = tiles;
    }
}

pub(in super::super) fn calculate_necessary_tiles(shoupai: &Shoupai) -> (u8, TileFlags) {
    let hash_m = hash_shupai(&shoupai.bingpai()[0..9]);
    let hash_p = hash_shupai(&shoupai.bingpai()[9..18]);
    let hash_s = hash_shupai(&shoupai.bingpai()[18..27]);
    let hash_z = hash_zipai(&shoupai.bingpai()[27..34]);

    let packed_rn_m = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_m];
    let packed_rn_p = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_p];
    let packed_rn_s = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_s];
    let packed_rn_z = &ZIPAI_REPLACEMENT_NUMBER_MAP[hash_z];
    let packed_nt_m = &SHUPAI_NECESSARY_TILES_MAP[hash_m];
    let packed_nt_p = &SHUPAI_NECESSARY_TILES_MAP[hash_p];
    let packed_nt_s = &SHUPAI_NECESSARY_TILES_MAP[hash_s];
    let packed_nt_z = &ZIPAI_NECESSARY_TILES_MAP[hash_z];

    let replacement_number_m = unpack_replacement_number(packed_rn_m);
    let replacement_number_p = unpack_replacement_number(packed_rn_p);
    let replacement_number_s = unpack_replacement_number(packed_rn_s);
    let replacement_number_z = unpack_replacement_number(packed_rn_z);
    let necessary_tiles_m = unpack_necessary_tiles(packed_nt_m);
    let necessary_tiles_p = unpack_necessary_tiles(packed_nt_p);
    let necessary_tiles_s = unpack_necessary_tiles(packed_nt_s);
    let necessary_tiles_z = unpack_necessary_tiles(packed_nt_z);

    let (mut entry0, entry1, entry2, entry3) = match shoupai.tile_counts() {
        None => (
            Entry {
                numbers: replacement_number_m,
                tiles: necessary_tiles_m.map(|t| t as TileFlags),
            },
            Entry {
                numbers: replacement_number_p,
                tiles: necessary_tiles_p.map(|t| (t as TileFlags) << 9),
            },
            Entry {
                numbers: replacement_number_s,
                tiles: necessary_tiles_s.map(|t| (t as TileFlags) << 18),
            },
            Entry {
                numbers: replacement_number_z,
                tiles: necessary_tiles_z.map(|t| (t as TileFlags) << 27),
            },
        ),
        Some(_) => {
            unimplemented!();
        }
    };

    update_dp(&mut entry0, &entry1);
    update_dp(&mut entry0, &entry2);
    update_dp(&mut entry0, &entry3);

    let n = 5 + shoupai.num_required_bingpai_mianzi() as usize;
    (entry0.numbers[n], entry0.tiles[n])
}

pub(in super::super) fn calculate_necessary_tiles_3_player(
    shoupai: &Shoupai3Player,
) -> (u8, TileFlags) {
    let hash_m = hash_19m(&shoupai.bingpai()[0..9]);
    let hash_p = hash_shupai(&shoupai.bingpai()[9..18]);
    let hash_s = hash_shupai(&shoupai.bingpai()[18..27]);
    let hash_z = hash_zipai(&shoupai.bingpai()[27..34]);

    let packed_rn_m = &WANZI_19_REPLACEMENT_NUMBER_MAP[hash_m];
    let packed_rn_p = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_p];
    let packed_rn_s = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_s];
    let packed_rn_z = &ZIPAI_REPLACEMENT_NUMBER_MAP[hash_z];
    let packed_nt_m = &WANZI_19_NECESSARY_TILES_MAP[hash_m];
    let packed_nt_p = &SHUPAI_NECESSARY_TILES_MAP[hash_p];
    let packed_nt_s = &SHUPAI_NECESSARY_TILES_MAP[hash_s];
    let packed_nt_z = &ZIPAI_NECESSARY_TILES_MAP[hash_z];

    let replacement_number_m = unpack_replacement_number(packed_rn_m);
    let replacement_number_p = unpack_replacement_number(packed_rn_p);
    let replacement_number_s = unpack_replacement_number(packed_rn_s);
    let replacement_number_z = unpack_replacement_number(packed_rn_z);
    let necessary_tiles_m = unpack_necessary_tiles(packed_nt_m);
    let necessary_tiles_p = unpack_necessary_tiles(packed_nt_p);
    let necessary_tiles_s = unpack_necessary_tiles(packed_nt_s);
    let necessary_tiles_z = unpack_necessary_tiles(packed_nt_z);

    let (mut entry0, entry1, entry2, entry3) = match shoupai.tile_counts() {
        None => (
            Entry {
                numbers: replacement_number_m,
                tiles: necessary_tiles_m.map(|t| t as TileFlags),
            },
            Entry {
                numbers: replacement_number_p,
                tiles: necessary_tiles_p.map(|t| (t as TileFlags) << 9),
            },
            Entry {
                numbers: replacement_number_s,
                tiles: necessary_tiles_s.map(|t| (t as TileFlags) << 18),
            },
            Entry {
                numbers: replacement_number_z,
                tiles: necessary_tiles_z.map(|t| (t as TileFlags) << 27),
            },
        ),
        Some(_) => {
            unimplemented!();
        }
    };

    update_dp(&mut entry0, &entry1);
    update_dp(&mut entry0, &entry2);
    update_dp(&mut entry0, &entry3);

    let n = 5 + shoupai.num_required_bingpai_mianzi() as usize;
    (entry0.numbers[n], entry0.tiles[n])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::tile::{TileCounts, TileFlags};

    #[test]
    fn calculate_necessary_tiles_shisanyao_13() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 9);
        assert_eq!(
            necessary_tiles,
            TileFlags::from_code("123789m123789p123789s1234567z")
        );
    }
}
