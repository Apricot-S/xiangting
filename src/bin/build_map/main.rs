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
use xiangting::standard::core::MapValue;
use xiangting::standard::hash::{hash_19m, hash_shupai, hash_zipai};
use xiangting::standard::shupai_table::SHUPAI_SIZE;
use xiangting::standard::wanzi_19_table::WANZI_19_SIZE;
use xiangting::standard::zipai_table::ZIPAI_SIZE;

type Map = Vec<MapValue>;

fn pack_replacement_numbers<const N: usize>(hand: &[u8; N]) -> MapValue {
    assert!([9, 7, 2].contains(&N));
    const MAX_REPLACEMENT_NUMBER: u8 = 9;

    let mut pack = [0u32; 5];
    for num_pair in 0..=1 {
        for num_meld in 0..=4 {
            let (replacement_number, necessary_tiles) = match N {
                9 => {
                    let mut hand9 = [0u8; 9];
                    hand9.copy_from_slice(&hand[0..9]);
                    const INITIAL_WINNING_HAND: [u8; 9] = [0u8; 9];
                    get_shupai_replacement_number(
                        &hand9,
                        num_meld,
                        num_pair,
                        0,
                        0,
                        0,
                        0,
                        0,
                        INITIAL_WINNING_HAND,
                        MAX_REPLACEMENT_NUMBER,
                        0,
                    )
                }
                7 => {
                    let mut hand7 = [0u8; 7];
                    hand7.copy_from_slice(&hand[0..7]);
                    const INITIAL_WINNING_HAND: [u8; 7] = [0u8; 7];
                    get_zipai_replacement_number(
                        &hand7,
                        num_meld,
                        num_pair,
                        0,
                        0,
                        0,
                        INITIAL_WINNING_HAND,
                        MAX_REPLACEMENT_NUMBER,
                        0,
                    )
                }
                2 => {
                    let full_hand = [hand[0], 0, 0, 0, 0, 0, 0, 0, hand[1]];
                    const INITIAL_WINNING_HAND: [u8; 9] = [0u8; 9];
                    get_19m_replacement_number(
                        &full_hand,
                        num_meld,
                        num_pair,
                        0,
                        0,
                        0,
                        INITIAL_WINNING_HAND,
                        MAX_REPLACEMENT_NUMBER,
                        0,
                    )
                }
                _ => unreachable!(),
            };
            let shift = if num_pair == 0 { 0 } else { 16 };
            pack[num_meld as usize] |= (replacement_number as u32) << shift;
            pack[num_meld as usize] |= (necessary_tiles as u32) << (shift + 4);
        }
    }
    pack
}

fn create_entry<const N: usize>(hand: &[u8; N], map: &mut Map) {
    assert!([9, 7, 2].contains(&N));

    let h = match N {
        9 => hash_shupai(hand),
        7 => hash_zipai(hand),
        2 => {
            let full_hand = [hand[0], 0, 0, 0, 0, 0, 0, 0, hand[1]];
            hash_19m(&full_hand)
        }
        _ => unreachable!(),
    };
    map[h] = pack_replacement_numbers(hand);
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
    assert!([9, 7, 2].contains(&N));

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
        9 => writeln!(
            w,
            "pub(super) static SHUPAI_MAP: [MapValue; SHUPAI_SIZE] = ["
        )?,
        7 => writeln!(w, "pub(super) static ZIPAI_MAP: [MapValue; ZIPAI_SIZE] = [")?,
        2 => writeln!(
            w,
            "pub(super) static WANZI_19_MAP: [MapValue; WANZI_19_SIZE] = ["
        )?,
        _ => unreachable!(),
    }

    for &entry in map {
        write!(w, "    [")?;
        for (i, pack) in entry.iter().enumerate() {
            let separator = if i < 4 { ", " } else { "" };
            write!(w, "0x{:X}{}", pack, separator)?;
        }
        writeln!(w, "],")?;
    }

    writeln!(w, "];")?;

    w.flush()?;

    Ok(())
}

fn main() {
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
        shupai_map.resize(SHUPAI_SIZE, [0u32; 5]);
        let mut hand = [0u8; 9];
        build_map(&mut hand, 0, 0, &mut shupai_map);

        dump_map::<9>(&shupai_map, shupai_map_path).expect("Failed to dump shupai map");
    }

    {
        let mut zipai_map = Map::new();
        zipai_map.resize(ZIPAI_SIZE, [0u32; 5]);
        let mut hand = [0u8; 7];
        build_map(&mut hand, 0, 0, &mut zipai_map);

        dump_map::<7>(&zipai_map, zipai_map_path).expect("Failed to dump zipai map");
    }

    {
        let mut wanzi_19_map = Map::new();
        wanzi_19_map.resize(WANZI_19_SIZE, [0u32; 5]);
        let mut hand = [0u8; 2];
        build_map(&mut hand, 0, 0, &mut wanzi_19_map);

        dump_map::<2>(&wanzi_19_map, wanzi_19_map_path).expect("Failed to dump wanzi 19 map");
    }
}
