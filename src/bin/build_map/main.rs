// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process;
use xiangting::standard::core::{Map, MapValue};
use xiangting::standard::hash::{hash_shupai, hash_zipai};
use xiangting::standard::shupai_table::SHUPAI_SIZE;
use xiangting::standard::zipai_table::ZIPAI_SIZE;

fn create_entry<const N: usize>(hand: &[u8; N], map: &mut Map)
where
    [(); N]:,
{
    assert!(N == 9 || N == 7);

    let h = match N {
        9 => hash_shupai(hand),
        7 => hash_zipai(hand),
        _ => unreachable!(),
    };

    let entry = [0; 5];

    map[h] = entry;
}

fn build_map<const N: usize>(hand: &mut [u8; N], i: usize, n: usize, map: &mut Map)
where
    [(); N]:,
{
    assert!(N == 9 || N == 7);
    assert!(i <= N);
    assert!(n <= 14);

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
    assert!(N == 9 || N == 7);

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
        _ => unreachable!(),
    }
    writeln!(w)?;

    writeln!(w, "#[rustfmt::skip]")?;
    match N {
        9 => writeln!(
            w,
            "pub(super) const SHUPAI_MAP: [MapValue; SHUPAI_SIZE] = ["
        )?,
        7 => writeln!(w, "pub(super) const ZIPAI_MAP: [MapValue; ZIPAI_SIZE] = [")?,
        _ => unreachable!(),
    }

    for &entry in map {
        write!(w, "    [")?;
        for (i, pack) in entry.iter().enumerate() {
            let separator = if i < 4 { ", " } else { "" };
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
        let mut shupai_map = Map::new();
        shupai_map.resize(SHUPAI_SIZE as usize, [0; 5]);
        let mut hand = [0u8; 9];
        build_map(&mut hand, 0, 0, &mut shupai_map);

        dump_map::<9>(&shupai_map, shupai_map_path).expect("Failed to dump shupai map");
    }

    {
        let mut zipai_map = Map::new();
        zipai_map.resize(ZIPAI_SIZE, [0; 5]);
        let mut hand = [0u8; 7];
        build_map(&mut hand, 0, 0, &mut zipai_map);

        dump_map::<7>(&zipai_map, zipai_map_path).expect("Failed to dump zipai map");
    }
}
