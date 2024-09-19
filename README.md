# xiangting

A library for calculation of deficiency number (a.k.a. xiangting number, 向聴数).

This is a port of [majiang-core/lib/xiangting.js](https://github.com/kobalab/majiang-core/blob/master/lib/xiangting.js) to Rust.  
However, the following differences apply:

- Calculate the replacement number, which is equal to the deficiency number (a.k.a. xiangting number, 向聴数) plus one.
- The original algorithm miscalculated the deficiency number in some cases. These errors have been corrected in this port, but the calculation speed has deteriorated as a result.
- It supports three-player mahjong.
- It does not support short hand or long hand.

## Usage

The hand is represented by the number of each tile in an array of `[u8; 34]`. The correspondence between the index and the tile is shown in the table below.

| Index | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   |
| ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile  | 1m  | 2m  | 3m  | 4m  | 5m  | 6m  | 7m  | 8m  | 9m  |

| Index | 9   | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  |
| ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile  | 1p  | 2p  | 3p  | 4p  | 5p  | 6p  | 7p  | 8p  | 9p  |

| Index | 18  | 19  | 20  | 21  | 22  | 23  | 24  | 25  | 26  |
| ----- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile  | 1s  | 2s  | 3s  | 4s  | 5s  | 6s  | 7s  | 8s  | 9s  |

| Index | 27   | 28    | 29   | 30    | 31    | 32    | 33  |
| ----- | ---- | ----- | ---- | ----- | ----- | ----- | --- |
| Tile  | East | South | West | North | White | Green | Red |

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

In the calculation for a hand with melds (副露), the meld tiles can be included or excluded in the counting of the tiles that exist in four copies within the hand.

If they are not included (e.g., 天鳳 (Tenhou), 雀魂 (Mahjong Soul)), `&None` should be specified for `fulu_mianzi`.

If they are included (e.g., World Riichi Championship, M.LEAGUE), the melds should be specified for `fulu_mianzi`.

```rust
use xiangting::{calculate_replacement_number, ClaimedTilePosition, Mianzi};

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
        Some(Mianzi::Shunzi(12, ClaimedTilePosition::Low)),
        Some(Mianzi::Gangzi(24)),
        Some(Mianzi::Kezi(27)),
        None,
    ];

    let replacement_number_wo_melds = calculate_replacement_number(&hand_4, &None);
    assert_eq!(replacement_number_wo_melds.unwrap(), 1u8);

    let replacement_number_w_melds = calculate_replacement_number(&hand_4, &Some(melds));
    assert_eq!(replacement_number_w_melds.unwrap(), 2u8);
}
```

In the case of three-player mahjong, the tiles from 2m to 8m do not exist.

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

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT license](LICENSE).
