# Benchmarks — oxidized-mc-types

Performance benchmarks for all hot-path operations using
[Criterion.rs](https://github.com/bheisler/criterion.rs).

## Running Benchmarks

```bash
# Full benchmark suite
cargo bench -p oxidized-mc-types

# Single benchmark group
cargo bench -p oxidized-mc-types -- block_pos
cargo bench -p oxidized-mc-types -- vec3
cargo bench -p oxidized-mc-types -- aabb

# Single benchmark
cargo bench -p oxidized-mc-types -- "block_pos/as_long"
```

## Comparing Against a Baseline

```bash
# Save a baseline
cargo bench -p oxidized-mc-types -- --save-baseline main

# Compare against it after changes
cargo bench -p oxidized-mc-types -- --baseline main
```

HTML reports are generated in `target/criterion/` — open `report/index.html`
in a browser for interactive graphs.

## Benchmark Results

All benchmarks were run on a single core. Times shown are the median of 100
samples collected by Criterion's statistical estimator.

### Coordinate Types (integer — all stack-allocated, Copy)

| Benchmark | Median | Notes |
|-----------|--------|-------|
| `block_pos/new` | ~0.89 ns | Struct construction |
| `block_pos/as_long` | ~0.79 ns | Pack to i64 (bit shifts + OR) |
| `block_pos/from_long` | ~0.71 ns | Unpack from i64 (shifts + sign extension) |
| `block_pos/as_long_roundtrip` | ~1.06 ns | Pack + unpack |
| `block_pos/offset` | ~0.91 ns | 3× wrapping add |
| `block_pos/relative_direction` | ~1.76 ns | Direction lookup + offset |
| `block_pos/above` | ~0.88 ns | Single add (inlined from macro) |
| `block_pos/containing` | ~6.96 ns | 3× f64→i32 floor conversion |
| `block_pos/dist_sqr` | ~1.29 ns | 3× i64 multiply-accumulate |
| `block_pos/get_center` | ~1.46 ns | 3× i32→f64 conversion + 0.5 |
| `section_pos/as_long` | ~0.79 ns | Pack to i64 |
| `section_pos/from_long` | ~0.71 ns | Unpack from i64 |
| `section_pos/as_long_roundtrip` | ~1.06 ns | Pack + unpack |
| `section_pos/of_block_pos` | ~0.75 ns | 3× arithmetic right shift |
| `section_pos/block_to_section_coord` | ~0.33 ns | Single right shift |

### Vector Types (floating-point — all stack-allocated, Copy)

| Benchmark | Median | Notes |
|-----------|--------|-------|
| `vec3/normalize` | ~2.63 ns | sqrt + 3× divide (or zero check) |
| `vec3/dot` | ~1.69 ns | 3× multiply + 2× add |
| `vec3/cross` | ~2.29 ns | 6× multiply + 3× subtract |
| `vec3/length_sqr` | ~1.04 ns | 3× multiply + 2× add |
| `vec3/add_vec` | ~1.87 ns | 3× f64 add |
| `vec3/add_operator` | ~1.89 ns | Same as add_vec (operator delegates) |
| `vec3/distance_to_sqr` | ~1.85 ns | 3× subtract + length_sqr |
| `vec3/scale` | ~1.49 ns | 3× f64 multiply |
| `vec3/lerp` | ~2.29 ns | 3× fused multiply-add |

### AABB (floating-point — stack-allocated, Copy)

| Benchmark | Median | Notes |
|-----------|--------|-------|
| `aabb/intersects_hit` | ~2.14 ns | 6× f64 comparison (overlapping) |
| `aabb/intersects_miss` | ~1.59 ns | Early exit on first axis miss |
| `aabb/contains` | ~2.36 ns | 6× f64 comparison |
| `aabb/inflate` | ~2.37 ns | 6× f64 add/subtract |
| `aabb/move_by` | ~2.48 ns | 6× f64 add |
| `aabb/expand_towards` | ~2.89 ns | Conditional min/max + add |
| `aabb/from_vec3` | ~3.02 ns | 6× f64 min/max (auto-correct) |
| `aabb/get_center` | ~1.78 ns | 3× midpoint calculation |
| `aabb/distance_to_sqr` | ~2.58 ns | Per-axis clamp + length_sqr |

### ResourceLocation (allocating — owns String fields)

| Benchmark | Median | Notes |
|-----------|--------|-------|
| `resource_location/parse_namespaced` | ~51 ns | Parse + allocate namespace + path |
| `resource_location/parse_default_ns` | ~41 ns | Default "minecraft" + allocate path |
| `resource_location/parse_long_path` | ~62 ns | Longer path string allocation |

### Direction (enum — zero-sized dispatch)

| Benchmark | Median | Notes |
|-----------|--------|-------|
| `direction/step_x` | ~0.43 ns | Match on 6 variants |
| `direction/step_y` | ~0.46 ns | Match on 6 variants |
| `direction/step_z` | ~0.42 ns | Match on 6 variants |
| `direction/opposite` | ~0.44 ns | XOR-based flip |
| `direction/axis` | ~0.44 ns | Match → Axis enum |
| `direction/get_nearest` | ~2.97 ns | 6× dot product comparison |

### Vec3i (integer vector — stack-allocated, Copy)

| Benchmark | Median | Notes |
|-----------|--------|-------|
| `vec3i/offset` | ~1.49 ns | 3× wrapping add |
| `vec3i/cross` | ~1.59 ns | 6× multiply + 3× subtract |
| `vec3i/dist_sqr` | ~1.43 ns | 3× i64 multiply-accumulate |
| `vec3i/relative` | ~1.76 ns | Direction lookup + offset |
| `vec3i/add_operator` | ~1.34 ns | 3× i32 add (operator trait) |
| `vec3i/get_axis` | ~0.73 ns | Match on Axis → field select |

## Zero-Cost Abstraction Verification

All newtype wrapper operations compile to **identical assembly** as raw
primitive operations. Evidence:

### Inlining

Every hot-path method is marked `#[inline]` or `const fn`. In release builds,
LLVM inlines them completely — functions like `as_long`, `from_long`, `offset`,
`add_vec`, `normalize`, `intersects`, and `contains` do **not appear as
separate symbols** in the emitted assembly. They are fully absorbed into their
call sites.

To verify:

```bash
cargo rustc -p oxidized-mc-types --release -- --emit=asm
# Check the .s file in target/release/deps/
# Hot-path functions won't appear — they're inlined
```

### Assembly Comparison

Functions that do appear (non-inlined due to complexity) compile to pure
arithmetic:

| Function | Assembly Pattern |
|----------|-----------------|
| `Direction::get_nearest` | Pure `mulsd`/`addsd`/`ucomisd`/`maxsd` — no calls, no allocation |
| `Aabb::clip` | Pure `movupd`/`subpd`/`mulpd`/`addsd` — SIMD-style operations, no allocation |
| `BlockPos::read`/`write` | Buffer bounds check + direct `get_i64`/`put_i64` |

### Performance Confirmation

| Operation | Expected | Actual | Verdict |
|-----------|----------|--------|---------|
| `BlockPos::new(x,y,z)` | 3× mov | ~0.89 ns | ✅ Zero-cost |
| `BlockPos::offset` | 3× add | ~0.91 ns | ✅ Zero-cost |
| `Vec3 + Vec3` (operator) | 3× addsd | ~1.89 ns | ✅ Same as `add_vec` (~1.87 ns) |
| `Aabb::intersects` | 6× cmp | ~2.14 ns | ✅ Zero-cost |
| `Direction::step_x` | 1× lookup | ~0.43 ns | ✅ Zero-cost |

The operator traits (`Add`, `Sub`, `Neg`) generated by `impl_vector_ops!`
produce identical performance to calling the named methods directly, confirming
the macro abstraction has no runtime cost.

## Allocation Profile

### Stack-only (zero heap allocation)

All coordinate and vector operations are completely allocation-free:

- **BlockPos, SectionPos** — `Copy` structs of 3× `i32` (12 bytes)
- **Vec3** — `Copy` struct of 3× `f64` (24 bytes)
- **Vec3i** — `Copy` struct of 3× `i32` (12 bytes)
- **Aabb** — `Copy` struct of 6× `f64` (48 bytes)
- **Direction** — `Copy` enum (1 byte)
- **Axis, AxisDirection** — `Copy` enums (1 byte each)

None of these types contain `Box`, `Vec`, `String`, `Arc`, or any heap pointer.
All arithmetic, comparison, packing, and conversion operations operate entirely
on registers/stack.

### Allocating (by design)

- **ResourceLocation** — Owns `namespace: String` and `path: String`. Parsing
  allocates two strings (~40–60 ns). This is expected and correct — resource
  locations are typically parsed once at startup or on reload, not in
  per-tick hot paths.

## CI Integration

To run benchmarks in CI for regression detection, add a step like:

```yaml
- name: Run benchmarks
  run: cargo bench -p oxidized-mc-types -- --output-format bencher
```

For comparison-based regression detection, use
[`criterion-compare-action`](https://github.com/boa-dev/criterion-compare-action)
or save/compare baselines manually.
