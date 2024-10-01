// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::{SingleColorTileFlag, ZipaiBlockCountImpl};
use xiangting::standard::core::ZipaiBlockCount;

pub(in super::super) fn count_zipai_block(zipai_bingpai: &[u8; 7]) -> ZipaiBlockCount {
    zipai_bingpai
        .iter()
        .enumerate()
        .fold(
            ZipaiBlockCountImpl {
                num_mianzi: 0,
                num_duizi: 0,
                num_gulipai: 0,
                gulipai: SingleColorTileFlag::ZERO,
                shuangpeng_ting: SingleColorTileFlag::ZERO,
                danqi_ting: SingleColorTileFlag::ZERO,
            },
            |mut acc, (i, &n)| {
                match n {
                    4 => {
                        acc.num_mianzi += 1;
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);
                        acc.danqi_ting.set(i, true);
                    }
                    3 => acc.num_mianzi += 1,
                    2 => {
                        acc.num_duizi += 1;
                        acc.shuangpeng_ting.set(i, true);
                    }
                    1 => {
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);
                        acc.danqi_ting.set(i, true);
                    }
                    0 => (),
                    _ => panic!("There are 5 or more of the same tiles: {} tiles", n),
                }

                acc
            },
        )
        .to_entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_zipai_block_works() {
        let zipai_bingpai = [1, 0, 3, 1, 2, 4, 0];
        let r = count_zipai_block(&zipai_bingpai);
        assert_eq!(r.0, 2);
        assert_eq!(r.1, 1);
        assert_eq!(r.2, 3);
        assert_eq!(r.3, 0b0101001);
        assert_eq!(r.4, 0b0010000);
        assert_eq!(r.5, 0b0101001);
    }

    #[test]
    fn count_zipai_block_empty() {
        let zipai_bingpai = [0, 0, 0, 0, 0, 0, 0];
        let r = count_zipai_block(&zipai_bingpai);
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 0);
        assert_eq!(r.2, 0);
        assert_eq!(r.3, 0b0000000);
        assert_eq!(r.4, 0b0000000);
        assert_eq!(r.5, 0b0000000);
    }

    #[test]
    #[should_panic]
    fn count_zipai_block_5_tiles() {
        let zipai_bingpai = [5, 0, 0, 0, 0, 0, 0];
        let _ = count_zipai_block(&zipai_bingpai);
    }
}
