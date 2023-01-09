# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.9.2 (2023-01-09)

### Bug Fixes

 - <csr-id-f16e0e21b8056914c410e224e47db22e3b6061e2/> 🐛 casteTags property in camel case

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - 🐛 casteTags property in camel case ([`f16e0e2`](https://github.com/nwesterhausen/dfraw_json_parser/commit/f16e0e21b8056914c410e224e47db22e3b6061e2))
</details>

## v0.9.1 (2023-01-09)

### Bug Fixes

 - <csr-id-b2781ed4df8ef99419169bc29313f309856fa824/> 📝 object property consistency changes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.9.1 ([`252113d`](https://github.com/nwesterhausen/dfraw_json_parser/commit/252113dd4c0c8d24bfbbd2b7a6be26146fa92941))
    - 📝 object property consistency changes ([`b2781ed`](https://github.com/nwesterhausen/dfraw_json_parser/commit/b2781ed4df8ef99419169bc29313f309856fa824))
</details>

## v0.9.0 (2023-01-09)

<csr-id-b5ac4440b1189f0c7ef8fdaeca97d36d334a3100/>

### Refactor

 - <csr-id-b5ac4440b1189f0c7ef8fdaeca97d36d334a3100/> 🔥 refactor typings for consistency
   Changes the json objects to use camelCase for all fields

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.9.0 ([`73a673e`](https://github.com/nwesterhausen/dfraw_json_parser/commit/73a673e1deead5d4dabbf93fb0933f5dc1ef5b0b))
    - 🔥 refactor typings for consistency ([`b5ac444`](https://github.com/nwesterhausen/dfraw_json_parser/commit/b5ac4440b1189f0c7ef8fdaeca97d36d334a3100))
</details>

## v0.8.1 (2023-01-09)

<csr-id-e87e4363aea2a138d7e1deb7c7a47f8c262ef9ff/>

### Chore

 - <csr-id-e87e4363aea2a138d7e1deb7c7a47f8c262ef9ff/> 🔊 add colored logs to cli example

### New Features

 - <csr-id-984103be271f86f8c683195f9e5463f218a1cbde/> ✨ add relative_path to parsed data

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.8.1 ([`5539af6`](https://github.com/nwesterhausen/dfraw_json_parser/commit/5539af65b57c653d4f877f1b3a712604390d1a6b))
    - ✨ add relative_path to parsed data ([`984103b`](https://github.com/nwesterhausen/dfraw_json_parser/commit/984103be271f86f8c683195f9e5463f218a1cbde))
    - 🔊 add colored logs to cli example ([`e87e436`](https://github.com/nwesterhausen/dfraw_json_parser/commit/e87e4363aea2a138d7e1deb7c7a47f8c262ef9ff))
</details>

## v0.8.0 (2023-01-09)

<csr-id-77cc03033f66655cfe58e102dbbea9500d91a98b/>
<csr-id-48d50ad26512c5a35b8c3d136a49da3b4f1cc95b/>
<csr-id-9669a1cd9d2be7518d989a26fbd4dc656edd4c9b/>

### New Features

 - <csr-id-ee95964c58bad45f0678a3c414523ff1958e4661/> ✨ add remaining inorganic tokens
 - <csr-id-1a52fb5b9e5f09cdb4bba0ebcd08a72662751d74/> ✨ add un-recognized BIOME tokens as-is
 - <csr-id-ca789a7e7bc8d5d10f1ff5bc43b0dfda96a249c9/> ✨ add inorganic object parsing
   Adds parsing support for inoragnic raws
   
   Updates the SimpleMaterial struct with a Temperatures to hold temperature information (removing "specific_heat").

### Bug Fixes

 - <csr-id-04a55b93596a794fa59f55e3146b7f3ecc9ff43a/> 🐛 handle NONE case for temperatures

### Refactor

 - <csr-id-77cc03033f66655cfe58e102dbbea9500d91a98b/> 🔥 remove unused set() on StateName
 - <csr-id-48d50ad26512c5a35b8c3d136a49da3b4f1cc95b/> 🧑‍💻 transform large reader.rs into module

### Style

 - <csr-id-9669a1cd9d2be7518d989a26fbd4dc656edd4c9b/> 🎨 format biomes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 1 day passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.8.0 ([`9ebf2df`](https://github.com/nwesterhausen/dfraw_json_parser/commit/9ebf2dfbd421d6666c71c6f4a28f6fe6f29a5b36))
    - 🔥 remove unused set() on StateName ([`77cc030`](https://github.com/nwesterhausen/dfraw_json_parser/commit/77cc03033f66655cfe58e102dbbea9500d91a98b))
    - 🎨 format biomes ([`9669a1c`](https://github.com/nwesterhausen/dfraw_json_parser/commit/9669a1cd9d2be7518d989a26fbd4dc656edd4c9b))
    - ✨ add remaining inorganic tokens ([`ee95964`](https://github.com/nwesterhausen/dfraw_json_parser/commit/ee95964c58bad45f0678a3c414523ff1958e4661))
    - ✨ add un-recognized BIOME tokens as-is ([`1a52fb5`](https://github.com/nwesterhausen/dfraw_json_parser/commit/1a52fb5b9e5f09cdb4bba0ebcd08a72662751d74))
    - 🐛 handle NONE case for temperatures ([`04a55b9`](https://github.com/nwesterhausen/dfraw_json_parser/commit/04a55b93596a794fa59f55e3146b7f3ecc9ff43a))
    - ✨ add inorganic object parsing ([`ca789a7`](https://github.com/nwesterhausen/dfraw_json_parser/commit/ca789a7e7bc8d5d10f1ff5bc43b0dfda96a249c9))
    - 🧑‍💻 transform large reader.rs into module ([`48d50ad`](https://github.com/nwesterhausen/dfraw_json_parser/commit/48d50ad26512c5a35b8c3d136a49da3b4f1cc95b))
</details>

## v0.7.7 (2023-01-08)

### New Features

 - <csr-id-3ae61273d150faeff3e407ef7d7d8ea4629ac021/> 🌟 include more plant data
   * feat:
* fix: 🐛 fix error message for bad raw input file in example
* feat: ✨ report all state colors
* chore: 🚀 update type definition for simple material

### Bug Fixes

 - <csr-id-f931dbeedff7e3a5ccfe065213b7672737b86de5/> 🐛 typescript typing cohesion
 - <csr-id-7bde574e03f659e1abff93cb6d4d1fc0dd0af34d/> 🐛 fix typings for plant

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#10](https://github.com/nwesterhausen/dfraw_json_parser/issues/10)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/nwesterhausen/dfraw_json_parser/issues/10)**
    - 🌟 include more plant data ([`3ae6127`](https://github.com/nwesterhausen/dfraw_json_parser/commit/3ae61273d150faeff3e407ef7d7d8ea4629ac021))
 * **Uncategorized**
    - Release dfraw_json_parser v0.7.7 ([`712dd58`](https://github.com/nwesterhausen/dfraw_json_parser/commit/712dd58040e86571a7e7aef9ba50a964e3038ac0))
    - 🐛 typescript typing cohesion ([`f931dbe`](https://github.com/nwesterhausen/dfraw_json_parser/commit/f931dbeedff7e3a5ccfe065213b7672737b86de5))
    - 🐛 fix typings for plant ([`7bde574`](https://github.com/nwesterhausen/dfraw_json_parser/commit/7bde574e03f659e1abff93cb6d4d1fc0dd0af34d))
</details>

## v0.7.6 (2023-01-01)

### New Features

 - <csr-id-3de87c132c9ffc1531242f03292d897a9043683f/> 🚀 add plant parsing into lib.rs
 - <csr-id-ba141563a121e2654ad1af165bef5642de5ff4ae/> 🌟 support plant raws
   * feat: 🚧 start implementing plant raws (and its requirements)
* feat: ✨ detect plant raws
* feat: ✨ parse plants simply
* feat: ✨ support plant raws

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#9](https://github.com/nwesterhausen/dfraw_json_parser/issues/9)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#9](https://github.com/nwesterhausen/dfraw_json_parser/issues/9)**
    - 🌟 support plant raws ([`ba14156`](https://github.com/nwesterhausen/dfraw_json_parser/commit/ba141563a121e2654ad1af165bef5642de5ff4ae))
 * **Uncategorized**
    - Release dfraw_json_parser v0.7.6 ([`3bfa8f9`](https://github.com/nwesterhausen/dfraw_json_parser/commit/3bfa8f9ca4b66598032f159483b576b082566d43))
    - 🚀 add plant parsing into lib.rs ([`3de87c1`](https://github.com/nwesterhausen/dfraw_json_parser/commit/3de87c132c9ffc1531242f03292d897a9043683f))
</details>

## v0.7.5 (2022-12-29)

### Bug Fixes

 - <csr-id-6c576b2a13153bbe757c5cd72f92107120f6e60a/> 🐛 use identifier if empty name

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.7.5 ([`57502cc`](https://github.com/nwesterhausen/dfraw_json_parser/commit/57502cc2047d17e52f503926523596285ebff7a5))
    - 🐛 use identifier if empty name ([`6c576b2`](https://github.com/nwesterhausen/dfraw_json_parser/commit/6c576b2a13153bbe757c5cd72f92107120f6e60a))
</details>

## v0.7.4 (2022-12-28)

### New Features

 - <csr-id-ed463c84a64cc8b774ebb576704ff20c0bec15d0/> ✨ add human-readable raw module source to raws

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.7.4 ([`2c28be1`](https://github.com/nwesterhausen/dfraw_json_parser/commit/2c28be190afab00ac02b4e4f07949252bb01b761))
    - ✨ add human-readable raw module source to raws ([`ed463c8`](https://github.com/nwesterhausen/dfraw_json_parser/commit/ed463c84a64cc8b774ebb576704ff20c0bec15d0))
</details>

## v0.7.3 (2022-12-28)

### New Features

 - <csr-id-d072b719c51dcb1b8065edd9251364aded5b79d2/> ✨ add parse_single_raw to lib and cli example

### Bug Fixes

 - <csr-id-ae012665b7097242b39e27e07741d5f284eec291/> 🐛 save all info for final raw read from file

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.7.3 ([`9800d75`](https://github.com/nwesterhausen/dfraw_json_parser/commit/9800d75e2a0d98cf86dd8ce6820ada716cd7a776))
    - 🐛 save all info for final raw read from file ([`ae01266`](https://github.com/nwesterhausen/dfraw_json_parser/commit/ae012665b7097242b39e27e07741d5f284eec291))
    - ✨ add parse_single_raw to lib and cli example ([`d072b71`](https://github.com/nwesterhausen/dfraw_json_parser/commit/d072b719c51dcb1b8065edd9251364aded5b79d2))
</details>

## v0.7.2 (2022-12-28)

<csr-id-6ed8d209706a530c35d0cd284df72d7c04d3efe6/>
<csr-id-5e6dde45b3489ad93660372f654fa637a4379d98/>
<csr-id-a6f00156cddc9b0c42b5f0a19883d5382c19d160/>
<csr-id-5a9740e01528680c3f6543673fbd722913ac57c2/>
<csr-id-7932e59c509f1da93e8f431fbbef9a164de144d1/>
<csr-id-bb5de4483e6c0b5bfb15a0a5e2defdd067bd6c9d/>
<csr-id-a8301caede5aa2090b769c05e19a699267e07b07/>
<csr-id-8d79e7bc493584248b8adbc98ae4f178f517a6a6/>
<csr-id-59e2dff2d583cd8c5c55411688a6c8f4ddcf361a/>


<csr-id-a8301caede5aa2090b769c05e19a699267e07b07/>
<csr-id-8d79e7bc493584248b8adbc98ae4f178f517a6a6/>
<csr-id-59e2dff2d583cd8c5c55411688a6c8f4ddcf361a/>


<csr-id-5e6dde45b3489ad93660372f654fa637a4379d98/>
<csr-id-a6f00156cddc9b0c42b5f0a19883d5382c19d160/>
<csr-id-5a9740e01528680c3f6543673fbd722913ac57c2/>
<csr-id-7932e59c509f1da93e8f431fbbef9a164de144d1/>
<csr-id-bb5de4483e6c0b5bfb15a0a5e2defdd067bd6c9d/>
<csr-id-a8301caede5aa2090b769c05e19a699267e07b07/>
<csr-id-8d79e7bc493584248b8adbc98ae4f178f517a6a6/>
<csr-id-59e2dff2d583cd8c5c55411688a6c8f4ddcf361a/>

### Chore

 - <csr-id-6ed8d209706a530c35d0cd284df72d7c04d3efe6/> 🔖 bump version
 - <csr-id-5e6dde45b3489ad93660372f654fa637a4379d98/> :bookmark: 0.6.0 raws identify themselves with a proprety raw_type
 - <csr-id-a6f00156cddc9b0c42b5f0a19883d5382c19d160/> 🔖 0.5.0 serialization as traits
 - <csr-id-5a9740e01528680c3f6543673fbd722913ac57c2/> 🔖 release 0.4.0
 - <csr-id-7932e59c509f1da93e8f431fbbef9a164de144d1/> 🧑‍💻 rust-analyzer for all features

### Documentation

 - <csr-id-db4c9c40712b05f619c12bd15b18b0220a5fd5ce/> 📝 add CHANGELOG.md
   using changelog generation via cargo-smart-release

### New Features

 - <csr-id-d21a63e2466562711aa277b0f6a26ace9ceb9534/> ✨ add an objectId to the info file object
 - <csr-id-3c48e4c79f7afe38d08057413b9549aa782bd528/> ✨ add function to write info_module json to file
   Also includes addtional property 'display_title' on df info file object
 - <csr-id-443c83957de911e1a775ef0c77bcb8bb5fcb3aa1/> ✨ add function to get the info.txt details for mods
 - <csr-id-021fe8e584658b8556b22c76a73eccdb6ebb55b4/> :sparkles: add field to report type of raw it is
 - <csr-id-30d48b2020a0681108db167489a726d58dfc360d/> :sparkles: coerce non-numeric numeric values into numerals
 - <csr-id-1b38a83d4427e8edbcd0bc1a80367b6f0335e431/> 🔊 improve error logging on into.txt parsing
 - <csr-id-0ffef9095c9156ea28dbdb138c31bb7350eda22e/> ✨ include raw module source directory
   Since df raws can be found in separate directories, we should save that in the data.
   
   The method signatures for parsing raws in directories have been changed to require a DFInfoFile reference instead of strings from that object.

### Bug Fixes

 - <csr-id-d180a175b6e40e46fd0d8ce136cf710078fa2b68/> :bug: fix raw_type field name

### Refactor

 - <csr-id-bb5de4483e6c0b5bfb15a0a5e2defdd067bd6c9d/> 🔥 remove object_id on creature
   Since objectId is something only used on the JSON side, move it into the JSON object converter only.
 - <csr-id-a8301caede5aa2090b769c05e19a699267e07b07/> ✨ modify example and update readme, bump release
 - <csr-id-8d79e7bc493584248b8adbc98ae4f178f517a6a6/> :fire: remove empty file
 - <csr-id-59e2dff2d583cd8c5c55411688a6c8f4ddcf361a/> 🔥 rewrite to support non-creature objects
   No non-creature objects were addedd, but this split apart the serialization steps to be traits which lets more object parsing be added in the future with out a ton of trouble. We now check the object type of the raw file first, then go and parse it. It adds an extra file read step (unfortunately). It may be possible to streamline further by opening file etc before checking and then parsring.
   
   Some library method signatures were modified

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 14 calendar days.
 - 14 days passed between releases.
 - 18 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dfraw_json_parser v0.7.2 ([`50ad40a`](https://github.com/nwesterhausen/dfraw_json_parser/commit/50ad40a8a2752b4e25c41de122c666b97c2261e1))
    - 📝 add CHANGELOG.md ([`db4c9c4`](https://github.com/nwesterhausen/dfraw_json_parser/commit/db4c9c40712b05f619c12bd15b18b0220a5fd5ce))
    - Release dfraw_json_parser v0.7.2 ([`f5568c8`](https://github.com/nwesterhausen/dfraw_json_parser/commit/f5568c8437d953b03190f4082f2fba9e49abfae7))
    - 🔥 remove object_id on creature ([`bb5de44`](https://github.com/nwesterhausen/dfraw_json_parser/commit/bb5de4483e6c0b5bfb15a0a5e2defdd067bd6c9d))
    - ✨ add an objectId to the info file object ([`d21a63e`](https://github.com/nwesterhausen/dfraw_json_parser/commit/d21a63e2466562711aa277b0f6a26ace9ceb9534))
    - ✨ modify example and update readme, bump release ([`a8301ca`](https://github.com/nwesterhausen/dfraw_json_parser/commit/a8301caede5aa2090b769c05e19a699267e07b07))
    - ✨ add function to write info_module json to file ([`3c48e4c`](https://github.com/nwesterhausen/dfraw_json_parser/commit/3c48e4c79f7afe38d08057413b9549aa782bd528))
    - 🔖 bump version ([`6ed8d20`](https://github.com/nwesterhausen/dfraw_json_parser/commit/6ed8d209706a530c35d0cd284df72d7c04d3efe6))
    - ✨ add function to get the info.txt details for mods ([`443c839`](https://github.com/nwesterhausen/dfraw_json_parser/commit/443c83957de911e1a775ef0c77bcb8bb5fcb3aa1))
    - :bookmark: 0.6.0 raws identify themselves with a proprety raw_type ([`5e6dde4`](https://github.com/nwesterhausen/dfraw_json_parser/commit/5e6dde45b3489ad93660372f654fa637a4379d98))
    - :bug: fix raw_type field name ([`d180a17`](https://github.com/nwesterhausen/dfraw_json_parser/commit/d180a175b6e40e46fd0d8ce136cf710078fa2b68))
    - :sparkles: add field to report type of raw it is ([`021fe8e`](https://github.com/nwesterhausen/dfraw_json_parser/commit/021fe8e584658b8556b22c76a73eccdb6ebb55b4))
    - :fire: remove empty file ([`8d79e7b`](https://github.com/nwesterhausen/dfraw_json_parser/commit/8d79e7bc493584248b8adbc98ae4f178f517a6a6))
    - :sparkles: coerce non-numeric numeric values into numerals ([`30d48b2`](https://github.com/nwesterhausen/dfraw_json_parser/commit/30d48b2020a0681108db167489a726d58dfc360d))
    - 🔖 0.5.0 serialization as traits ([`a6f0015`](https://github.com/nwesterhausen/dfraw_json_parser/commit/a6f00156cddc9b0c42b5f0a19883d5382c19d160))
    - 🔥 rewrite to support non-creature objects ([`59e2dff`](https://github.com/nwesterhausen/dfraw_json_parser/commit/59e2dff2d583cd8c5c55411688a6c8f4ddcf361a))
    - 🔊 improve error logging on into.txt parsing ([`1b38a83`](https://github.com/nwesterhausen/dfraw_json_parser/commit/1b38a83d4427e8edbcd0bc1a80367b6f0335e431))
    - 🔖 release 0.4.0 ([`5a9740e`](https://github.com/nwesterhausen/dfraw_json_parser/commit/5a9740e01528680c3f6543673fbd722913ac57c2))
    - ✨ include raw module source directory ([`0ffef90`](https://github.com/nwesterhausen/dfraw_json_parser/commit/0ffef9095c9156ea28dbdb138c31bb7350eda22e))
    - 🧑‍💻 rust-analyzer for all features ([`7932e59`](https://github.com/nwesterhausen/dfraw_json_parser/commit/7932e59c509f1da93e8f431fbbef9a164de144d1))
</details>

## v0.3.1 (2022-12-13)

<csr-id-373da1570664724774cddd13d34ae50c05a47dd8/>

### Chore

 - <csr-id-373da1570664724774cddd13d34ae50c05a47dd8/> 🔖 bump version

### New Features

 - <csr-id-a222b276b20a65f460d337235c1c5833576cda25/> ✨ add extra info about what kind of raw module is parsed
 - <csr-id-72fc3a666dd7c274c181f8877bc3d1bb2d2742e0/> ✨ send current module along with progress
 - <csr-id-0b2449691aa6895f236da1c4e2869659dba1d167/> 🚀 add feature "tauri" to emit progress

### Bug Fixes

 - <csr-id-2fbd8699570e0f1c95ef14a8903d86de3c289685/> 🚑️ with final check of pct, must be initialized first
 - <csr-id-2505a5cdc04dd52e46189f26b047c55980a8f5c4/> ⚡️ only emit 1.0 if it hasn't already

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - 🔖 bump version ([`373da15`](https://github.com/nwesterhausen/dfraw_json_parser/commit/373da1570664724774cddd13d34ae50c05a47dd8))
    - Merge pull request #7 from nwesterhausen/tauri-emit ([`75f37a4`](https://github.com/nwesterhausen/dfraw_json_parser/commit/75f37a494796bb3a53b119a5b8dc7219aa2c5edb))
    - ✨ add extra info about what kind of raw module is parsed ([`a222b27`](https://github.com/nwesterhausen/dfraw_json_parser/commit/a222b276b20a65f460d337235c1c5833576cda25))
    - ✨ send current module along with progress ([`72fc3a6`](https://github.com/nwesterhausen/dfraw_json_parser/commit/72fc3a666dd7c274c181f8877bc3d1bb2d2742e0))
    - 🚑️ with final check of pct, must be initialized first ([`2fbd869`](https://github.com/nwesterhausen/dfraw_json_parser/commit/2fbd8699570e0f1c95ef14a8903d86de3c289685))
    - ⚡️ only emit 1.0 if it hasn't already ([`2505a5c`](https://github.com/nwesterhausen/dfraw_json_parser/commit/2505a5cdc04dd52e46189f26b047c55980a8f5c4))
    - 🚀 add feature "tauri" to emit progress ([`0b24496`](https://github.com/nwesterhausen/dfraw_json_parser/commit/0b2449691aa6895f236da1c4e2869659dba1d167))
</details>

## v0.3.0 (2022-12-13)

<csr-id-0c7aebb62c9b8cae212207bddcae6edbece83995/>

### Chore

 - <csr-id-0c7aebb62c9b8cae212207bddcae6edbece83995/> ⬆️ update dependencies

### Documentation

 - <csr-id-bd06df53aef97bd7a7e44276f707a58d52387541/> 📝 update rustdoc
 - <csr-id-cc6ee32284b48968c361337f88d45d9059f44af6/> 📝 update crates.io required documentation

### New Features

 - <csr-id-41822170b73e9b26df9561789791945f6658db40/> ✨ improve library-ness, bump to 0.2.0
 - <csr-id-5698147910b87f039fe755f46af7111f384b8e38/> ✨ update from other project, make library

### Bug Fixes

 - <csr-id-c8dba2275374c1cbff53b285388b00cb79dfb844/> 🔥 don't check in Cargo.lock
 - <csr-id-b47b6f3e60c84d3af03ee84444c85b84c14b6112/> 🐛 fix type definition

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 328 calendar days.
 - 329 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #6 from nwesterhausen/nwesterhausen/issue5 ([`116e5fe`](https://github.com/nwesterhausen/dfraw_json_parser/commit/116e5fe8872c2e2d4e6e46f579dd27ddab38f866))
    - support DF 50.xx Fixes #5 ([`1b1eafa`](https://github.com/nwesterhausen/dfraw_json_parser/commit/1b1eafa8dec31e412e03d6a5f6270e49fde4e61b))
    - ⬆️ update dependencies ([`0c7aebb`](https://github.com/nwesterhausen/dfraw_json_parser/commit/0c7aebb62c9b8cae212207bddcae6edbece83995))
    - 🔥 don't check in Cargo.lock ([`c8dba22`](https://github.com/nwesterhausen/dfraw_json_parser/commit/c8dba2275374c1cbff53b285388b00cb79dfb844))
    - 📝 update rustdoc ([`bd06df5`](https://github.com/nwesterhausen/dfraw_json_parser/commit/bd06df53aef97bd7a7e44276f707a58d52387541))
    - 📝 update crates.io required documentation ([`cc6ee32`](https://github.com/nwesterhausen/dfraw_json_parser/commit/cc6ee32284b48968c361337f88d45d9059f44af6))
    - Create LICENSE ([`45739ae`](https://github.com/nwesterhausen/dfraw_json_parser/commit/45739aec1e9a7d1db3252bd5a034502d72a0d823))
    - 🐛 fix type definition ([`b47b6f3`](https://github.com/nwesterhausen/dfraw_json_parser/commit/b47b6f3e60c84d3af03ee84444c85b84c14b6112))
    - ✨ improve library-ness, bump to 0.2.0 ([`4182217`](https://github.com/nwesterhausen/dfraw_json_parser/commit/41822170b73e9b26df9561789791945f6658db40))
    - ✨ update from other project, make library ([`5698147`](https://github.com/nwesterhausen/dfraw_json_parser/commit/5698147910b87f039fe755f46af7111f384b8e38))
    - remove powershell runner and test raws ([`eb80888`](https://github.com/nwesterhausen/dfraw_json_parser/commit/eb808885c62180480f5c3f0f30aa2ba4354de33b))
    - Update Cargo.toml ([`3005293`](https://github.com/nwesterhausen/dfraw_json_parser/commit/30052931cc877fca0bada03d87a3bde24f8ae2b4))
    - format cargo.toml ([`9d5b6ef`](https://github.com/nwesterhausen/dfraw_json_parser/commit/9d5b6ef1422634ed300038eb8a6c9b6f5aaac374))
</details>

## v0.1.1 (2022-01-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #2 from nwesterhausen/creature-improvements ([`d0f1b5e`](https://github.com/nwesterhausen/dfraw_json_parser/commit/d0f1b5e586045b06049cf64f95d49a88d8f56e19))
    - cargo fmt ([`ceac5e5`](https://github.com/nwesterhausen/dfraw_json_parser/commit/ceac5e58fcc84ef6417d050d597b1a73ec899bce))
    - do more heavy lifting in rust program ([`a146143`](https://github.com/nwesterhausen/dfraw_json_parser/commit/a146143e0c2fa254f4f592d6b6cd66349782860a))
    - cargo fmt ([`e63ebe1`](https://github.com/nwesterhausen/dfraw_json_parser/commit/e63ebe1cd82f1f7d618aa5b56811397f8adbde3c))
    - fix milkable parsing ([`b102c4f`](https://github.com/nwesterhausen/dfraw_json_parser/commit/b102c4f91625dc3d1de067fa11c0af28a88de130))
    - added tokens to match statement ([`695f7a6`](https://github.com/nwesterhausen/dfraw_json_parser/commit/695f7a64290451d50b77fc79f7f3aa051634a445))
    - rewrite creature raw struct ([`2b43dfe`](https://github.com/nwesterhausen/dfraw_json_parser/commit/2b43dfe73c441dc8ea074f076054c89ded21f626))
    - decouple string conversion from reader ([`cd6ae5c`](https://github.com/nwesterhausen/dfraw_json_parser/commit/cd6ae5ce96b1d568131ee816a23f7a4a386cc252))
    - play with building modal ala dfwiki ([`ef58bbe`](https://github.com/nwesterhausen/dfraw_json_parser/commit/ef58bbe70c5e510b25df5a42b45379a26bce21be))
    - improve results display ([`88e4600`](https://github.com/nwesterhausen/dfraw_json_parser/commit/88e460038c410269a40d2ecb3d4b5e2cb936fe1c))
    - abstract names ([`69c29c0`](https://github.com/nwesterhausen/dfraw_json_parser/commit/69c29c0ac70ba58d32c5654ab3c2464a2ec821ef))
    - remove print messages ([`78125b1`](https://github.com/nwesterhausen/dfraw_json_parser/commit/78125b128be9351848f5a084b22b4f740ddb3b8c))
</details>

## v0.1.0 (2022-01-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 25 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - cargo fmt ([`773599d`](https://github.com/nwesterhausen/dfraw_json_parser/commit/773599d4fa25f90b0d609d380bee85a6acc3083a))
    - Update README.md ([`8a329a8`](https://github.com/nwesterhausen/dfraw_json_parser/commit/8a329a8bed1194d9d1841de1df09dc5a55f45f9a))
    - remove 'NOT IMPLEMENTED' message ([`c0649c4`](https://github.com/nwesterhausen/dfraw_json_parser/commit/c0649c456c02b2ee266cdeaec86ae45ada25f7e7))
    - add web server ([`0026ce6`](https://github.com/nwesterhausen/dfraw_json_parser/commit/0026ce66f1bb92fe2b7e9575f135ed180d341242))
    - add args for web server and port ([`57ca9b6`](https://github.com/nwesterhausen/dfraw_json_parser/commit/57ca9b6ea66b1c533287068c202b9c0998db6fc4))
    - remove perl script from repo ([`042c5a2`](https://github.com/nwesterhausen/dfraw_json_parser/commit/042c5a2b2ed94e346b49148797c05c529886bb72))
    - move parser to module ([`cfe7bad`](https://github.com/nwesterhausen/dfraw_json_parser/commit/cfe7bad8daca8112f73aff50356ff7f5d8513d43))
    - Merge pull request #1 from nwesterhausen/rust-conv ([`36e47b8`](https://github.com/nwesterhausen/dfraw_json_parser/commit/36e47b84bc23d2b2cd073adbf3154f91392ed355))
    - add long_help for arguments ([`244985a`](https://github.com/nwesterhausen/dfraw_json_parser/commit/244985a3e8b06cb1de62d6efe9dddc928d17f8e8))
    - remove enumeration ([`e85c4ad`](https://github.com/nwesterhausen/dfraw_json_parser/commit/e85c4adb3f451bc2472bd47d33ddaab865271851))
    - working ([`7e120e3`](https://github.com/nwesterhausen/dfraw_json_parser/commit/7e120e33711e95aa9397d1b00c3c20536c6779fa))
    - handle directory globbing inside rust ([`474831c`](https://github.com/nwesterhausen/dfraw_json_parser/commit/474831c8d3253c1accac591f56ad98b6cda474a3))
    - raws_dir optional, and not specified means no parsing ([`eba5318`](https://github.com/nwesterhausen/dfraw_json_parser/commit/eba5318a3898bcaabadfa0fdff858adb6183966e))
    - add argument parser clap and walkdir ([`1f5747b`](https://github.com/nwesterhausen/dfraw_json_parser/commit/1f5747bd581deb51d98623304daa54d54b026602))
    - update readme with rust info ([`65d3a27`](https://github.com/nwesterhausen/dfraw_json_parser/commit/65d3a278540740e98d38355aec20807600e33c07))
    - improve readability ([`5c72c99`](https://github.com/nwesterhausen/dfraw_json_parser/commit/5c72c99286e55b0de4a7cadb1a24867065016f5f))
    - comment html code and spread out javascript ([`46d08f0`](https://github.com/nwesterhausen/dfraw_json_parser/commit/46d08f0c3c9d7be886dd99a1619d15f81a2b7900))
    - fix warning about snake_case ([`65bfc11`](https://github.com/nwesterhausen/dfraw_json_parser/commit/65bfc11b86508f8123f8e56f5e5cf84606deaa2b))
    - change web directory to www ([`6b78592`](https://github.com/nwesterhausen/dfraw_json_parser/commit/6b785921ac44ea8dc89ec5ecd3fb831ab11b3295))
    - add webpage which lets you search the out.json file ([`122772a`](https://github.com/nwesterhausen/dfraw_json_parser/commit/122772ac559e87b3ae20681569a2c85ca22cc99e))
    - keep track of when we reach the final file and don't put a comma ([`8849753`](https://github.com/nwesterhausen/dfraw_json_parser/commit/884975378bdab366ba8c595ac9af044b06e209f3))
    - df raw files are ASCII, decode as such ([`e03da5c`](https://github.com/nwesterhausen/dfraw_json_parser/commit/e03da5c3deb3947f2d1ec059ff3851490cf2e659))
    - semi-working rust impl ([`456e614`](https://github.com/nwesterhausen/dfraw_json_parser/commit/456e614d542587bcb0801c256de0bf16a7d3527b))
    - redone in perl with powershell runner ([`942e6e3`](https://github.com/nwesterhausen/dfraw_json_parser/commit/942e6e360e3c1813002a716c0e776ae1b2517f9c))
    - rewrite to perl ([`234e503`](https://github.com/nwesterhausen/dfraw_json_parser/commit/234e50385c4490e0dd024265807a6a568a155a46))
    - ditonal changes -- now not working properly ([`b52aaa4`](https://github.com/nwesterhausen/dfraw_json_parser/commit/b52aaa48b626ffd46d16a556ba64061bd07c783b))
    - initial commit ([`abb1950`](https://github.com/nwesterhausen/dfraw_json_parser/commit/abb1950f4ab12e53b87f54b8c8cc1f122d0cee9a))
</details>

