// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod replacement_number;

use self::replacement_number::{
    get_19m_replacement_number, get_shupai_replacement_number, get_zipai_replacement_number,
};
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process;
use xiangting::standard::core::{NecessaryTilesMapValue, ReplacementNumberMapValue};
use xiangting::standard::hash::{hash_19m, hash_shupai, hash_zipai};
use xiangting::standard::shupai_table::SHUPAI_SIZE;
use xiangting::standard::wanzi_19_table::WANZI_19_SIZE;
use xiangting::standard::zipai_table::ZIPAI_SIZE;

struct MapValue {
    replacement_number: ReplacementNumberMapValue,
    necessary_tiles: NecessaryTilesMapValue,
}

impl Default for MapValue {
    fn default() -> Self {
        MapValue {
            replacement_number: 0u32,
            necessary_tiles: [0u32; 3],
        }
    }
}

type Map = Vec<MapValue>;

fn pack_values<const N: usize>(hand: &[u8; N]) -> MapValue {
    debug_assert!([9, 7, 2].contains(&N));
    const MAX_REPLACEMENT_NUMBER: u8 = 14;

    let mut pack = MapValue::default();

    for num_pair in 0..=1 {
        for num_meld in 0..=4 {
            if num_pair == 0 && num_meld == 0 {
                continue;
            }

            let (replacement_number, necessary_tiles) = match N {
                9 => {
                    let hand9 = hand.first_chunk::<9>().unwrap();
                    let mut initial_target_hand: [u8; 9] = [0u8; 9];
                    get_shupai_replacement_number(
                        hand9,
                        num_meld,
                        num_pair,
                        0,
                        0,
                        0,
                        &mut initial_target_hand,
                        MAX_REPLACEMENT_NUMBER,
                        0,
                    )
                }
                7 => {
                    let hand7 = hand.first_chunk::<7>().unwrap();
                    let mut initial_target_hand: [u8; 7] = [0u8; 7];
                    get_zipai_replacement_number(
                        hand7,
                        num_meld,
                        num_pair,
                        0,
                        0,
                        0,
                        &mut initial_target_hand,
                        MAX_REPLACEMENT_NUMBER,
                        0,
                    )
                }
                2 => {
                    let full_hand = [hand[0], 0, 0, 0, 0, 0, 0, 0, hand[1]];
                    let mut initial_target_hand: [u8; 9] = [0u8; 9];
                    get_19m_replacement_number(
                        &full_hand,
                        num_meld,
                        num_pair,
                        0,
                        0,
                        0,
                        &mut initial_target_hand,
                        MAX_REPLACEMENT_NUMBER,
                        0,
                    )
                }
                _ => unreachable!(),
            };

            match (num_pair, num_meld) {
                (0, 1) => pack.replacement_number |= replacement_number as u32,
                (0, 2) => pack.replacement_number |= (replacement_number as u32) << 2,
                (0, 3) => pack.replacement_number |= (replacement_number as u32) << 5,
                (0, 4) => pack.replacement_number |= (replacement_number as u32) << 9,
                (1, 0) => pack.replacement_number |= (replacement_number as u32) << 13,
                (1, 1) => pack.replacement_number |= (replacement_number as u32) << 15,
                (1, 2) => pack.replacement_number |= (replacement_number as u32) << 18,
                (1, 3) => pack.replacement_number |= (replacement_number as u32) << 22,
                (1, 4) => pack.replacement_number |= (replacement_number as u32) << 26,
                _ => unreachable!(),
            }

            match (num_pair, num_meld) {
                (0, 1) => pack.necessary_tiles[0] |= necessary_tiles as u32,
                (0, 2) => pack.necessary_tiles[0] |= (necessary_tiles as u32) << 9,
                (0, 3) => pack.necessary_tiles[0] |= (necessary_tiles as u32) << (9 * 2),
                (0, 4) => pack.necessary_tiles[1] |= necessary_tiles as u32,
                (1, 0) => pack.necessary_tiles[1] |= (necessary_tiles as u32) << 9,
                (1, 1) => pack.necessary_tiles[1] |= (necessary_tiles as u32) << (9 * 2),
                (1, 2) => pack.necessary_tiles[2] |= necessary_tiles as u32,
                (1, 3) => pack.necessary_tiles[2] |= (necessary_tiles as u32) << 9,
                (1, 4) => pack.necessary_tiles[2] |= (necessary_tiles as u32) << (9 * 2),
                _ => unreachable!(),
            }
        }
    }

    pack
}

