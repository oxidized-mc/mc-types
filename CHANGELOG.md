# Changelog

## [0.3.0](https://github.com/oxidized-mc/mc-types/compare/v0.2.0...v0.3.0) (2026-04-10)


### ⚠ BREAKING CHANGES

* **mc-types:** EntityDimensions::new() is no longer const fn (computes eye_height). Use EntityDimensions::fixed() for compile-time construction.

### 🚀 Features

* add MobCategory, SoundSource, and EntitySpawnReason ([2db2e9c](https://github.com/oxidized-mc/mc-types/commit/2db2e9ccc2a30fa773cc7659a0ec5a0bad68367b))
* **mc-types:** add repository best practices (phase 06) ([5a1283b](https://github.com/oxidized-mc/mc-types/commit/5a1283b1106486e1cfb2e9f876d9ce14b1983af5))


### 🐛 Bug Fixes

* **ci:** chain publish into release-please workflow ([2920399](https://github.com/oxidized-mc/mc-types/commit/2920399ff11a2506779ef22e4865fb49aa4243da))
* **mc-types:** address final audit findings ([bb0a9cf](https://github.com/oxidized-mc/mc-types/commit/bb0a9cfc8e9781f5fce97b400e55149f4a5e3601))
* **mc-types:** correct Vec2::MIN to match Java Float.MIN_VALUE ([bfbff73](https://github.com/oxidized-mc/mc-types/commit/bfbff734d01904fb93808e7765d302806fd5f6eb))
* **mc-types:** phase 08 final audit — correctness fixes and test additions ([0adc8d6](https://github.com/oxidized-mc/mc-types/commit/0adc8d6713ad606f4135cd90dd09a7c73d72570c))
* **mc-types:** prevent panic in mth::floor/ceil on infinity inputs ([eb8cee5](https://github.com/oxidized-mc/mc-types/commit/eb8cee5d5a1b7533d4cb9f8fe76490efb3ab6673))
* **mc-types:** resolve all open tech debt (TD-007, TD-017–TD-020) ([0626ad1](https://github.com/oxidized-mc/mc-types/commit/0626ad12bcee40b64634a2084ff1c746566df3f8))
* **mc-types:** use truncating modulo in Rotations to match vanilla ([7108d63](https://github.com/oxidized-mc/mc-types/commit/7108d63b6ab7b09f4997caa685f7b72ea3db9fb5))
* **mc-types:** use vanilla ordinal order for HORIZONTALS array ([ba989a3](https://github.com/oxidized-mc/mc-types/commit/ba989a3e90aa391819e0422959241463abea3606))
* **mc-types:** use vanilla Vec3 normalize threshold 1e-4 ([6d6303a](https://github.com/oxidized-mc/mc-types/commit/6d6303abf3890ce79f07001567c1908fd1f10e78))
* **mc-types:** validate Direction VarInt range before u8 cast ([725fc2e](https://github.com/oxidized-mc/mc-types/commit/725fc2e2cca86afd0702ca41ab2de125a7752387))
* **release:** switch to rust release-type and bump version to 0.2.0 ([4dd15e6](https://github.com/oxidized-mc/mc-types/commit/4dd15e6803cb0cba1031b66413a55bb74d552ab8))


### 🔨 Refactor

* move [patch.crates-io] to meta-repo .cargo/config.toml ([7d05129](https://github.com/oxidized-mc/mc-types/commit/7d05129e312e8ceab72b341412b6023aaa4c12d7))

## [0.2.0](https://github.com/oxidized-mc/mc-types/compare/v0.1.0...v0.2.0) (2026-04-10)


### 🚀 Features

* **ci:** add dev publish workflow ([e6ca823](https://github.com/oxidized-mc/mc-types/commit/e6ca8232150b9b6235b14e678a632ce58c54e736))
* **mc-types:** add criterion benchmark suite and performance documentation ([754fbe0](https://github.com/oxidized-mc/mc-types/commit/754fbe0b7dbca6d760df982be0cdae77abec1b0e))
* **mc-types:** extract game types from oxidized-protocol ([520e4a2](https://github.com/oxidized-mc/mc-types/commit/520e4a2990432744d9985bcf9b5b5cf1cea2521e))
* **mc-types:** implement phase 02 API completeness ([d65aa53](https://github.com/oxidized-mc/mc-types/commit/d65aa53c93df9856c5ee00ce2ce4562c251afbdd))


### 🐛 Bug Fixes

* **ci:** add permissions to release-please caller ([c15a131](https://github.com/oxidized-mc/mc-types/commit/c15a1310e8ccb69172630a4b008bdb0fe72cfa65))
* **deny:** use allow-org for git source allowlist ([087d59c](https://github.com/oxidized-mc/mc-types/commit/087d59c18feb0f6545e3f1ba375b2c7899c46c2a))
* **deps:** switch from git to version deps for crates.io publishing ([d09b031](https://github.com/oxidized-mc/mc-types/commit/d09b0319c0554e7ec930089d223c5257bfac08ab))
* **mc-types:** remove [patch] section that breaks CI ([29a3cae](https://github.com/oxidized-mc/mc-types/commit/29a3cae8bfc873f5409e6514b72d12b691d30089))
* **mc-types:** remove re-exports and switch deps to crates.io versions ([6128014](https://github.com/oxidized-mc/mc-types/commit/612801491412c6377ba859d90da866737f44df09))

## Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).
