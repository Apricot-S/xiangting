# xiangting

A library for calculating the deficiency number (a.k.a. xiangting number, 向聴数).

This library is based on the algorithm in [Cryolite's Nyanten](https://github.com/Cryolite/nyanten).  
However, it introduces the following additional features:

- Supports rules that include and exclude melded tiles when determining if a hand contains four identical tiles.
- Supports three-player mahjong.

Documentation:

- [API reference (main branch)](https://Apricot-S.github.io/xiangting/xiangting)
- [API reference (docs.rs)](https://docs.rs/xiangting)

Reference:

- [[麻雀]シャンテン数計算アルゴリズム #C++ - Qiita](https://qiita.com/tomohxx/items/75b5f771285e1334c0a5)
- [Theoretical Background of Nyanten (Efficient Computation of Shanten/Deficiency Numbers) #麻雀 - Qiita](https://qiita.com/Cryolite/items/75d504c7489426806b87)
- [A Fast and Space-Efficient Algorithm for Calculating Deficient Numbers (a.k.a. Shanten Numbers).pdf](https://www.slideshare.net/slideshow/a-fast-and-space-efficient-algorithm-for-calculating-deficient-numbers-a-k-a-shanten-numbers-pdf/269706674)

## Installation

```sh
cargo add xiangting
```

## Usage

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

Calculates the replacement number, which is equal to the deficiency number (a.k.a. xiangting number, 向聴数) + 1.

```rust
use xiangting::calculate_replacement_number;

fn main() {
    // 123m456p789s11222z
    let hand_14: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 1, 1, 1, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 1, 1, 1, // s
        2, 3, 0, 0, 0, 0, 0, // z
    ];

    let replacement_number = calculate_replacement_number(&hand_14, &None);
    assert_eq!(replacement_number.unwrap(), 0u8);
}
```

In the calculation for a hand with melds (副露),
the melded tiles can be included or excluded when counting tiles to determine if a hand contains four identical ones.

If melds are excluded (e.g., 天鳳 (Tenhou), 雀魂 (Mahjong Soul)), specify `None` for `fulu_mianzi_list`.

If melds are included (e.g., World Riichi Championship, M.LEAGUE), the melds should be included in the `fulu_mianzi_list`.

```rust
use xiangting::{calculate_replacement_number, ClaimedTilePosition, FuluMianzi};

fn main() {
    // 123m1z (3 melds)
    let hand_4: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        1, 0, 0, 0, 0, 0, 0, // z
    ];

    // 456p 7777s 111z
    let melds = [
        Some(FuluMianzi::Shunzi(12, ClaimedTilePosition::Low)),
        Some(FuluMianzi::Gangzi(24)),
        Some(FuluMianzi::Kezi(27)),
        None,
    ];

    let replacement_number_wo_melds = calculate_replacement_number(&hand_4, &None);
    assert_eq!(replacement_number_wo_melds.unwrap(), 1u8);

    let replacement_number_w_melds = calculate_replacement_number(&hand_4, &Some(melds));
    assert_eq!(replacement_number_w_melds.unwrap(), 2u8);
}
```

In three-player mahjong, the tiles from 2m (二萬) to 8m (八萬) are not used.
Additionally, melded sequences (明順子) are not allowed.

```rust
use xiangting::{calculate_replacement_number, calculate_replacement_number_3_player};

fn main() {
    // 1111m111122233z
    let hand_13: [u8; 34] = [
        4, 0, 0, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        4, 3, 2, 0, 0, 0, 0, // z
    ];

    let replacement_number_4p = calculate_replacement_number(&hand_13, &None);
    assert_eq!(replacement_number_4p.unwrap(), 2u8);

    let replacement_number_3p = calculate_replacement_number_3_player(&hand_13, &None);
    assert_eq!(replacement_number_3p.unwrap(), 3u8);
}
```

## Build tables and maps (For developers only)

```sh
xiangting$ scripts/build_table.sh && scripts/build_map.sh
```

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT license](LICENSE).
