// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;

// table[i][n]
// i = 0, 1, ..., N - 1 (N = 9 or 7)
// n = 0, 1, ..., 14
type TableImpl<const N: usize> = [[u32; 15]; N];
type ShupaiTableImpl = TableImpl<9>;
type ZipaiTableImpl = TableImpl<7>;

const INIT_SHUPAI_TABLE: ShupaiTableImpl = [[0; 15]; 9];
const INIT_ZIPAI_TABLE: ZipaiTableImpl = [[0; 15]; 7];

fn build_table<const N: usize>(i: usize, n: usize, table: &mut TableImpl<N>) -> u32 {
    assert!(N == 9 || N == 7);
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
    assert!(N == 9 || N == 7);

    let mut ofs = File::create(table_path)?;
    writeln!(ofs, "// SPDX-FileCopyrightText: 2024 Apricot S.")?;
    writeln!(ofs, "// SPDX-License-Identifier: MIT")?;
    writeln!(
        ofs,
        "// This file is part of https://github.com/Apricot-S/xiangting"
    )?;
    writeln!(ofs)?;

    if N == 9 {
        writeln!(ofs, "use super::core::ShupaiTable;")?;
    } else {
        writeln!(ofs, "use super::core::ZipaiTable;")?;
    }
    writeln!(ofs)?;

    if N == 9 {
        writeln!(ofs, "pub(super) const SHUPAI_SIZE: u32 = {};", table[0][0])?;
    } else {
        writeln!(ofs, "pub(super) const ZIPAI_SIZE: u32 = {};", table[0][0])?;
    }
    writeln!(ofs)?;

    writeln!(ofs, "#[rustfmt::skip]")?;
    if N == 9 {
        writeln!(ofs, "pub(super) const SHUPAI_TABLE: ShupaiTable = [")?;
    } else {
        writeln!(ofs, "pub(super) const ZIPAI_TABLE: ZipaiTable = [")?;
    }

    for i in 0..N {
        writeln!(ofs, "    [")?;
        writeln!(ofs, "        // i = {}", i)?;
        for n in 0..=14 {
            write!(ofs, "        [")?;
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
                write!(ofs, "{}{}", entry, separator)?;
            }
            writeln!(ofs, "], // n = {}", n)?;
        }
        writeln!(ofs, "    ],")?;
    }
    writeln!(ofs, "];")?;

    ofs.flush()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <PATH TO SHUPAI TABLE> <PATH TO ZIPAI TABLE>",
            args[0]
        );
        process::exit(1);
    }

    let shupai_table_path = Path::new(&args[1]);
    let zipai_table_path = Path::new(&args[2]);

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
}