fn create_entry<const N: usize>(hand: &[u8; N], map: &mut Map) {
    debug_assert!([9, 7, 2].contains(&N));

    let h = match N {
        9 => hash_shupai(hand),
        7 => hash_zipai(hand),
        2 => {
            let full_hand = [hand[0], 0, 0, 0, 0, 0, 0, 0, hand[1]];
            hash_19m(&full_hand)
        }
        _ => unreachable!(),
    };
    map[h] = pack_values(hand);
}

fn build_map<const N: usize>(hand: &mut [u8; N], i: usize, n: usize, map: &mut Map) {
    debug_assert!([9, 7, 2].contains(&N));
    debug_assert!(i <= N);
    debug_assert!(n <= 14);

    if i == N {
        create_entry(hand, map);
        return;
    }

    for c in 0..=4 {
        if n + c > 14 {
            break;
        }

        hand[i] = c as u8;
        build_map(hand, i + 1, n + c, map);
        hand[i] = 0;
    }
}

fn dump_map<const N: usize>(map: &Map, map_path: &Path) -> io::Result<()> {
    debug_assert!([9, 7, 2].contains(&N));

    let file = File::create(map_path)?;
    let mut w = BufWriter::new(file);

    writeln!(w, "// SPDX-FileCopyrightText: 2024 Apricot S.")?;
    writeln!(w, "// SPDX-License-Identifier: MIT")?;
    writeln!(
        w,
        "// This file is part of https://github.com/Apricot-S/xiangting"
    )?;
    writeln!(w)?;
    writeln!(w, "use super::core::MapValue;")?;

    match N {
        9 => writeln!(w, "use super::shupai_table::SHUPAI_SIZE;")?,
        7 => writeln!(w, "use super::zipai_table::ZIPAI_SIZE;")?,
        2 => writeln!(w, "use super::wanzi_19_table::WANZI_19_SIZE;")?,
        _ => unreachable!(),
    }

    writeln!(w)?;
    writeln!(w, "#[rustfmt::skip]")?;

    match N {
        9 => write!(
            w,
            "pub(super) static SHUPAI_MAP: [MapValue; SHUPAI_SIZE] = ["
        )?,
        7 => write!(w, "pub(super) static ZIPAI_MAP: [MapValue; ZIPAI_SIZE] = [")?,
        2 => write!(
            w,
            "pub(super) static WANZI_19_MAP: [MapValue; WANZI_19_SIZE] = ["
        )?,
        _ => unreachable!(),
    }

    for entry in map {
        write!(w, "[")?;
        write!(w, "{},", entry.replacement_number)?;
        write!(w, "{},", entry.necessary_tiles[0])?;
        write!(w, "{},", entry.necessary_tiles[1])?;
        write!(w, "{}", entry.necessary_tiles[2])?;
        write!(w, "],")?;
    }

    writeln!(w, "];")?;

    w.flush()?;

    Ok(())
}

fn main() {
    let start = std::time::Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <PATH TO SHUPAI MAP> <PATH TO ZIPAI MAP> <PATH TO WANZI 19 MAP>",
            args[0]
        );
        process::exit(1);
    }

    let shupai_map_path = Path::new(&args[1]);
    let zipai_map_path = Path::new(&args[2]);
    let wanzi_19_map_path = Path::new(&args[3]);

    {
        let mut shupai_map = Map::new();
        shupai_map.resize_with(SHUPAI_SIZE, Default::default);
        let mut hand = [0u8; 9];
        build_map(&mut hand, 0, 0, &mut shupai_map);

        dump_map::<9>(&shupai_map, shupai_map_path).expect("Failed to dump shupai map");
    }

    {
        let mut zipai_map = Map::new();
        zipai_map.resize_with(ZIPAI_SIZE, Default::default);
        let mut hand = [0u8; 7];
        build_map(&mut hand, 0, 0, &mut zipai_map);

        dump_map::<7>(&zipai_map, zipai_map_path).expect("Failed to dump zipai map");
    }

    {
        let mut wanzi_19_map = Map::new();
        wanzi_19_map.resize_with(WANZI_19_SIZE, Default::default);
        let mut hand = [0u8; 2];
        build_map(&mut hand, 0, 0, &mut wanzi_19_map);

        dump_map::<2>(&wanzi_19_map, wanzi_19_map_path).expect("Failed to dump wanzi 19 map");
    }

    let elapsed_time = start.elapsed();
    println!("elapsed time: {:?}", elapsed_time);
}
