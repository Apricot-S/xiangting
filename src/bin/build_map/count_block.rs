// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod core;
mod shupai;
mod wanzi_19;
mod zipai;

pub(super) use self::shupai::count_shupai_block;
pub(super) use self::wanzi_19::count_19m_block;
pub(super) use self::zipai::count_zipai_block;
