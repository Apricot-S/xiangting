// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process;
use xiangting::standard::core::{ShupaiMap, ZipaiMap};
use xiangting::standard::hash::{hash_shupai, hash_zipai};
use xiangting::standard::shupai_table::SHUPAI_SIZE;
use xiangting::standard::zipai_table::ZIPAI_SIZE;

fn create_shupai_entry(hand: &[u8; 9], map: &mut ShupaiMap) {
    let h = hash_shupai(hand);
    let entry = [(0, 0, 0, 0, 0, 0); 2];
    map[h] = entry;
}

fn create_zipai_entry(hand: &[u8; 7], map: &mut ZipaiMap) {
    let h = hash_zipai(hand);
    let entry = (0, 0, 0, 0, 0, 0);
    map[h] = entry;
}

fn build_shupai_map(hand: &mut [u8; 9], i: usize, n: usize, map: &mut ShupaiMap) {
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

fn build_zipai_map(hand: &mut [u8; 7], i: usize, n: usize, map: &mut ZipaiMap) {
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

fn dump_shupai_map(map: &ShupaiMap, map_path: &Path) -> io::Result<()> {
    let file = File::create(map_path)?;
    let mut w = BufWriter::new(file);

    writeln!(w, "// SPDX-FileCopyrightText: 2024 Apricot S.")?;
    writeln!(w, "// SPDX-License-Identifier: MIT")?;
    writeln!(
        w,
        "// This file is part of https://github.com/Apricot-S/xiangting"
    )?;
    writeln!(w)?;
    writeln!(w, "use super::core::ShupaiMapValue;")?;
    writeln!(w, "use super::shupai_table::SHUPAI_SIZE;")?;
    writeln!(w)?;
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(
        w,
        "pub(super) const SHUPAI_MAP: [ShupaiMapValue; SHUPAI_SIZE] = ["
    )?;

    for &entry in map {
        write!(
            w,
            "    [({}, {}, {}, {}, 0b{:09b}, 0b{:09b}), ",
            entry[0].0, entry[0].1, entry[0].2, entry[0].3, entry[0].4, entry[0].5,
        )?;
        writeln!(
            w,
            "({}, {}, {}, {}, 0b{:09b}, 0b{:09b})],",
            entry[1].0, entry[1].1, entry[1].2, entry[1].3, entry[1].4, entry[1].5,
        )?;
    }
    writeln!(w, "];")?;

    w.flush()?;

    Ok(())
}

fn dump_zipai_map(map: &ZipaiMap, map_path: &Path) -> io::Result<()> {
    let file = File::create(map_path)?;
    let mut w = BufWriter::new(file);

    writeln!(w, "// SPDX-FileCopyrightText: 2024 Apricot S.")?;
    writeln!(w, "// SPDX-License-Identifier: MIT")?;
    writeln!(
        w,
        "// This file is part of https://github.com/Apricot-S/xiangting"
    )?;
    writeln!(w)?;
    writeln!(w, "use super::core::ZipaiMapValue;")?;
    writeln!(w, "use super::zipai_table::ZIPAI_SIZE;")?;
    writeln!(w)?;
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(
        w,
        "pub(super) const ZIPAI_MAP: [ZipaiMapValue; ZIPAI_SIZE] = ["
    )?;

    for &entry in map {
        writeln!(
            w,
            "    ({}, {}, {}, {}, 0b{:07b}, 0b{:07b}),",
            entry.0, entry.1, entry.2, entry.3, entry.4, entry.5
        )?;
    }
    writeln!(w, "];")?;

    w.flush()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <PATH TO SHUPAI MAP> <PATH TO ZIPAI MAP>",
            args[0]
        );
        process::exit(1);
    }

    let shupai_map_path = Path::new(&args[1]);
    let zipai_map_path = Path::new(&args[2]);

    {
        let mut shupai_map = ShupaiMap::new();
        shupai_map.resize(SHUPAI_SIZE, [(0, 0, 0, 0, 0, 0); 2]);
        let mut hand = [0u8; 9];
        build_shupai_map(&mut hand, 0, 0, &mut shupai_map);

        dump_shupai_map(&shupai_map, shupai_map_path).expect("Failed to dump shupai map");
    }

    {
        let mut zipai_map = ZipaiMap::new();
        zipai_map.resize(ZIPAI_SIZE, (0, 0, 0, 0, 0, 0));
        let mut hand = [0u8; 7];
        build_zipai_map(&mut hand, 0, 0, &mut zipai_map);

        dump_zipai_map(&zipai_map, zipai_map_path).expect("Failed to dump zipai map");
    }
}
