// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "build-map")]
#[path = "build_map/main.rs"]
mod build_map;
mod build_table;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some((command, command_args)) = args[1..].split_first() else {
        eprintln!("Usage: cargo xtask <build-table|build-map> <OUTPUT PATH>...");
        process::exit(1);
    };

    match command.as_str() {
        "build-table" => build_table::run(command_args),
        "build-map" => {
            #[cfg(feature = "build-map")]
            build_map::run(command_args);

            #[cfg(not(feature = "build-map"))]
            {
                let _ = command_args;
                eprintln!("build-map requires the `build-map` xtask feature");
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown xtask command: {command}");
            process::exit(1);
        }
    }
}
