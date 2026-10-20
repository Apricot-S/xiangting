# Standard-hand forward/backward calculation

The public APIs are unchanged: replacement number alone, replacement number
with necessary tiles, and replacement number with unnecessary tiles. Seven
pairs, thirteen orphans, and the public minimum/tie-merging logic are unchanged.

## Representation and composition order

Each suit hash selects a `u8` pattern ID instead of a packed `u32` distance
vector. The corresponding dictionary preserves all ten distances, indexed by
`melds + 5 * pair`. Patterns are interned by exact equality, in lexicographic
order; IDs do not encode tile masks.

Composition proceeds **honors -> sou -> pin -> man**. There are 55 honor,
126 numbered-suit, and 10 three-player man patterns. The intermediate stages
have 165 and 180 patterns. Only the final lookup differs between player rules.

The final lookup stores five replacement numbers directly, one per required
meld count. An ID encoding was also measured. Direct values remove a dependent
dictionary lookup and performed better in most mask benchmarks in this local
experiment. They use 97,920 additional bytes of composition tables compared
with IDs, while remaining within the distribution budget. Only the selected
encoding is generated and shipped.

Composition tables occupy 150,120 bytes; the three leaf ID maps occupy 448,505
bytes; leaf and intermediate cost dictionaries occupy 5,360 bytes. Existing
necessary/unnecessary mask tables occupy 10,764,120 bytes and are unchanged.
These are array payload sizes, excluding hash tables, alignment, and code.

## Backward correctness

For a suit hand H and target T, the distance is the sum of `max(T[t]-H[t], 0)`.
The existing generated masks union shortages and surpluses over every target
attaining the minimum for the specified meld/pair count.

After fixing the meld/pair split, suit target sets form a Cartesian product
and their distances add. Every part of an optimal complete target must thus
be a local minimizer. Backward visits every split whose two costs sum to the
parent minimum. Its `u16` state sets retain all tied predecessors, and leaf
masks are still selected using the original hand hashes.

Projecting optimal splits onto separate child state sets is sufficient because
the output is a union of tile flags. It would not retain enough information to
enumerate complete winning hands or correlate necessary and unnecessary tiles.
State zero is retained: its distance is zero, but its unnecessary mask can be
nonempty. The existing value 14 is preserved verbatim; it must not universally
be interpreted as infinity because it can also be a finite local distance.

Runtime calculation uses fixed-size stack values and static arrays. No heap
allocation, unsafe code, runtime decompression, build script, or download is
required. The library remains `no_std`, with Rust 1.85 as its MSRV.

## Validation

- Existing unit and documentation tests.
- Every entry in the four composition tables is checked against an independent
  Cartesian-product enumeration of costs and all optimal predecessor states.
  Single-state, multiple-state, empty, and zero-state propagation are covered.
- 10,000 deterministic sampled hands cover both rules, all ten valid lengths,
  ordinary wall draws, and full-flush hands. All three public APIs are compared
  with the independent `shanten-dp` solver. Unnecessary tiles are checked by
  removing each present tile and comparing scalar distances while keeping the
  required meld count fixed. This avoids the external solver's thirteen-orphans
  discard-mask omission of simple tiles.
- During migration, all 448,505 leaf entries were compared against their old
  packed values, and both mask maps were checked unchanged.
- Full-hand exhaustive correctness tests are deliberately not run; the existing
  ignored tests remain available for separate long-running verification.

## Local performance experiment

`cargo bench --bench algorithm` pre-generates 100,000 hands per case with seed
42. Input generation is not timed. Each API is warmed up, then the median of
seven batches of 500,000 calls is reported. The original executable was saved
before replacing the algorithm, and executables were run sequentially without
concurrent builds for comparison. Both public input checking and special forms
are included. Mixed cases cycle through all ten legal hand lengths.

Environment: Windows x86-64, Intel Family 6 Model 140, Rust 1.99.0, default Cargo
bench profile. Results below are one sequential pass, in ns/hand, and are
exploratory measurements rather than confidence intervals or portable claims.

| Case | API | Original | Final ID | Direct final values |
|---|---|---:|---:|---:|
| 4p mixed | number | 85.46 | 39.83 | 37.95 |
| 4p mixed | necessary | 239.49 | 145.26 | 154.59 |
| 4p mixed | unnecessary | 267.63 | 149.86 | 151.62 |
| 4p 14 | number | 88.86 | 50.72 | 47.64 |
| 4p 14 | necessary | 437.18 | 331.13 | 281.21 |
| 4p 14 | unnecessary | 432.84 | 328.98 | 321.40 |
| 3p mixed | number | 70.82 | 38.38 | 38.62 |
| 3p mixed | necessary | 223.54 | 161.60 | 155.05 |
| 3p mixed | unnecessary | 229.70 | 161.40 | 153.70 |
| 3p 14 | number | 77.73 | 47.94 | 47.45 |
| 3p 14 | necessary | 421.23 | 346.52 | 334.23 |
| 3p 14 | unnecessary | 435.75 | 379.03 | 343.95 |

Some differences between final encodings are small or reverse across cases.
Frequency scaling and cache effects were not controlled with hardware counters.
The direct encoding is a local choice, not evidence that IDs are always slower.
Profile L1/L2/LLC behavior and additional workloads before extending this result
to other processors. Full generator and clean-build timings have not been
measured as comparative benchmarks.

## Generation and distribution

`scripts/build_map.sh` generates local distances/masks and then the composition
LUT. To regenerate just the composition LUT from the embedded suit dictionaries:

```sh
cargo run --release -p xtask -- build-lut \
  src/standard/shupai_map.rs src/standard/zipai_map.rs \
  src/standard/wanzi_19_map.rs src/standard/lut.rs
```

The generated Rust sources are included in the crate. CI checks that composition
regeneration makes no changes, builds the packaged crate using `cargo package`,
and checks its compressed size with `scripts/check_package_size.py`. The same
package check runs before publishing. The budget is 9,000,000 bytes, leaving
headroom below the documented crates.io 10 MB limit; array payload size is not
the distribution-size metric.

The starting compressed package was 4,444,740 bytes. Always measure the actual
new `.crate` archive, including supporting source and documentation, rather than
inferring its size from the arrays.

References: [nyanten](https://github.com/Cryolite/nyanten),
[Cryolite's algorithm article](https://qiita.com/Cryolite/items/2650ce76ffac925d7876).
