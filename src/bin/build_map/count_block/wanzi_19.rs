// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::{SingleColorTileFlag, Wanzi19BlockCountImpl};
use xiangting::standard::core::Wanzi19BlockCount;

pub(in super::super) fn count_19m_block(wanzi_bingpai: &[u8; 9]) -> Wanzi19BlockCount {
    wanzi_bingpai
        .iter()
        .enumerate()
        .fold(
            Wanzi19BlockCountImpl {
                num_mianzi: 0,
                num_duizi: 0,
                num_gulipai: 0,
                gulipai: SingleColorTileFlag::ZERO,
                shuangpeng_ting: SingleColorTileFlag::ZERO,
            },
            |mut acc, (i, &n)| {
                if i == 0 || i == 8 {
                    match n {
                        4 => {
                            acc.num_mianzi += 1;
                            acc.num_gulipai += 1;
                            acc.gulipai.set(i, true);
                        }
                        3 => acc.num_mianzi += 1,
                        2 => {
                            acc.num_duizi += 1;
                            acc.shuangpeng_ting.set(i, true);
                        }
                        1 => {
                            acc.num_gulipai += 1;
                            acc.gulipai.set(i, true);
                        }
                        0 => (),
                        _ => panic!("There are 5 or more of the same tiles: {} tiles", n),
                    }
                }

                acc
            },
        )
        .to_entry()
}

#[cfg(test)]
mod tests {
    use super::*;
    use xiangting::standard::core::Wanzi19BlockCountExt;

    #[test]
    fn count_19m_block_works() {
        let wanzi_bingpai = [4, 0, 0, 0, 0, 0, 0, 0, 2];
        let r = count_19m_block(&wanzi_bingpai);
        assert_eq!(r.num_mianzi(), 1);
        assert_eq!(r.num_duizi(), 1);
        assert_eq!(r.num_gulipai(), 1);
        assert_eq!(r.gulipai(), 0b000000001);
        assert_eq!(r.shuangpeng_ting(), 0b100000000);
    }

    #[test]
    fn count_19m_block_empty() {
        let wanzi_bingpai = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        let r = count_19m_block(&wanzi_bingpai);
        assert_eq!(r.num_mianzi(), 0);
        assert_eq!(r.num_duizi(), 0);
        assert_eq!(r.num_gulipai(), 0);
        assert_eq!(r.gulipai(), 0b000000000);
        assert_eq!(r.shuangpeng_ting(), 0b000000000);
    }

    #[test]
    #[should_panic]
    fn count_19m_block_5_tiles() {
        let wanzi_bingpai = [5, 0, 0, 0, 0, 0, 0, 0, 0];
        let _ = count_19m_block(&wanzi_bingpai);
    }

    #[test]
    fn count_19m_block_ignore_2m_to_8m() {
        let wanzi_bingpai = [4, 3, 5, 0, 0, 0, 0, 1, 2];
        let r = count_19m_block(&wanzi_bingpai);
        assert_eq!(r.num_mianzi(), 1);
        assert_eq!(r.num_duizi(), 1);
        assert_eq!(r.num_gulipai(), 1);
        assert_eq!(r.gulipai(), 0b000000001);
        assert_eq!(r.shuangpeng_ting(), 0b100000000);
    }
}
