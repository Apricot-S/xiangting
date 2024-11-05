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
use xiangting::standard::core::{Map, MapValue};
use xiangting::standard::hash::{hash_19m, hash_shupai, hash_zipai};
use xiangting::standard::shupai_table::SHUPAI_SIZE;
use xiangting::standard::wanzi_19_table::WANZI_19_SIZE;
use xiangting::standard::zipai_table::ZIPAI_SIZE;

fn pack_shupai_replacement_numbers(hand: &[u8; 9]) -> MapValue {
    let mut pack = [0u8; 10];
    for num_pair in 0..=1 {
        for num_meld in 0..=4 {
            const MAX_REPLACEMENT_NUMBER: u8 = 9;
            const INITIAL_WINNING_HAND: [u8; 9] = [0u8; 9];
            let min_replacement_number = get_shupai_replacement_number(
                hand,
                num_meld,
                num_pair,
                0,
                0,
                0,
                0,
                0,
                INITIAL_WINNING_HAND,
                MAX_REPLACEMENT_NUMBER,
            );
            pack[(num_meld + num_pair * 5) as usize] = min_replacement_number;
        }
    }
    pack
}

fn pack_zipai_replacement_numbers(hand: &[u8; 7]) -> MapValue {
    let mut pack = [0u8; 10];
    for num_pair in 0..=1 {
        for num_meld in 0..=4 {
            const MAX_REPLACEMENT_NUMBER: u8 = 9;
            const INITIAL_WINNING_HAND: [u8; 7] = [0u8; 7];
            let min_replacement_number = get_zipai_replacement_number(
                hand,
                num_meld,
                num_pair,
                0,
                0,
                0,
                INITIAL_WINNING_HAND,
                MAX_REPLACEMENT_NUMBER,
            );
            pack[(num_meld + num_pair * 5) as usize] = min_replacement_number;
        }
    }
    pack
}

fn pack_19m_replacement_numbers(hand: &[u8; 9]) -> MapValue {
    let mut pack = [0u8; 10];
    for num_pair in 0..=1 {
        for num_meld in 0..=4 {
            const MAX_REPLACEMENT_NUMBER: u8 = 9;
            const INITIAL_WINNING_HAND: [u8; 9] = [0u8; 9];
            let min_replacement_number = get_19m_replacement_number(
                hand,
                num_meld,
                num_pair,
                0,
                0,
                0,
                INITIAL_WINNING_HAND,
                MAX_REPLACEMENT_NUMBER,
            );
            pack[(num_meld + num_pair * 5) as usize] = min_replacement_number;
        }
    }
    pack
}

fn create_shupai_entry(hand: &[u8; 9], map: &mut Map) {
    let h = hash_shupai(hand);
    let entry = pack_shupai_replacement_numbers(hand);
    map[h] = entry;
}

fn create_zipai_entry(hand: &[u8; 7], map: &mut Map) {
    let h = hash_zipai(hand);
    let entry = pack_zipai_replacement_numbers(hand);
    map[h] = entry;
}

fn create_19m_entry(hand: &[u8; 2], map: &mut Map) {
    let full_hand = [hand[0], 0, 0, 0, 0, 0, 0, 0, hand[1]];
    let h = hash_19m(&full_hand);
    let entry = pack_19m_replacement_numbers(&full_hand);
    map[h] = entry;
}

fn build_shupai_map(hand: &mut [u8; 9], i: usize, n: usize, map: &mut Map) {
    debug_assert!(i <= 9);
    debug_assert!(n <= 14);

    if i == 9 {
        create_shupai_entry(hand, map);
        return;
    }

    for c in 0..=4 {
        if n + c > 14 {
            break;
        }

        hand[i] = c as u8;
        build_shupai_map(hand, i + 1, n + c, map);
        hand[i] = 0;
    }
}

fn build_zipai_map(hand: &mut [u8; 7], i: usize, n: usize, map: &mut Map) {
    debug_assert!(i <= 7);
    debug_assert!(n <= 14);

    if i == 7 {
        create_zipai_entry(hand, map);
        return;
    }

    for c in 0..=4 {
        if n + c > 14 {
            break;
        }

        hand[i] = c as u8;
        build_zipai_map(hand, i + 1, n + c, map);
        hand[i] = 0;
    }
}

fn build_19m_map(hand: &mut [u8; 2], i: usize, n: usize, map: &mut Map) {
    debug_assert!(i <= 2);
    debug_assert!(n <= 8);

    if i == 2 {
        create_19m_entry(hand, map);
        return;
    }

    for c in 0..=4 {
        if n + c > 8 {
            break;
        }

        hand[i] = c as u8;
        build_19m_map(hand, i + 1, n + c, map);
        hand[i] = 0;
    }
}

fn dump_shupai_map(map: &Map, map_path: &Path) -> io::Result<()> {
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
    writeln!(w, "use super::shupai_table::SHUPAI_SIZE;")?;
    writeln!(w)?;
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(
        w,
        "pub(super) static SHUPAI_MAP: [MapValue; SHUPAI_SIZE] = ["
    )?;

    for &entry in map {
        write!(w, "    [")?;
        for (i, pack) in entry.iter().enumerate() {
            let separator = if i < 9 { ", " } else { "" };
            write!(w, "{}{}", pack, separator)?;
        }
        writeln!(w, "],")?;
    }

    writeln!(w, "];")?;

    w.flush()?;

    Ok(())
}

fn dump_zipai_map(map: &Map, map_path: &Path) -> io::Result<()> {
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
    writeln!(w, "use super::zipai_table::ZIPAI_SIZE;")?;
    writeln!(w)?;
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(w, "pub(super) static ZIPAI_MAP: [MapValue; ZIPAI_SIZE] = [")?;

    for &entry in map {
        write!(w, "    [")?;
        for (i, pack) in entry.iter().enumerate() {
            let separator = if i < 9 { ", " } else { "" };
            write!(w, "{}{}", pack, separator)?;
        }
        writeln!(w, "],")?;
    }

    writeln!(w, "];")?;

    w.flush()?;

    Ok(())
}

fn dump_19m_map(map: &Map, map_path: &Path) -> io::Result<()> {
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
    writeln!(w, "use super::wanzi_19_table::WANZI_19_SIZE;")?;
    writeln!(w)?;
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(
        w,
        "pub(super) static WANZI_19_MAP: [MapValue; WANZI_19_SIZE] = ["
    )?;

    for &entry in map {
        write!(w, "    [")?;
        for (i, pack) in entry.iter().enumerate() {
            let separator = if i < 9 { ", " } else { "" };
            write!(w, "{}{}", pack, separator)?;
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
        shupai_map.resize(SHUPAI_SIZE, [0; 10]);
        let mut hand = [0u8; 9];
        build_shupai_map(&mut hand, 0, 0, &mut shupai_map);

        dump_shupai_map(&shupai_map, shupai_map_path).expect("Failed to dump shupai map");
    }

    {
        let mut zipai_map = Map::new();
        zipai_map.resize(ZIPAI_SIZE, [0; 10]);
        let mut hand = [0u8; 7];
        build_zipai_map(&mut hand, 0, 0, &mut zipai_map);

        dump_zipai_map(&zipai_map, zipai_map_path).expect("Failed to dump zipai map");
    }

    {
        let mut wanzi_19_map = Map::new();
        wanzi_19_map.resize(WANZI_19_SIZE, [0; 10]);
        let mut hand = [0u8; 2];
        build_19m_map(&mut hand, 0, 0, &mut wanzi_19_map);

        dump_19m_map(&wanzi_19_map, wanzi_19_map_path).expect("Failed to dump wanzi 19 map");
    }
}
