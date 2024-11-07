// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process;

// table[i][n]
// i = 0, 1, ..., N - 1 (N = 9 or 7 or 2)
// n = 0, 1, ..., 14
type TableImpl<const N: usize> = [[u32; 15]; N];
type ShupaiTableImpl = TableImpl<9>;
type ZipaiTableImpl = TableImpl<7>;
type Wanzi19TableImpl = TableImpl<2>;

const INIT_SHUPAI_TABLE: ShupaiTableImpl = [[0; 15]; 9];
const INIT_ZIPAI_TABLE: ZipaiTableImpl = [[0; 15]; 7];
const INIT_WANZI_19_TABLE: Wanzi19TableImpl = [[0; 15]; 2];

fn build_table<const N: usize>(i: usize, n: usize, table: &mut TableImpl<N>) -> u32 {
    assert!([9, 7, 2].contains(&N));
    assert!(i < N);
    assert!(n <= 14);

    if table[i][n] != 0 {
        return table[i][n];
    }

    for c in 0..=4 {
        if n + c > 14 {
            break;
        }

        if (i + 1) < N {
            table[i][n] += build_table(i + 1, n + c, table);
        } else {
            table[i][n] += 1;
        }
    }

    table[i][n]
}

fn dump_table<const N: usize>(table: &TableImpl<N>, table_path: &Path) -> io::Result<()> {
    assert!([9, 7, 2].contains(&N));

    let file = File::create(table_path)?;
    let mut w = BufWriter::new(file);

    writeln!(w, "// SPDX-FileCopyrightText: 2024 Apricot S.")?;
    writeln!(w, "// SPDX-License-Identifier: MIT")?;
    writeln!(
        w,
        "// This file is part of https://github.com/Apricot-S/xiangting"
    )?;
    writeln!(w)?;

    match N {
        9 => writeln!(w, "use super::core::ShupaiTable;")?,
        7 => writeln!(w, "use super::core::ZipaiTable;")?,
        2 => writeln!(w, "use super::core::Wanzi19Table;")?,
        _ => unreachable!(),
    }
    writeln!(w)?;

    match N {
        9 => writeln!(w, "pub const SHUPAI_SIZE: usize = {};", table[0][0])?,
        7 => writeln!(w, "pub const ZIPAI_SIZE: usize = {};", table[0][0])?,
        2 => writeln!(w, "pub const WANZI_19_SIZE: usize = {};", table[0][0])?,
        _ => unreachable!(),
    }
    writeln!(w)?;

    writeln!(w, "#[rustfmt::skip]")?;
    match N {
        9 => writeln!(w, "pub const SHUPAI_TABLE: ShupaiTable = [")?,
        7 => writeln!(w, "pub const ZIPAI_TABLE: ZipaiTable = [")?,
        2 => writeln!(w, "pub const WANZI_19_TABLE: Wanzi19Table = [")?,
        _ => unreachable!(),
    }

    for i in 0..N {
        writeln!(w, "    [")?;
        writeln!(w, "        // i = {}", i)?;
        for n in 0..=14 {
            write!(w, "        [")?;
            for c in 0..=4 {
                let mut entry = 0;
                for cc in 0..c {
                    if n + cc < c || n + cc > 14 + c {
                        break;
                    }
                    if i + 1 < N {
                        entry += table[i + 1][n - c + cc];
                    } else {
                        entry += 1;
                    }
                }
                let separator = if c < 4 { ", " } else { "" };
                write!(w, "{}{}", entry, separator)?;
            }
            writeln!(w, "], // n = {}", n)?;
        }
        writeln!(w, "    ],")?;
    }

    writeln!(w, "];")?;

    w.flush()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <PATH TO SHUPAI TABLE> <PATH TO ZIPAI TABLE> <PATH TO WANZI 19 TABLE>",
            args[0]
        );
        process::exit(1);
    }

    let shupai_table_path = Path::new(&args[1]);
    let zipai_table_path = Path::new(&args[2]);
    let wanzi_19_table_path = Path::new(&args[3]);

    {
        let mut shupai_table = INIT_SHUPAI_TABLE;
        let _ = build_table(0, 0, &mut shupai_table);
        assert_eq!(shupai_table[0][0], 405_350);

        dump_table(&shupai_table, shupai_table_path).expect("Failed to dump shupai table");
    }

    {
        let mut zipai_table = INIT_ZIPAI_TABLE;
        let _ = build_table(0, 0, &mut zipai_table);
        assert_eq!(zipai_table[0][0], 43_130);

        dump_table(&zipai_table, zipai_table_path).expect("Failed to dump zipai table");
    }

    {
        let mut wanzi_19_table = INIT_WANZI_19_TABLE;
        let _ = build_table(0, 0, &mut wanzi_19_table);
        assert_eq!(wanzi_19_table[0][0], 25);

        dump_table(&wanzi_19_table, wanzi_19_table_path).expect("Failed to dump wanzi 19 table");
    }
}
