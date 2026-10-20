// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

//! Deterministic pattern interning and honor -> sou -> pin -> man composition.
//! Accepts both the original packed maps and maps already converted to IDs.

use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

type Costs = [u8; 10];

fn unpack(v: u32) -> Costs {
    let shifts = [0, 0, 2, 5, 9, 13, 15, 18, 22, 26];
    let masks = [0, 3, 7, 15, 15, 3, 7, 15, 15, 15];
    core::array::from_fn(|i| u8::try_from((v >> shifts[i]) & masks[i]).unwrap())
}

fn combine(a: &Costs, b: &Costs) -> Costs {
    core::array::from_fn(|k| {
        let (h, m) = (k / 5, k % 5);
        (0..=h)
            .flat_map(|p| (0..=m).map(move |n| a[p * 5 + n] + b[(h - p) * 5 + m - n]))
            .min()
            .unwrap()
    })
}

fn patterns(values: impl Iterator<Item = Costs>) -> Vec<Costs> {
    let result: Vec<_> = values.collect::<BTreeSet<_>>().into_iter().collect();
    assert!(result.len() <= 256, "pattern IDs must fit in u8");
    result
}

fn id(values: &[Costs], value: &Costs) -> u8 {
    u8::try_from(values.binary_search(value).unwrap()).unwrap()
}

fn dump<T: Debug>(w: &mut impl Write, name: &str, ty: &str, values: &[T]) -> io::Result<()> {
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(w, "pub(super) static {name}: [{ty}; {}] = [", values.len())?;
    for chunk in values.chunks(32) {
        for v in chunk {
            write!(w, "{v:?},")?;
        }
        writeln!(w)?;
    }
    writeln!(w, "];\n")
}

pub(super) fn write_number_map(
    w: &mut impl Write,
    name: &str,
    packed: impl Iterator<Item = u32>,
) -> io::Result<()> {
    let costs: Vec<_> = packed.map(unpack).collect();
    let dictionary = patterns(costs.iter().copied());
    dump(w, &format!("{name}_PATTERNS"), "[u8; 10]", &dictionary)?;
    let ids: Vec<_> = costs.iter().map(|c| id(&dictionary, c)).collect();
    dump(w, &format!("{name}_PATTERN_MAP"), "u8", &ids)
}

fn numbers(source: &str, name: &str) -> Vec<u32> {
    let start = source.find(&format!("static {name}:")).unwrap();
    let body = source[start..]
        .split_once("= [")
        .unwrap()
        .1
        .split_once("];")
        .unwrap()
        .0;
    body.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

#[allow(clippy::chunks_exact_to_as_chunks)] // as_chunks requires Rust 1.88; MSRV is 1.85.
#[allow(clippy::manual_is_multiple_of)] // is_multiple_of requires Rust 1.87.
fn read_map(path: &Path, name: &str) -> io::Result<Vec<Costs>> {
    let source = fs::read_to_string(path)?;
    if source.contains(&format!("static {name}_REPLACEMENT_NUMBER_MAP:")) {
        let packed = numbers(&source, &format!("{name}_REPLACEMENT_NUMBER_MAP"));
        let dictionary = patterns(packed.iter().copied().map(unpack));
        let start = source[..source
            .find(&format!("static {name}_REPLACEMENT_NUMBER_MAP:"))
            .unwrap()]
            .rfind("#[rustfmt::skip]")
            .unwrap();
        let end = start + source[start..].find("];").unwrap() + 2;
        let mut generated = Vec::new();
        write_number_map(&mut generated, name, packed.into_iter())?;
        let rewritten = format!(
            "{}{}{}",
            &source[..start],
            String::from_utf8(generated).unwrap(),
            source[end..].trim_start_matches('\n')
        )
        .replace(
            "NecessaryTilesMapValue, ReplacementNumberMapValue, UnnecessaryTilesMapValue",
            "NecessaryTilesMapValue, UnnecessaryTilesMapValue",
        );
        fs::write(path, rewritten)?;
        Ok(dictionary)
    } else {
        let values = numbers(&source, &format!("{name}_PATTERNS"));
        assert!(
            !values.is_empty() && values.len() % 10 == 0,
            "invalid cost dictionary"
        );
        Ok(values
            .chunks_exact(10)
            .map(|c| core::array::from_fn(|i| u8::try_from(c[i]).unwrap()))
            .collect())
    }
}

fn merge_table(a: &[Costs], b: &[Costs]) -> (Vec<Costs>, Vec<u8>) {
    let combined: Vec<_> = a
        .iter()
        .flat_map(|x| b.iter().map(move |y| combine(x, y)))
        .collect();
    let dictionary = patterns(combined.iter().copied());
    let ids = combined.iter().map(|c| id(&dictionary, c)).collect();
    (dictionary, ids)
}

#[allow(clippy::similar_names)] // zs and zsp name successive composition stages.
fn build(args: &[String]) -> io::Result<()> {
    let s = read_map(Path::new(&args[0]), "SHUPAI")?;
    let z = read_map(Path::new(&args[1]), "ZIPAI")?;
    let m = read_map(Path::new(&args[2]), "WANZI_19")?;
    let (zs, zs_ids) = merge_table(&z, &s);
    let (zsp, zsp_ids) = merge_table(&zs, &s);
    let (four, four_ids) = merge_table(&zsp, &s);
    let (three, three_ids) = merge_table(&zsp, &m);
    assert_eq!(
        four, three,
        "the final dictionary must be shared by both rules"
    );
    let mut w = Vec::new();
    writeln!(
        w,
        "// SPDX-FileCopyrightText: 2026 Apricot S.\n// SPDX-License-Identifier: MIT\n// Generated by xtask build-lut; do not edit.\n"
    )?;
    dump(&mut w, "ZS_PATTERNS", "[u8; 10]", &zs)?;
    dump(&mut w, "ZSP_PATTERNS", "[u8; 10]", &zsp)?;
    dump(&mut w, "MERGE_ZS", "u8", &zs_ids)?;
    dump(&mut w, "MERGE_ZSP", "u8", &zsp_ids)?;
    let direct_four: Vec<[u8; 5]> = four_ids
        .iter()
        .map(|&i| four[usize::from(i)][5..].try_into().unwrap())
        .collect();
    let direct_three: Vec<[u8; 5]> = three_ids
        .iter()
        .map(|&i| four[usize::from(i)][5..].try_into().unwrap())
        .collect();
    dump(&mut w, "DIRECT_FOUR", "[u8; 5]", &direct_four)?;
    dump(&mut w, "DIRECT_THREE", "[u8; 5]", &direct_three)?;
    // rustfmt removes a trailing empty line even on skipped array literals.
    assert!(w.ends_with(b"\n\n"));
    w.pop();
    fs::write(&args[3], w)
}

pub(super) fn run(args: &[String]) {
    assert_eq!(
        args.len(),
        4,
        "build-lut <SHUPAI MAP> <ZIPAI MAP> <WANZI 19 MAP> <LUT>"
    );
    build(args).unwrap();
}
