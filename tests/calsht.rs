// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/Apricot-S/xiangting

// Dummy implementation
#[allow(dead_code)]
pub(crate) struct Calsht {
    #[allow(dead_code)]
    mp1: usize,
    #[allow(dead_code)]
    mp2: usize,
}

impl Calsht {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Calsht {
            mp1: 1953125,
            mp2: 78125,
        }
    }

    #[allow(dead_code, unused_variables)]
    pub(crate) fn initialize(&mut self, dir: &str) {
        // Dummy implementation
    }

    #[allow(dead_code, unused_variables)]
    pub(crate) fn operator(&self, t: &Vec<i32>, m: i32, mode: i32) -> (i32, i32) {
        // Dummy implementation
        (1, 1)
    }
}
