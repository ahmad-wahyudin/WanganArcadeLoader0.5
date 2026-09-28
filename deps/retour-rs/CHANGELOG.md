# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.0-alpha.2 (2023-11-15)

### New Features

 - <csr-id-faaeec330e42fe3c34b4e3abcaf24d12fdf0884c/> add trailing comma support

### Bug Fixes

 - <csr-id-3bf7863bc0ad4ba1d0657e6ee98d43145c16b658/> remove feature requirement for thiscall
   * fix: remove feature requirement for thiscall
   abi_thiscall is now stabilized rust and no longer requires nightly rust
   https://github.com/rust-lang/rust/commit/06daa9e263db87b3c5d4d80110938130db846183
   
   * fix: remove feature attribute definition
   
   * Keep thiscall-abi feature gate for back compat
   
   * Update docs for required thiscall-abi version
   
   ---------

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 31 calendar days.
 - 63 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#34](https://github.com/Hpmason/retour-rs/issues/34), [#45](https://github.com/Hpmason/retour-rs/issues/45)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#34](https://github.com/Hpmason/retour-rs/issues/34)**
    - Remove feature requirement for thiscall ([`3bf7863`](https://github.com/Hpmason/retour-rs/commit/3bf7863bc0ad4ba1d0657e6ee98d43145c16b658))
 * **[#45](https://github.com/Hpmason/retour-rs/issues/45)**
    - Add trailing comma support ([`faaeec3`](https://github.com/Hpmason/retour-rs/commit/faaeec330e42fe3c34b4e3abcaf24d12fdf0884c))
 * **Uncategorized**
    - Bump version ([`4e0d383`](https://github.com/Hpmason/retour-rs/commit/4e0d383eea2c25d93d537d3e89db5ba5e38f8f57))
</details>

## 0.3.1 (2023-07-18)

## 0.8.0 (2021-05-10)

### Chore

 - <csr-id-07b346570c69736a57a25212d7121309711ee50b/> update version 0.7.1 → 0.8.0
 - <csr-id-5808bed377134c7f5d7f7f45ea10154d6b06dab5/> add linux musl & i686-gnu targets
 - <csr-id-6af8b287e7215d5226eede7cb623e9841afcc550/> add custom macOS linker for RWX execution
 - <csr-id-aeadc0032eb1631bedd40476b6211fd41207375a/> add CI for rust stable
 - <csr-id-1aa9cc8b795abcb4b0028ff2ed5cef29e8c21a17/> use latest macOS iamge (10.15)
 - <csr-id-f792981ebb34228d34b444e5a4d736d18aa0fdf5/> disable 'i686-apple-darwin' CI
   Since macOS x86 is now a tier 3 target, the advantages of supporting
   this target has greatly diminished. For now, it's CI suite will be
   disabled.
 - <csr-id-b840d9eef1260959764f00b7161fa8aa33a3c095/> update 'cfg-if' dependency
 - <csr-id-d651621f2c2767f0dd52690298bbe9453f4ac8e5/> use shield badges for Azure
 - <csr-id-28c0fdeff412f04f4dc398bd9c1ecab98f7dbc29/> simplify target configuration
 - <csr-id-3f7f55b189144086d95fc72de4aa3347e7aa6e3b/> update dependencies

### Bug Fixes

 - <csr-id-5bdab402e69b43bebf4ff0c793ce06845b1359f0/> replace deprecated features
 - <csr-id-6c1d92cd50bc082ca4607c6983c598206ddc1b63/> remove deprecated '--all' flag
 - <csr-id-ee9f8205150d0f9a82b591e810a6c249f0b1bf6d/> 32-bit range calculation overflow
 - <csr-id-f2c3f4c5f4bbcb9b2c49a03f9843e06b6d55766a/> replace obsolete 'kernel32-sys' dependency
 - <csr-id-e6fa25f6425429f59dd9b07a0c2c4ff6423c855a/> use new 'Protection' constants
 - <csr-id-eb49d95122a08ca44e54d8b59811789a3d872a7c/> replace deprecated 'asm!'

### Style

 - <csr-id-9c7dcb0cbbd98c39f195b18e968fc34258d6fbfe/> apply rustfmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 346 calendar days.
 - 624 days passed between releases.
 - 17 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version 0.7.1 → 0.8.0 ([`07b3465`](https://github.com/Hpmason/retour-rs/commit/07b346570c69736a57a25212d7121309711ee50b))
    - Apply rustfmt ([`9c7dcb0`](https://github.com/Hpmason/retour-rs/commit/9c7dcb0cbbd98c39f195b18e968fc34258d6fbfe))
    - Add unstable feature use of `const_fn_trait_bound` ([`44f2111`](https://github.com/Hpmason/retour-rs/commit/44f2111ab533c7aa750ef7800a1904a8581907dd))
    - Replace deprecated features ([`5bdab40`](https://github.com/Hpmason/retour-rs/commit/5bdab402e69b43bebf4ff0c793ce06845b1359f0))
    - Remove deprecated '--all' flag ([`6c1d92c`](https://github.com/Hpmason/retour-rs/commit/6c1d92cd50bc082ca4607c6983c598206ddc1b63))
    - Add linux musl & i686-gnu targets ([`5808bed`](https://github.com/Hpmason/retour-rs/commit/5808bed377134c7f5d7f7f45ea10154d6b06dab5))
    - Add custom macOS linker for RWX execution ([`6af8b28`](https://github.com/Hpmason/retour-rs/commit/6af8b287e7215d5226eede7cb623e9841afcc550))
    - Add CI for rust stable ([`aeadc00`](https://github.com/Hpmason/retour-rs/commit/aeadc0032eb1631bedd40476b6211fd41207375a))
    - Use latest macOS iamge (10.15) ([`1aa9cc8`](https://github.com/Hpmason/retour-rs/commit/1aa9cc8b795abcb4b0028ff2ed5cef29e8c21a17))
    - Disable 'i686-apple-darwin' CI ([`f792981`](https://github.com/Hpmason/retour-rs/commit/f792981ebb34228d34b444e5a4d736d18aa0fdf5))
    - 32-bit range calculation overflow ([`ee9f820`](https://github.com/Hpmason/retour-rs/commit/ee9f8205150d0f9a82b591e810a6c249f0b1bf6d))
    - Update 'cfg-if' dependency ([`b840d9e`](https://github.com/Hpmason/retour-rs/commit/b840d9eef1260959764f00b7161fa8aa33a3c095))
    - Replace obsolete 'kernel32-sys' dependency ([`f2c3f4c`](https://github.com/Hpmason/retour-rs/commit/f2c3f4c5f4bbcb9b2c49a03f9843e06b6d55766a))
    - Use shield badges for Azure ([`d651621`](https://github.com/Hpmason/retour-rs/commit/d651621f2c2767f0dd52690298bbe9453f4ac8e5))
    - Simplify target configuration ([`28c0fde`](https://github.com/Hpmason/retour-rs/commit/28c0fdeff412f04f4dc398bd9c1ecab98f7dbc29))
    - Update dependencies ([`3f7f55b`](https://github.com/Hpmason/retour-rs/commit/3f7f55b189144086d95fc72de4aa3347e7aa6e3b))
    - Use new 'Protection' constants ([`e6fa25f`](https://github.com/Hpmason/retour-rs/commit/e6fa25f6425429f59dd9b07a0c2c4ff6423c855a))
    - Replace deprecated 'asm!' ([`eb49d95`](https://github.com/Hpmason/retour-rs/commit/eb49d95122a08ca44e54d8b59811789a3d872a7c))
</details>

## 0.7.1 (2019-08-25)

### Chore

 - <csr-id-23c7fb478c8312da6609ec5b81a83b214a5db8d7/> update version 0.7.0 → 0.7.1
 - <csr-id-c41834bca87889591093a987683e0768f343c6fd/> use azure for all CI
 - <csr-id-4de8f01736147cd28bc0ce4c69a397773975aa99/> add missing 32-bit libs
 - <csr-id-53997fa75416c06cc5baebde59cc046da0d84f51/> add azure pipelines configuration

### New Features

 - <csr-id-09cda49357b3c0a8b19b888415b6ea971c63bd24/> add support for scoped visibility
 - <csr-id-03ca7e47a777bbbca3c1272e30b34757051d50ed/> test latest udis86
 - <csr-id-49cfaf330bdfc0e50aca7f22aafad68eab101f9a/> update dependencies

### Bug Fixes

 - <csr-id-e2c8866990d1e665608c16b4ec2576773efd0400/> use the latest published libudis86
 - <csr-id-e658cfcdcc00df31d9052dff2d8cd32ad4ff900b/> disable GNU targets for azure
 - <csr-id-a099038a15c356a48c0352395e7ef46e77b7650e/> explicitly drop boxed data
 - <csr-id-a12175c7bf25a29367df210330229e22f7d1c444/> use atomic for detour enabled state

### Refactor

 - <csr-id-b8170fff2ae6f3f5e5b3c66d5ea010e70feecbf2/> use inner functions for tests
 - <csr-id-e5124e13a79de218b6c2320d935da135f09da3ed/> prevent 'FreeRegionIter' from being exposed

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 76 calendar days.
 - 76 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version 0.7.0 → 0.7.1 ([`23c7fb4`](https://github.com/Hpmason/retour-rs/commit/23c7fb478c8312da6609ec5b81a83b214a5db8d7))
    - Use the latest published libudis86 ([`e2c8866`](https://github.com/Hpmason/retour-rs/commit/e2c8866990d1e665608c16b4ec2576773efd0400))
    - Use azure for all CI ([`c41834b`](https://github.com/Hpmason/retour-rs/commit/c41834bca87889591093a987683e0768f343c6fd))
    - Disable GNU targets for azure ([`e658cfc`](https://github.com/Hpmason/retour-rs/commit/e658cfcdcc00df31d9052dff2d8cd32ad4ff900b))
    - Explicitly drop boxed data ([`a099038`](https://github.com/Hpmason/retour-rs/commit/a099038a15c356a48c0352395e7ef46e77b7650e))
    - Add support for scoped visibility ([`09cda49`](https://github.com/Hpmason/retour-rs/commit/09cda49357b3c0a8b19b888415b6ea971c63bd24))
    - Test latest udis86 ([`03ca7e4`](https://github.com/Hpmason/retour-rs/commit/03ca7e47a777bbbca3c1272e30b34757051d50ed))
    - Add missing 32-bit libs ([`4de8f01`](https://github.com/Hpmason/retour-rs/commit/4de8f01736147cd28bc0ce4c69a397773975aa99))
    - Add azure pipelines configuration ([`53997fa`](https://github.com/Hpmason/retour-rs/commit/53997fa75416c06cc5baebde59cc046da0d84f51))
    - Use inner functions for tests ([`b8170ff`](https://github.com/Hpmason/retour-rs/commit/b8170fff2ae6f3f5e5b3c66d5ea010e70feecbf2))
    - Update dependencies ([`49cfaf3`](https://github.com/Hpmason/retour-rs/commit/49cfaf330bdfc0e50aca7f22aafad68eab101f9a))
    - Use atomic for detour enabled state ([`a12175c`](https://github.com/Hpmason/retour-rs/commit/a12175c7bf25a29367df210330229e22f7d1c444))
    - Prevent 'FreeRegionIter' from being exposed ([`e5124e1`](https://github.com/Hpmason/retour-rs/commit/e5124e13a79de218b6c2320d935da135f09da3ed))
</details>

## 0.7.0 (2019-06-10)

### Chore

 - <csr-id-7ff857d5d8cf1e02d11552df03070ef130589ebf/> update version 6.0 → 7.0

### Documentation

 - <csr-id-81ee8436255e339e136a1a1562762e5ab9687996/> add 'MessageBoxW' detour example
 - <csr-id-1ba909e81d01c6b080f7e25b11885c9fbefb45a0/> update comments
 - <csr-id-6b7b2f726f637239a025ecffe6ebb7d76d6e32c7/> update macro/detour comments

### New Features

 - <csr-id-10ce4225359f8031eafbd9166297cdee3ccd3f6b/> return '&self' from 'initialize'
 - <csr-id-6682e7725f2ae7b4af0f1b3712e588e97c047753/> migrate to '2018' edition
 - <csr-id-4ce411265609aad929e5ccaaf21bc0d8b5743827/> implement std 'Error' trait

### Bug Fixes

 - <csr-id-03b8db911b1028c04d686c076532624e3e0df119/> add missing 'dyn' specifiers
 - <csr-id-5e85585a9522d47ec874f5281fc8b2ec20262fd4/> temporarily remove 'musl' CI tests
   Cargo does not enable OS/cfg specific examples, so the 'cdylib' tests
   errors when using 'musl' due to not supporting the crate type.
   Temporarily remove these targets until a workaround can be implemented.
 - <csr-id-7d799b203cb56b955a7fad21413ea64ce08e35aa/> use 'dyn' for dynamic dispatches
 - <csr-id-a4a4172a1c979c7295e861bb0a8c0175baf8bc9b/> remove 'const_fn' thanks to 2018 macros
 - <csr-id-1ac74031a6b85e1d5533434b6407a7dbb0aa035f/> remove 'ref' prefix
 - <csr-id-eeb82cae86ede737431d36fdcd6c8a6eaa381621/> iterator for descending regions

### Refactor

 - <csr-id-073e0e740941cc76cd0279697bfa53ae0d7bdc93/> use stabilized range 'contains'
 - <csr-id-693abce46d859e8adfb287542531fa081fe029b1/> use 'expect' prior to 'unwrap'
 - <csr-id-80cd96f83f3936a77a411b24b8a08d8c8623b9a8/> remove 'tap' dependency
 - <csr-id-dd8dbd04a180c90a2861c9eea3b15d6b412695b4/> remove 'Boolinator' dependency
 - <csr-id-9a9c14348a5692ea241d50ac6c069f8a67c3f041/> simplify 'StaticDetour' API
 - <csr-id-24f2ce11dbc65e4807f969cbd0c889531d89e067/> use 'static ref' instead of 'struct'

### Style

 - <csr-id-ab9f22aab30f35b49f77b84ff0d43c48757e5f53/> apply rustfmt
 - <csr-id-46cda1077ad0f97886a421bc1e8483000b5c0e78/> normalize macro indentation
 - <csr-id-7d80778ebd38af65d5abf7a27983fe796d9430fa/> normalize indentation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 156 calendar days.
 - 157 days passed between releases.
 - 22 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version 6.0 → 7.0 ([`7ff857d`](https://github.com/Hpmason/retour-rs/commit/7ff857d5d8cf1e02d11552df03070ef130589ebf))
    - Add missing 'dyn' specifiers ([`03b8db9`](https://github.com/Hpmason/retour-rs/commit/03b8db911b1028c04d686c076532624e3e0df119))
    - Temporarily remove 'musl' CI tests ([`5e85585`](https://github.com/Hpmason/retour-rs/commit/5e85585a9522d47ec874f5281fc8b2ec20262fd4))
    - Add 'MessageBoxW' detour example ([`81ee843`](https://github.com/Hpmason/retour-rs/commit/81ee8436255e339e136a1a1562762e5ab9687996))
    - Use 'dyn' for dynamic dispatches ([`7d799b2`](https://github.com/Hpmason/retour-rs/commit/7d799b203cb56b955a7fad21413ea64ce08e35aa))
    - Return '&self' from 'initialize' ([`10ce422`](https://github.com/Hpmason/retour-rs/commit/10ce4225359f8031eafbd9166297cdee3ccd3f6b))
    - Update comments ([`1ba909e`](https://github.com/Hpmason/retour-rs/commit/1ba909e81d01c6b080f7e25b11885c9fbefb45a0))
    - Use stabilized range 'contains' ([`073e0e7`](https://github.com/Hpmason/retour-rs/commit/073e0e740941cc76cd0279697bfa53ae0d7bdc93))
    - Remove 'const_fn' thanks to 2018 macros ([`a4a4172`](https://github.com/Hpmason/retour-rs/commit/a4a4172a1c979c7295e861bb0a8c0175baf8bc9b))
    - Apply rustfmt ([`ab9f22a`](https://github.com/Hpmason/retour-rs/commit/ab9f22aab30f35b49f77b84ff0d43c48757e5f53))
    - Migrate to '2018' edition ([`6682e77`](https://github.com/Hpmason/retour-rs/commit/6682e7725f2ae7b4af0f1b3712e588e97c047753))
    - Use 'expect' prior to 'unwrap' ([`693abce`](https://github.com/Hpmason/retour-rs/commit/693abce46d859e8adfb287542531fa081fe029b1))
    - Remove 'ref' prefix ([`1ac7403`](https://github.com/Hpmason/retour-rs/commit/1ac74031a6b85e1d5533434b6407a7dbb0aa035f))
    - Update macro/detour comments ([`6b7b2f7`](https://github.com/Hpmason/retour-rs/commit/6b7b2f726f637239a025ecffe6ebb7d76d6e32c7))
    - Remove 'tap' dependency ([`80cd96f`](https://github.com/Hpmason/retour-rs/commit/80cd96f83f3936a77a411b24b8a08d8c8623b9a8))
    - Remove 'Boolinator' dependency ([`dd8dbd0`](https://github.com/Hpmason/retour-rs/commit/dd8dbd04a180c90a2861c9eea3b15d6b412695b4))
    - Simplify 'StaticDetour' API ([`9a9c143`](https://github.com/Hpmason/retour-rs/commit/9a9c14348a5692ea241d50ac6c069f8a67c3f041))
    - Use 'static ref' instead of 'struct' ([`24f2ce1`](https://github.com/Hpmason/retour-rs/commit/24f2ce11dbc65e4807f969cbd0c889531d89e067))
    - Normalize macro indentation ([`46cda10`](https://github.com/Hpmason/retour-rs/commit/46cda1077ad0f97886a421bc1e8483000b5c0e78))
    - Implement std 'Error' trait ([`4ce4112`](https://github.com/Hpmason/retour-rs/commit/4ce411265609aad929e5ccaaf21bc0d8b5743827))
    - Iterator for descending regions ([`eeb82ca`](https://github.com/Hpmason/retour-rs/commit/eeb82cae86ede737431d36fdcd6c8a6eaa381621))
    - Normalize indentation ([`7d80778`](https://github.com/Hpmason/retour-rs/commit/7d80778ebd38af65d5abf7a27983fe796d9430fa))
</details>

## 0.6.0 (2019-01-03)

### Chore

 - <csr-id-bfa4c4100ffd21d0b276ace23e11b88616fe1f57/> update version 0.5 → 0.6
   This version greatly decreases the compile time and total number of
   required dependencies.
 - <csr-id-4dc337ddc2d505adae55d0a666ab77b122f608b9/> update dependencies
 - <csr-id-5cd38b50f05827ca42990717ac34eade8c3457ee/> add windows (GNU) to travis config

### Bug Fixes

 - <csr-id-c54bb3c136013610e1271b37bea2588a3be7b622/> remove 'failure' as dependency

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 81 calendar days.
 - 234 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version 0.5 → 0.6 ([`bfa4c41`](https://github.com/Hpmason/retour-rs/commit/bfa4c4100ffd21d0b276ace23e11b88616fe1f57))
    - Update dependencies ([`4dc337d`](https://github.com/Hpmason/retour-rs/commit/4dc337ddc2d505adae55d0a666ab77b122f608b9))
    - Remove 'failure' as dependency ([`c54bb3c`](https://github.com/Hpmason/retour-rs/commit/c54bb3c136013610e1271b37bea2588a3be7b622))
    - Add windows (GNU) to travis config ([`5cd38b5`](https://github.com/Hpmason/retour-rs/commit/5cd38b50f05827ca42990717ac34eade8c3457ee))
    - Update static_detours to support "thiscall" convention ([`19c88cd`](https://github.com/Hpmason/retour-rs/commit/19c88cd02b766d7bc3ad70e7f72f7c13e0c4de23))
    - Bump generic-array and tap versions ([`7af703d`](https://github.com/Hpmason/retour-rs/commit/7af703dab98f0cc63be0b10531ebda29ce62d00f))
</details>

## 0.5.0 (2018-05-14)

### Chore

 - <csr-id-70931eec858b83ff41df79a7f63e1bc496364575/> update editorconfig; matches rustfmt.toml

### New Features

 - <csr-id-8b88bd213eb24e9d26a564df156db19d0fa9f605/> update version → 0.5

### Bug Fixes

 - <csr-id-25f5cc1bf60c3ceb1e9d28fd8c5f2b584f4d2646/> apply clippy recommendations
 - <csr-id-ecb9d87d20ed7c69fa22972e96aef4a51aa21cb5/> use local Error type instead of failure

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 49 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version → 0.5 ([`8b88bd2`](https://github.com/Hpmason/retour-rs/commit/8b88bd213eb24e9d26a564df156db19d0fa9f605))
    - Apply clippy recommendations ([`25f5cc1`](https://github.com/Hpmason/retour-rs/commit/25f5cc1bf60c3ceb1e9d28fd8c5f2b584f4d2646))
    - Update editorconfig; matches rustfmt.toml ([`70931ee`](https://github.com/Hpmason/retour-rs/commit/70931eec858b83ff41df79a7f63e1bc496364575))
    - Use local Error type instead of failure ([`ecb9d87`](https://github.com/Hpmason/retour-rs/commit/ecb9d87d20ed7c69fa22972e96aef4a51aa21cb5))
</details>

## 0.4.1 (2018-03-26)

### Documentation

 - <csr-id-31bc37a3e5278191f306162f5451769e1804f76b/> add distinction between travis/appveyor

### New Features

 - <csr-id-9e7f222d00f3aa178910a53ec2bf948fc49869fc/> update version → 0.4.1
 - <csr-id-a0419ffd3a2a16aa19c85cc6b994f2e25a95b140/> add clippy config
 - <csr-id-f3d0c05a91f7b35817849b00f730e103d1c3ae1d/> add rustfmt config

### Bug Fixes

 - <csr-id-87fec81e73ee7fc2d7367216b9be596487139d55/> scoping issue with 1.26 nightly

### Refactor

 - <csr-id-f7d34c80ed909bd833cf3aeec62bf3a7c0e85512/> use doc for attribute test

### Style

 - <csr-id-927ab0a87528b4fc381c71dd8b587b8ebda726b9/> format project using rustfmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 20 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version → 0.4.1 ([`9e7f222`](https://github.com/Hpmason/retour-rs/commit/9e7f222d00f3aa178910a53ec2bf948fc49869fc))
    - Add clippy config ([`a0419ff`](https://github.com/Hpmason/retour-rs/commit/a0419ffd3a2a16aa19c85cc6b994f2e25a95b140))
    - Format project using rustfmt ([`927ab0a`](https://github.com/Hpmason/retour-rs/commit/927ab0a87528b4fc381c71dd8b587b8ebda726b9))
    - Add rustfmt config ([`f3d0c05`](https://github.com/Hpmason/retour-rs/commit/f3d0c05a91f7b35817849b00f730e103d1c3ae1d))
    - Use doc for attribute test ([`f7d34c8`](https://github.com/Hpmason/retour-rs/commit/f7d34c80ed909bd833cf3aeec62bf3a7c0e85512))
    - Scoping issue with 1.26 nightly ([`87fec81`](https://github.com/Hpmason/retour-rs/commit/87fec81e73ee7fc2d7367216b9be596487139d55))
    - Add distinction between travis/appveyor ([`31bc37a`](https://github.com/Hpmason/retour-rs/commit/31bc37a3e5278191f306162f5451769e1804f76b))
    - Merge branch 'kgv-master' ([`48de87a`](https://github.com/Hpmason/retour-rs/commit/48de87acb00bd4787f3a94c846fd8622ede95452))
    - Merge branch 'master' into master ([`f1a7055`](https://github.com/Hpmason/retour-rs/commit/f1a70554fa72f8ff35ef441a965e5b4012141b3f))
</details>

## 0.4.0 (2018-03-05)

### New Features

 - <csr-id-192ce59c1b1031b0046d3329a00f100f7c3d2f06/> update version → 0.4
 - <csr-id-785e3440be6292c1d134032c370d53f8ba23bf78/> update dependencies
 - <csr-id-254a29598cf2888b52deb407bbb1876d13e8e32f/> improve architecture abstraction.
   This change includes a ton of refactoring that should have been split
   into multiple atomic commits. The architecture specific code has a much
   improved abstraction whilst the nightly feature is no longer required.

### Bug Fixes

 - <csr-id-5d708e960da12239f66a618e127c3e234acec04c/> use wrapping_add in handle_relative_branch
   * displacement can be negative in some cases
 - <csr-id-8eedeba920eebc771e832ba9d10643ff78210d02/> static_detours macro parse meta attributes #4
 - <csr-id-d11d7299a44b196c5353fdbdd8302d5e7b3fb503/> generate 'call' method for unsafe types

### Refactor

 - <csr-id-873270f654469e8c9dbe801b2795745261d072ab/> StaticThunk → FixedThunk
 - <csr-id-7d20fc09c11a91e50b042dcd44db2c4d0ebdd676/> Closure → Vec for x64 thunks

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 108 calendar days.
 - 151 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version → 0.4 ([`192ce59`](https://github.com/Hpmason/retour-rs/commit/192ce59c1b1031b0046d3329a00f100f7c3d2f06))
    - Update dependencies ([`785e344`](https://github.com/Hpmason/retour-rs/commit/785e3440be6292c1d134032c370d53f8ba23bf78))
    - StaticThunk → FixedThunk ([`873270f`](https://github.com/Hpmason/retour-rs/commit/873270f654469e8c9dbe801b2795745261d072ab))
    - Improve architecture abstraction. ([`254a295`](https://github.com/Hpmason/retour-rs/commit/254a29598cf2888b52deb407bbb1876d13e8e32f))
    - Closure → Vec for x64 thunks ([`7d20fc0`](https://github.com/Hpmason/retour-rs/commit/7d20fc09c11a91e50b042dcd44db2c4d0ebdd676))
    - Use wrapping_add in handle_relative_branch ([`5d708e9`](https://github.com/Hpmason/retour-rs/commit/5d708e960da12239f66a618e127c3e234acec04c))
    - Static_detours macro parse meta attributes #4 ([`8eedeba`](https://github.com/Hpmason/retour-rs/commit/8eedeba920eebc771e832ba9d10643ff78210d02))
    - Generate 'call' method for unsafe types ([`d11d729`](https://github.com/Hpmason/retour-rs/commit/d11d7299a44b196c5353fdbdd8302d5e7b3fb503))
</details>

## 0.4.0-alpha.1 (2023-09-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 98 calendar days.
 - 98 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#24](https://github.com/Hpmason/retour-rs/issues/24), [#26](https://github.com/Hpmason/retour-rs/issues/26), [#30](https://github.com/Hpmason/retour-rs/issues/30), [#31](https://github.com/Hpmason/retour-rs/issues/31), [#38](https://github.com/Hpmason/retour-rs/issues/38), [#39](https://github.com/Hpmason/retour-rs/issues/39)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#24](https://github.com/Hpmason/retour-rs/issues/24)**
    - Improve README.md ([`8a8ae9e`](https://github.com/Hpmason/retour-rs/commit/8a8ae9e8963cc77eb580dc1b5624f2242a9f8d1c))
 * **[#26](https://github.com/Hpmason/retour-rs/issues/26)**
    - COM example: DXGI swapchain Present hook ([`8dca3f2`](https://github.com/Hpmason/retour-rs/commit/8dca3f2aa0ff3e4da486a6099e2c61e2c1c47c1b))
 * **[#30](https://github.com/Hpmason/retour-rs/issues/30)**
    - Use slice-pool2 ([`629399a`](https://github.com/Hpmason/retour-rs/commit/629399a0e853adf774ba116e0a1d9f0af941ede9))
 * **[#31](https://github.com/Hpmason/retour-rs/issues/31)**
    - Fix import static detour ([`ad33b38`](https://github.com/Hpmason/retour-rs/commit/ad33b38a40f4fa9cf5562f60b3b28126b5547823))
 * **[#38](https://github.com/Hpmason/retour-rs/issues/38)**
    - Document Supported Versions ([`07085b0`](https://github.com/Hpmason/retour-rs/commit/07085b083cd2895227d3e55bb30884e34a49fb29))
 * **[#39](https://github.com/Hpmason/retour-rs/issues/39)**
    - Expose trampoline() on Generic and Static Detour ([`5e63f92`](https://github.com/Hpmason/retour-rs/commit/5e63f92932652d98e987f0bef172987e703b4e63))
 * **Uncategorized**
    - Pre-publish for v0.4.0-alpha.1 ([`0e86431`](https://github.com/Hpmason/retour-rs/commit/0e86431d05feff70c6455676a1c0e3492f7bd2af))
    - Fix nightly-check.yml ([`a58667e`](https://github.com/Hpmason/retour-rs/commit/a58667e830eed4a6f7ae5fea8d0359b6e749dd82))
    - Fix nightly-check.yml ([`e078224`](https://github.com/Hpmason/retour-rs/commit/e07822436029505ecbfa4bd02eb707fd6b993bee))
    - Fix nightly-check.yml ([`e116704`](https://github.com/Hpmason/retour-rs/commit/e116704b8d4934d0c6fbeabec9726d41a1ae2795))
    - Adjusting changelogs prior to release of retour v0.3.1 ([`f65a8be`](https://github.com/Hpmason/retour-rs/commit/f65a8be0c639d7fd302b8b4a42ce175db78f4933))
    - Add changelog ([`c977a43`](https://github.com/Hpmason/retour-rs/commit/c977a438da96485c51f236fb370fa19a6f67bb95))
</details>

## v0.3.0 (2017-10-05)

<csr-id-e180448c80855dc533edcbc62390be2e3106294b/>



### Bug Fixes

 - <csr-id-958166199c544d28b70344e6abd261270ac0e8df/> use features for latest nightly

### New Features

 - <csr-id-4e3676f1722eb67c6a90788ea6cc3f3e23d99d42/> update version 0.2 → 0.3

### Chore

 - <csr-id-e180448c80855dc533edcbc62390be2e3106294b/> Release retour version 0.3.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 21 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version 0.2 → 0.3 ([`4e3676f`](https://github.com/Hpmason/retour-rs/commit/4e3676f1722eb67c6a90788ea6cc3f3e23d99d42))
    - Use features for latest nightly ([`9581661`](https://github.com/Hpmason/retour-rs/commit/958166199c544d28b70344e6abd261270ac0e8df))
</details>

## v0.2.0 (2017-09-14)

<csr-id-19db76b1f3c6a1889d8e5bb781ee920512f06250/>
<csr-id-0bc9dcc8672899ddfde083100f4707603d29000c/>
<csr-id-2963f60705e87edda310f547cd4f3d742b3432fd/>


<csr-id-0bc9dcc8672899ddfde083100f4707603d29000c/>
<csr-id-2963f60705e87edda310f547cd4f3d742b3432fd/>

### New Features

 - <csr-id-7c7a75fc84d04a2aeb60acfcfd64dd8966f27ffd/> update version 0.1.1 → 0.2
 - <csr-id-ff71a86a6c4b5a4093ca053f391b813a119e3f4f/> update dependency versions.
   Also remove backtrace from `error-chain` for XP support.

### Chore

 - <csr-id-19db76b1f3c6a1889d8e5bb781ee920512f06250/> Release retour version 0.2.0
 - <csr-id-0bc9dcc8672899ddfde083100f4707603d29000c/> squelch various lints

### Bug Fixes

 - <csr-id-9109c7cba74772c8eb255e4625cea519f66191bd/> GenericDetour doesn't accept unsafe fns

### Other

 - <csr-id-2963f60705e87edda310f547cd4f3d742b3432fd/> update ctor

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 17 calendar days.
 - 83 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version 0.1.1 → 0.2 ([`7c7a75f`](https://github.com/Hpmason/retour-rs/commit/7c7a75fc84d04a2aeb60acfcfd64dd8966f27ffd))
    - Update dependency versions. ([`ff71a86`](https://github.com/Hpmason/retour-rs/commit/ff71a86a6c4b5a4093ca053f391b813a119e3f4f))
    - Merge pull request #2 from nick70/master ([`c585dfd`](https://github.com/Hpmason/retour-rs/commit/c585dfd6cb1724c75b11b88c557e83724a183351))
    - Fix spelling mistake. ([`c3c9ee2`](https://github.com/Hpmason/retour-rs/commit/c3c9ee2075fda4169dd5a680ffceeb1d355fcd33))
</details>

## 0.1.1 (2017-06-22)

### Documentation

 - <csr-id-5f0583b679eeefa213482ed229a6e3ae9b8739e2/> add badges to Cargo

### Bug Fixes

 - <csr-id-ff374474f362e4b915aea02a04716c23e922af1c/> warning on 32-bit
 - <csr-id-46ed445083097be6277805222c1f4d6593e1f940/> use volatile cell to prevent inlining

### Refactor

 - <csr-id-df1764b6f5efda3a25b71935bd929e0b2e79dfba/> variant → detour, x86 → arch/x86

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add badges to Cargo ([`5f0583b`](https://github.com/Hpmason/retour-rs/commit/5f0583b679eeefa213482ed229a6e3ae9b8739e2))
    - Warning on 32-bit ([`ff37447`](https://github.com/Hpmason/retour-rs/commit/ff374474f362e4b915aea02a04716c23e922af1c))
    - Use volatile cell to prevent inlining ([`46ed445`](https://github.com/Hpmason/retour-rs/commit/46ed445083097be6277805222c1f4d6593e1f940))
    - Variant → detour, x86 → arch/x86 ([`df1764b`](https://github.com/Hpmason/retour-rs/commit/df1764b6f5efda3a25b71935bd929e0b2e79dfba))
</details>

## v0.1.0 (2017-06-17)

<csr-id-07b346570c69736a57a25212d7121309711ee50b/>
<csr-id-5808bed377134c7f5d7f7f45ea10154d6b06dab5/>
<csr-id-6af8b287e7215d5226eede7cb623e9841afcc550/>
<csr-id-aeadc0032eb1631bedd40476b6211fd41207375a/>
<csr-id-1aa9cc8b795abcb4b0028ff2ed5cef29e8c21a17/>
<csr-id-f792981ebb34228d34b444e5a4d736d18aa0fdf5/>
<csr-id-b840d9eef1260959764f00b7161fa8aa33a3c095/>
<csr-id-d651621f2c2767f0dd52690298bbe9453f4ac8e5/>
<csr-id-28c0fdeff412f04f4dc398bd9c1ecab98f7dbc29/>
<csr-id-3f7f55b189144086d95fc72de4aa3347e7aa6e3b/>
<csr-id-23c7fb478c8312da6609ec5b81a83b214a5db8d7/>
<csr-id-c41834bca87889591093a987683e0768f343c6fd/>
<csr-id-4de8f01736147cd28bc0ce4c69a397773975aa99/>
<csr-id-53997fa75416c06cc5baebde59cc046da0d84f51/>
<csr-id-7ff857d5d8cf1e02d11552df03070ef130589ebf/>
<csr-id-bfa4c4100ffd21d0b276ace23e11b88616fe1f57/>
<csr-id-4dc337ddc2d505adae55d0a666ab77b122f608b9/>
<csr-id-5cd38b50f05827ca42990717ac34eade8c3457ee/>
<csr-id-70931eec858b83ff41df79a7f63e1bc496364575/>
<csr-id-f9a24d61af4313f5003def3661da8798f02f1666/>
<csr-id-b8170fff2ae6f3f5e5b3c66d5ea010e70feecbf2/>
<csr-id-e5124e13a79de218b6c2320d935da135f09da3ed/>
<csr-id-073e0e740941cc76cd0279697bfa53ae0d7bdc93/>
<csr-id-693abce46d859e8adfb287542531fa081fe029b1/>
<csr-id-80cd96f83f3936a77a411b24b8a08d8c8623b9a8/>
<csr-id-dd8dbd04a180c90a2861c9eea3b15d6b412695b4/>
<csr-id-9a9c14348a5692ea241d50ac6c069f8a67c3f041/>
<csr-id-24f2ce11dbc65e4807f969cbd0c889531d89e067/>
<csr-id-f7d34c80ed909bd833cf3aeec62bf3a7c0e85512/>
<csr-id-873270f654469e8c9dbe801b2795745261d072ab/>
<csr-id-7d20fc09c11a91e50b042dcd44db2c4d0ebdd676/>
<csr-id-df1764b6f5efda3a25b71935bd929e0b2e79dfba/>
<csr-id-ae44d79f9179cb858272024ea2d19f929e1920f7/>
<csr-id-9c7dcb0cbbd98c39f195b18e968fc34258d6fbfe/>
<csr-id-ab9f22aab30f35b49f77b84ff0d43c48757e5f53/>
<csr-id-46cda1077ad0f97886a421bc1e8483000b5c0e78/>
<csr-id-7d80778ebd38af65d5abf7a27983fe796d9430fa/>
<csr-id-927ab0a87528b4fc381c71dd8b587b8ebda726b9/>


<csr-id-5808bed377134c7f5d7f7f45ea10154d6b06dab5/>
<csr-id-6af8b287e7215d5226eede7cb623e9841afcc550/>
<csr-id-aeadc0032eb1631bedd40476b6211fd41207375a/>
<csr-id-1aa9cc8b795abcb4b0028ff2ed5cef29e8c21a17/>
<csr-id-f792981ebb34228d34b444e5a4d736d18aa0fdf5/>
<csr-id-b840d9eef1260959764f00b7161fa8aa33a3c095/>
<csr-id-d651621f2c2767f0dd52690298bbe9453f4ac8e5/>
<csr-id-28c0fdeff412f04f4dc398bd9c1ecab98f7dbc29/>
<csr-id-3f7f55b189144086d95fc72de4aa3347e7aa6e3b/>
<csr-id-23c7fb478c8312da6609ec5b81a83b214a5db8d7/>
<csr-id-c41834bca87889591093a987683e0768f343c6fd/>
<csr-id-4de8f01736147cd28bc0ce4c69a397773975aa99/>
<csr-id-53997fa75416c06cc5baebde59cc046da0d84f51/>
<csr-id-7ff857d5d8cf1e02d11552df03070ef130589ebf/>
<csr-id-bfa4c4100ffd21d0b276ace23e11b88616fe1f57/>
<csr-id-4dc337ddc2d505adae55d0a666ab77b122f608b9/>
<csr-id-5cd38b50f05827ca42990717ac34eade8c3457ee/>
<csr-id-70931eec858b83ff41df79a7f63e1bc496364575/>
<csr-id-f9a24d61af4313f5003def3661da8798f02f1666/>
<csr-id-b8170fff2ae6f3f5e5b3c66d5ea010e70feecbf2/>
<csr-id-e5124e13a79de218b6c2320d935da135f09da3ed/>
<csr-id-073e0e740941cc76cd0279697bfa53ae0d7bdc93/>
<csr-id-693abce46d859e8adfb287542531fa081fe029b1/>
<csr-id-80cd96f83f3936a77a411b24b8a08d8c8623b9a8/>
<csr-id-dd8dbd04a180c90a2861c9eea3b15d6b412695b4/>
<csr-id-9a9c14348a5692ea241d50ac6c069f8a67c3f041/>
<csr-id-24f2ce11dbc65e4807f969cbd0c889531d89e067/>
<csr-id-f7d34c80ed909bd833cf3aeec62bf3a7c0e85512/>
<csr-id-873270f654469e8c9dbe801b2795745261d072ab/>
<csr-id-7d20fc09c11a91e50b042dcd44db2c4d0ebdd676/>
<csr-id-df1764b6f5efda3a25b71935bd929e0b2e79dfba/>
<csr-id-ae44d79f9179cb858272024ea2d19f929e1920f7/>
<csr-id-9c7dcb0cbbd98c39f195b18e968fc34258d6fbfe/>
<csr-id-ab9f22aab30f35b49f77b84ff0d43c48757e5f53/>
<csr-id-46cda1077ad0f97886a421bc1e8483000b5c0e78/>
<csr-id-7d80778ebd38af65d5abf7a27983fe796d9430fa/>
<csr-id-927ab0a87528b4fc381c71dd8b587b8ebda726b9/>

### Chore

 - <csr-id-07b346570c69736a57a25212d7121309711ee50b/> update version 0.7.1 → 0.8.0
 - <csr-id-5808bed377134c7f5d7f7f45ea10154d6b06dab5/> add linux musl & i686-gnu targets
 - <csr-id-6af8b287e7215d5226eede7cb623e9841afcc550/> add custom macOS linker for RWX execution
 - <csr-id-aeadc0032eb1631bedd40476b6211fd41207375a/> add CI for rust stable
 - <csr-id-1aa9cc8b795abcb4b0028ff2ed5cef29e8c21a17/> use latest macOS iamge (10.15)
 - <csr-id-f792981ebb34228d34b444e5a4d736d18aa0fdf5/> disable 'i686-apple-darwin' CI
   Since macOS x86 is now a tier 3 target, the advantages of supporting
   this target has greatly diminished. For now, it's CI suite will be
   disabled.
 - <csr-id-b840d9eef1260959764f00b7161fa8aa33a3c095/> update 'cfg-if' dependency
 - <csr-id-d651621f2c2767f0dd52690298bbe9453f4ac8e5/> use shield badges for Azure
 - <csr-id-28c0fdeff412f04f4dc398bd9c1ecab98f7dbc29/> simplify target configuration
 - <csr-id-3f7f55b189144086d95fc72de4aa3347e7aa6e3b/> update dependencies
 - <csr-id-23c7fb478c8312da6609ec5b81a83b214a5db8d7/> update version 0.7.0 → 0.7.1
 - <csr-id-c41834bca87889591093a987683e0768f343c6fd/> use azure for all CI
 - <csr-id-4de8f01736147cd28bc0ce4c69a397773975aa99/> add missing 32-bit libs
 - <csr-id-53997fa75416c06cc5baebde59cc046da0d84f51/> add azure pipelines configuration
 - <csr-id-7ff857d5d8cf1e02d11552df03070ef130589ebf/> update version 6.0 → 7.0
 - <csr-id-bfa4c4100ffd21d0b276ace23e11b88616fe1f57/> update version 0.5 → 0.6
   This version greatly decreases the compile time and total number of
   required dependencies.
 - <csr-id-4dc337ddc2d505adae55d0a666ab77b122f608b9/> update dependencies
 - <csr-id-5cd38b50f05827ca42990717ac34eade8c3457ee/> add windows (GNU) to travis config
 - <csr-id-70931eec858b83ff41df79a7f63e1bc496364575/> update editorconfig; matches rustfmt.toml
 - <csr-id-f9a24d61af4313f5003def3661da8798f02f1666/> add continous integration

### Documentation

 - <csr-id-0a329fcb1f6686ef327b552dccd8093a4f020ea1/> stylize caption
 - <csr-id-81ee8436255e339e136a1a1562762e5ab9687996/> add 'MessageBoxW' detour example
 - <csr-id-1ba909e81d01c6b080f7e25b11885c9fbefb45a0/> update comments
 - <csr-id-6b7b2f726f637239a025ecffe6ebb7d76d6e32c7/> update macro/detour comments
 - <csr-id-31bc37a3e5278191f306162f5451769e1804f76b/> add distinction between travis/appveyor
 - <csr-id-5f0583b679eeefa213482ed229a6e3ae9b8739e2/> add badges to Cargo
 - <csr-id-39d9112dfa2066fdc3e99f644ff8cd1dfa355cc9/> mention type of detour
 - <csr-id-f9df60ab26746621987779c4dafd51a6b55c7302/> add special mention
 - <csr-id-0afcba9f8efcd4288573dce359ae5a90967acbb8/> fix styling
 - <csr-id-e932129eb635b357d99a39e82835d085677205fe/> fix appendix anchor
 - <csr-id-7ad791721535e64acb37cffaf6ccedc2e9db3b3d/> add readme appendix

### New Features

 - <csr-id-09cda49357b3c0a8b19b888415b6ea971c63bd24/> add support for scoped visibility
 - <csr-id-03ca7e47a777bbbca3c1272e30b34757051d50ed/> test latest udis86
 - <csr-id-49cfaf330bdfc0e50aca7f22aafad68eab101f9a/> update dependencies
 - <csr-id-10ce4225359f8031eafbd9166297cdee3ccd3f6b/> return '&self' from 'initialize'
 - <csr-id-6682e7725f2ae7b4af0f1b3712e588e97c047753/> migrate to '2018' edition
 - <csr-id-4ce411265609aad929e5ccaaf21bc0d8b5743827/> implement std 'Error' trait
 - <csr-id-8b88bd213eb24e9d26a564df156db19d0fa9f605/> update version → 0.5
 - <csr-id-9e7f222d00f3aa178910a53ec2bf948fc49869fc/> update version → 0.4.1
 - <csr-id-a0419ffd3a2a16aa19c85cc6b994f2e25a95b140/> add clippy config
 - <csr-id-f3d0c05a91f7b35817849b00f730e103d1c3ae1d/> add rustfmt config
 - <csr-id-192ce59c1b1031b0046d3329a00f100f7c3d2f06/> update version → 0.4
 - <csr-id-785e3440be6292c1d134032c370d53f8ba23bf78/> update dependencies
 - <csr-id-254a29598cf2888b52deb407bbb1876d13e8e32f/> improve architecture abstraction.
   This change includes a ton of refactoring that should have been split
   into multiple atomic commits. The architecture specific code has a much
   improved abstraction whilst the nightly feature is no longer required.
 - <csr-id-4e3676f1722eb67c6a90788ea6cc3f3e23d99d42/> update version 0.2 → 0.3
 - <csr-id-7c7a75fc84d04a2aeb60acfcfd64dd8966f27ffd/> update version 0.1.1 → 0.2
 - <csr-id-ff71a86a6c4b5a4093ca053f391b813a119e3f4f/> update dependency versions.
   Also remove backtrace from `error-chain` for XP support.
 - <csr-id-772e7851f337624d674a1e35f0c905ad9177593c/> implement proper static detours
 - <csr-id-6c3aa72493810974c0b19bf82b3d8f0981979d2e/> split InlineDetour → RawDetour/GenericDetour
 - <csr-id-5d43691730cbba33c1b21e1120d6022260d2e68b/> implement static detours
 - <csr-id-c51d3f3c0d6ce6ef45b87c7971c7210eb49e9fab/> update slice-pool → 0.3.4
 - <csr-id-38948894dd8028c253441212b0dd762927d48698/> update dependency versions
 - <csr-id-d95ce516d9bde060704bd98d4bc280c6f8555ec7/> add editorconfig
 - <csr-id-c2f9fd41450d101499a1ab0f705bcc707c0e2995/> refactor file organisation
 - <csr-id-63bc6907b5003de237a86934f18c441763aa3875/> localize dependencies
 - <csr-id-68ead08200da0c72c222ea307f4b670e4d2f0f90/> add test for address range
 - <csr-id-adc50ed0752d9947a64eea763713445bb9480ae0/> only implement inline detour
 - <csr-id-d3713e0fd1b54ca43fe3a3db0a59c899b0321671/> implemented memory pool
 - <csr-id-0b5fd5fdc47313ca92828a0fa1a13a13d6956127/> add mutex for unique access
 - <csr-id-668c73c2ff32e3184ed18aefe967122a399e08a0/> initial commit

### Bug Fixes

<csr-id-8eedeba920eebc771e832ba9d10643ff78210d02/>
<csr-id-d11d7299a44b196c5353fdbdd8302d5e7b3fb503/>
<csr-id-958166199c544d28b70344e6abd261270ac0e8df/>
<csr-id-ff374474f362e4b915aea02a04716c23e922af1c/>
<csr-id-46ed445083097be6277805222c1f4d6593e1f940/>
<csr-id-0c7e273b18fec764c8c1bfaa30d0e35fcc643b77/>
<csr-id-a0b23c2a68f6f5dc4d617f8262962a0521218bd2/>
<csr-id-4224d1a9e8e2f9d454a9cf65cc98a3c80260b65c/>
<csr-id-ff4e54958e0c92ea1720e7f0dd2516068a87b486/>

 - <csr-id-5bdab402e69b43bebf4ff0c793ce06845b1359f0/> replace deprecated features
 - <csr-id-6c1d92cd50bc082ca4607c6983c598206ddc1b63/> remove deprecated '--all' flag
 - <csr-id-ee9f8205150d0f9a82b591e810a6c249f0b1bf6d/> 32-bit range calculation overflow
 - <csr-id-f2c3f4c5f4bbcb9b2c49a03f9843e06b6d55766a/> replace obsolete 'kernel32-sys' dependency
 - <csr-id-e6fa25f6425429f59dd9b07a0c2c4ff6423c855a/> use new 'Protection' constants
 - <csr-id-eb49d95122a08ca44e54d8b59811789a3d872a7c/> replace deprecated 'asm!'
 - <csr-id-e2c8866990d1e665608c16b4ec2576773efd0400/> use the latest published libudis86
 - <csr-id-e658cfcdcc00df31d9052dff2d8cd32ad4ff900b/> disable GNU targets for azure
 - <csr-id-a099038a15c356a48c0352395e7ef46e77b7650e/> explicitly drop boxed data
 - <csr-id-a12175c7bf25a29367df210330229e22f7d1c444/> use atomic for detour enabled state
 - <csr-id-03b8db911b1028c04d686c076532624e3e0df119/> add missing 'dyn' specifiers
 - <csr-id-5e85585a9522d47ec874f5281fc8b2ec20262fd4/> temporarily remove 'musl' CI tests
   Cargo does not enable OS/cfg specific examples, so the 'cdylib' tests
   errors when using 'musl' due to not supporting the crate type.
   Temporarily remove these targets until a workaround can be implemented.
 - <csr-id-7d799b203cb56b955a7fad21413ea64ce08e35aa/> use 'dyn' for dynamic dispatches
 - <csr-id-a4a4172a1c979c7295e861bb0a8c0175baf8bc9b/> remove 'const_fn' thanks to 2018 macros
 - <csr-id-1ac74031a6b85e1d5533434b6407a7dbb0aa035f/> remove 'ref' prefix
 - <csr-id-eeb82cae86ede737431d36fdcd6c8a6eaa381621/> iterator for descending regions
 - <csr-id-c54bb3c136013610e1271b37bea2588a3be7b622/> remove 'failure' as dependency
 - <csr-id-25f5cc1bf60c3ceb1e9d28fd8c5f2b584f4d2646/> apply clippy recommendations
 - <csr-id-ecb9d87d20ed7c69fa22972e96aef4a51aa21cb5/> use local Error type instead of failure
 - <csr-id-87fec81e73ee7fc2d7367216b9be596487139d55/> scoping issue with 1.26 nightly
 - <csr-id-5d708e960da12239f66a618e127c3e234acec04c/> use wrapping_add in handle_relative_branch
   * displacement can be negative in some cases

### Refactor

 - <csr-id-b8170fff2ae6f3f5e5b3c66d5ea010e70feecbf2/> use inner functions for tests
 - <csr-id-e5124e13a79de218b6c2320d935da135f09da3ed/> prevent 'FreeRegionIter' from being exposed
 - <csr-id-073e0e740941cc76cd0279697bfa53ae0d7bdc93/> use stabilized range 'contains'
 - <csr-id-693abce46d859e8adfb287542531fa081fe029b1/> use 'expect' prior to 'unwrap'
 - <csr-id-80cd96f83f3936a77a411b24b8a08d8c8623b9a8/> remove 'tap' dependency
 - <csr-id-dd8dbd04a180c90a2861c9eea3b15d6b412695b4/> remove 'Boolinator' dependency
 - <csr-id-9a9c14348a5692ea241d50ac6c069f8a67c3f041/> simplify 'StaticDetour' API
 - <csr-id-24f2ce11dbc65e4807f969cbd0c889531d89e067/> use 'static ref' instead of 'struct'
 - <csr-id-f7d34c80ed909bd833cf3aeec62bf3a7c0e85512/> use doc for attribute test
 - <csr-id-873270f654469e8c9dbe801b2795745261d072ab/> StaticThunk → FixedThunk
 - <csr-id-7d20fc09c11a91e50b042dcd44db2c4d0ebdd676/> Closure → Vec for x64 thunks
 - <csr-id-df1764b6f5efda3a25b71935bd929e0b2e79dfba/> variant → detour, x86 → arch/x86
 - <csr-id-ae44d79f9179cb858272024ea2d19f929e1920f7/> general improvements

### Style

 - <csr-id-9c7dcb0cbbd98c39f195b18e968fc34258d6fbfe/> apply rustfmt
 - <csr-id-ab9f22aab30f35b49f77b84ff0d43c48757e5f53/> apply rustfmt
 - <csr-id-46cda1077ad0f97886a421bc1e8483000b5c0e78/> normalize macro indentation
 - <csr-id-7d80778ebd38af65d5abf7a27983fe796d9430fa/> normalize indentation
 - <csr-id-927ab0a87528b4fc381c71dd8b587b8ebda726b9/> format project using rustfmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 195 calendar days.
 - 24 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Mention type of detour ([`39d9112`](https://github.com/Hpmason/retour-rs/commit/39d9112dfa2066fdc3e99f644ff8cd1dfa355cc9))
    - Add special mention ([`f9df60a`](https://github.com/Hpmason/retour-rs/commit/f9df60ab26746621987779c4dafd51a6b55c7302))
    - Use mmap-fixed from crates.io ([`0c7e273`](https://github.com/Hpmason/retour-rs/commit/0c7e273b18fec764c8c1bfaa30d0e35fcc643b77))
    - Detour local functions ([`a0b23c2`](https://github.com/Hpmason/retour-rs/commit/a0b23c2a68f6f5dc4d617f8262962a0521218bd2))
    - Implement proper static detours ([`772e785`](https://github.com/Hpmason/retour-rs/commit/772e7851f337624d674a1e35f0c905ad9177593c))
    - Split InlineDetour → RawDetour/GenericDetour ([`6c3aa72`](https://github.com/Hpmason/retour-rs/commit/6c3aa72493810974c0b19bf82b3d8f0981979d2e))
    - Implement static detours ([`5d43691`](https://github.com/Hpmason/retour-rs/commit/5d43691730cbba33c1b21e1120d6022260d2e68b))
    - General improvements ([`ae44d79`](https://github.com/Hpmason/retour-rs/commit/ae44d79f9179cb858272024ea2d19f929e1920f7))
    - Update slice-pool → 0.3.4 ([`c51d3f3`](https://github.com/Hpmason/retour-rs/commit/c51d3f3c0d6ce6ef45b87c7971c7210eb49e9fab))
    - Add unreachable message ([`4224d1a`](https://github.com/Hpmason/retour-rs/commit/4224d1a9e8e2f9d454a9cf65cc98a3c80260b65c))
    - Tests in release mode ([`ff4e549`](https://github.com/Hpmason/retour-rs/commit/ff4e54958e0c92ea1720e7f0dd2516068a87b486))
    - Add continous integration ([`f9a24d6`](https://github.com/Hpmason/retour-rs/commit/f9a24d61af4313f5003def3661da8798f02f1666))
    - Fix styling ([`0afcba9`](https://github.com/Hpmason/retour-rs/commit/0afcba9f8efcd4288573dce359ae5a90967acbb8))
    - Fix appendix anchor ([`e932129`](https://github.com/Hpmason/retour-rs/commit/e932129eb635b357d99a39e82835d085677205fe))
    - Update dependency versions ([`3894889`](https://github.com/Hpmason/retour-rs/commit/38948894dd8028c253441212b0dd762927d48698))
    - Add readme appendix ([`7ad7917`](https://github.com/Hpmason/retour-rs/commit/7ad791721535e64acb37cffaf6ccedc2e9db3b3d))
    - Add editorconfig ([`d95ce51`](https://github.com/Hpmason/retour-rs/commit/d95ce516d9bde060704bd98d4bc280c6f8555ec7))
    - Refactor file organisation ([`c2f9fd4`](https://github.com/Hpmason/retour-rs/commit/c2f9fd41450d101499a1ab0f705bcc707c0e2995))
    - Localize dependencies ([`63bc690`](https://github.com/Hpmason/retour-rs/commit/63bc6907b5003de237a86934f18c441763aa3875))
    - Add test for address range ([`68ead08`](https://github.com/Hpmason/retour-rs/commit/68ead08200da0c72c222ea307f4b670e4d2f0f90))
    - Only implement inline detour ([`adc50ed`](https://github.com/Hpmason/retour-rs/commit/adc50ed0752d9947a64eea763713445bb9480ae0))
    - Implemented memory pool ([`d3713e0`](https://github.com/Hpmason/retour-rs/commit/d3713e0fd1b54ca43fe3a3db0a59c899b0321671))
    - Add mutex for unique access ([`0b5fd5f`](https://github.com/Hpmason/retour-rs/commit/0b5fd5fdc47313ca92828a0fa1a13a13d6956127))
    - Initial commit ([`668c73c`](https://github.com/Hpmason/retour-rs/commit/668c73c2ff32e3184ed18aefe967122a399e08a0))
</details>

<csr-unknown>









<csr-unknown/>

