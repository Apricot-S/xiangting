# xiangting

[![Crate](https://img.shields.io/crates/v/xiangting.svg)](https://crates.io/crates/xiangting)
[![Minimum Supported Rust Version](https://img.shields.io/crates/msrv/xiangting)](https://crates.io/crates/xiangting)
[![API](https://img.shields.io/badge/api-main-yellow.svg)](https://apricot-s.github.io/xiangting/xiangting)
[![API](https://docs.rs/xiangting/badge.svg)](https://docs.rs/xiangting)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/Apricot-S/xiangting)

A library for calculating the deficiency number (a.k.a. xiàngtīng number, 向聴数).

Documentation:

- [API reference (main branch)](https://Apricot-S.github.io/xiangting/xiangting)
- [API reference (docs.rs)](https://docs.rs/xiangting)

## References

- [[麻雀]シャンテン数計算アルゴリズム #C++ - Qiita](https://qiita.com/tomohxx/items/75b5f771285e1334c0a5)
- [5. 集合漸化式 - 麻雀アルゴリズム](https://tomohxx.github.io/mahjong-algorithm-book/srf/)
- [【図解】向聴数計算アルゴリズム - 麻雀アルゴリズム](https://tomohxx.github.io/mahjong-algorithm-book/illustration/)
- [Theoretical Background of Nyanten (Efficient Computation of Shanten/Deficiency Numbers) #麻雀 - Qiita](https://qiita.com/Cryolite/items/75d504c7489426806b87)
- [A Fast and Space-Efficient Algorithm for Calculating Deficient Numbers (a.k.a. Shanten Numbers).pdf](https://www.slideshare.net/slideshow/a-fast-and-space-efficient-algorithm-for-calculating-deficient-numbers-a-k-a-shanten-numbers-pdf/269706674)

## Language Bindings

- Python: [xiangting-py](https://github.com/Apricot-S/xiangting-py)

## Installation

```sh
cargo add xiangting
```

## Usage

### Basic Usage

The hand is represented as an array of `[u8; 34]`, where each element represents the count of a specific tile.
The correspondence between the index and the tile is shown in the table below.

| Index | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   |
| ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile  | 1m  | 2m  | 3m  | 4m  | 5m  | 6m  | 7m  | 8m  | 9m  |

| Index | 9   | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  |
| ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile  | 1p  | 2p  | 3p  | 4p  | 5p  | 6p  | 7p  | 8p  | 9p  |

| Index | 18  | 19  | 20  | 21  | 22  | 23  | 24  | 25  | 26  |
| ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile  | 1s  | 2s  | 3s  | 4s  | 5s  | 6s  | 7s  | 8s  | 9s  |

| Index | 27        | 28         | 29        | 30         | 31         | 32         | 33       |
| ----- | --------- | ---------- | --------- | ---------- | ---------- | ---------- | -------- |
| Tile  | East (1z) | South (2z) | West (3z) | North (4z) | White (5z) | Green (6z) | Red (7z) |

Calculates the replacement number, which is equal to the deficiency number (a.k.a. xiàngtīng number, 向聴数) + 1.

```rust
use xiangting::{PlayerCount, calculate_replacement_number};

fn main() {
    // 123m456p789s11222z
    let hand: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 1, 1, 1, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 1, 1, 1, // s
        2, 3, 0, 0, 0, 0, 0, // z
    ];

    let replacement_number = calculate_replacement_number(&hand, &PlayerCount::Four).unwrap();
    assert_eq!(replacement_number, 0u8);
}
```

### Necessary and Unnecessary Tiles

It is also possible to calculate necessary or unnecessary tiles together with the replacement number.

- Necessary tiles
  - Tiles needed to win with the minimum number of replacements
  - Tiles that reduce the replacement number when drawn
  - In Japanese, these are referred to as *有効牌 (yūkōhai)* or *受け入れ (ukeire)*

- Unnecessary tiles
  - Tiles not needed to win with the minimum number of replacements
  - Tiles that can be discarded without changing the replacement number
  - In Japanese, these are referred to as *不要牌 (fuyōhai)* or *余剰牌 (yojōhai)*

```rust
use xiangting::{PlayerCount, calculate_necessary_tiles, calculate_unnecessary_tiles};

fn main() {
    // 199m146779p12s246z
    let hand: [u8; 34] = [
        1, 0, 0, 0, 0, 0, 0, 0, 2, // m
        1, 0, 0, 1, 0, 1, 2, 0, 1, // p
        1, 1, 0, 0, 0, 0, 0, 0, 0, // s
        0, 1, 0, 1, 0, 1, 0, // z
    ];

    let (replacement_number1, necessary_tiles) =
        calculate_necessary_tiles(&hand, &PlayerCount::Four).unwrap();
    let (replacement_number2, unnecessary_tiles) =
        calculate_unnecessary_tiles(&hand, &PlayerCount::Four).unwrap();

    assert_eq!(replacement_number1, 5);
    assert_eq!(replacement_number1, replacement_number2);
    assert_eq!(necessary_tiles, 0b1111111_100000111_111111111_100000111); // 1239m123456789p1239s1234567z
    assert_eq!(unnecessary_tiles, 0b0101010_000000011_101101001_000000001); // 1m14679p12s246z
}
```

### Support for Three-Player Mahjong

In three-player mahjong, the tiles from 2m (二萬) to 8m (八萬) are not used.

```rust
use xiangting::{PlayerCount, calculate_necessary_tiles, calculate_unnecessary_tiles};

fn main() {
    // 1111m111122233z
    let hand: [u8; 34] = [
        4, 0, 0, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        4, 3, 2, 0, 0, 0, 0, // z
    ];

    let (rn_4p, nt_4p) = calculate_necessary_tiles(&hand, &PlayerCount::Four).unwrap();
    let (_, ut_4p) = calculate_unnecessary_tiles(&hand, &PlayerCount::Four).unwrap();
    assert_eq!(rn_4p, 2u8);
    assert_eq!(nt_4p, 0b0000000_000000000_000000000_000000110); // 23m
    assert_eq!(ut_4p, 0b0000001_000000000_000000000_000000000); // 1z

    let (rn_3p, nt_3p) = calculate_necessary_tiles(&hand, &PlayerCount::Three).unwrap();
    let (_, ut_3p) = calculate_unnecessary_tiles(&hand, &PlayerCount::Three).unwrap();
    assert_eq!(rn_3p, 3u8);
    assert_eq!(nt_3p, 0b1111100_111111111_111111111_100000000); // 9m123456789p123456789s34567z
    assert_eq!(ut_3p, 0b0000001_000000000_000000000_000000001); // 1m1z
}
```

## Build tables and maps (For developers only)

```sh
xiangting$ scripts/build_table.sh && scripts/build_map.sh
```

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT license](LICENSE).
