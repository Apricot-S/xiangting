// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::{BlockCountImpl, SingleColorTileFlag};
use xiangting::standard::core::BlockCount;

pub(in super::super) fn count_19m_block(wanzi_19_bingpai: &[u8; 2]) -> BlockCount {
    wanzi_19_bingpai
        .iter()
        .enumerate()
        .fold(
            BlockCountImpl {
                num_mianzi: 0,
                num_dazi: 0,
                num_duizi: 0,
                num_gulipai: 0,
                gulipai: SingleColorTileFlag::ZERO,
                four_tiles_gulipai: SingleColorTileFlag::ZERO,
            },
            |mut acc, (i, &n)| {
                match n {
                    4 => {
                        acc.num_mianzi += 1;
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);

                        acc.four_tiles_gulipai.set(i, true);
                    }
                    3 => acc.num_mianzi += 1,
                    2 => acc.num_duizi += 1,
                    1 => {
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);
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
    fn count_19m_block_works() {
        let wanzi_bingpai = [4, 0, 0, 0, 0, 0, 0, 0, 2];
        let r = count_19m_block(&[wanzi_bingpai[0], wanzi_bingpai[8]]);
        assert_eq!(r.0, 1);
        assert_eq!(r.1, 0);
        assert_eq!(r.2, 1);
        assert_eq!(r.3, 1);
    }
}
