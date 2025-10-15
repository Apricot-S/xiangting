// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

mod necessary_tiles;
mod replacement_number;
mod unnecessary_tiles;

pub(super) use necessary_tiles::{calculate_necessary_tiles, calculate_necessary_tiles_3_player};
pub(super) use replacement_number::calculate_replacement_number;
pub(super) use unnecessary_tiles::{
    calculate_unnecessary_tiles, calculate_unnecessary_tiles_3_player,
};
