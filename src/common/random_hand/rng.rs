// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use mt19937::MT19937;
use rand::{thread_rng, Rng, SeedableRng};

pub fn create_rng() -> MT19937 {
    let mut seed = mt19937::Seed::default();
    thread_rng().fill(&mut seed.0[..]);
    MT19937::from_seed(seed)
}
