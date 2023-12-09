# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.0.1 (2023-12-09)

<csr-id-afa8e0e1d327654b1cfca2bad6d1cfad2deabe2b/>
<csr-id-458bfbcdebd4e9f3463ac52aa0a4b8d74823e314/>
<csr-id-550c0b756c7a4c15ddc466b2ed84decc4ae8801f/>
<csr-id-65288c71f553aedef24ec9cf80cfd65601044f73/>
<csr-id-bd013b63d53420088773f96c096a0df55655bc59/>

### Changed

 - <csr-id-3afa6a764c0dcc8b4cc3678a647f6bfbe3cfcd5a/> replace log calls with tracing

### Chore

 - <csr-id-afa8e0e1d327654b1cfca2bad6d1cfad2deabe2b/> bump versions
 - <csr-id-458bfbcdebd4e9f3463ac52aa0a4b8d74823e314/> specify version
 - <csr-id-550c0b756c7a4c15ddc466b2ed84decc4ae8801f/> bump package versions
 - <csr-id-65288c71f553aedef24ec9cf80cfd65601044f73/> move to workspace

### New Features

 - <csr-id-b1fda6c6f90758ffe4c19641aa49d6c9729b15e6/> improve creature parsing
   * refactor: CreatureVariation as object
   
   * chore: update bindings
   
   * feat: impl RawObject for CreatureVariation
   
   * chore(deps): update github/codeql-action action to v2.22.9
   
   * feat: apply_creature_variation in progress
   
   * feat: add parsing of gaits and add them to creatures
   
   * fix: gait is a caste tag
   
   * feat: add unprocessed raw type to facilitate creature parsing
   
   * wip: creature variation parsing for unprocessed_raw
   
   * feat: handle copy_tags_from and creature_variations properly
   
   * feat: add `log_summary` option
   
   * refactor: CreatureVariation as object
   
   * chore: update bindings
   
   * feat: impl RawObject for CreatureVariation
   
   * feat: apply_creature_variation in progress
   
   * feat: add parsing of gaits and add them to creatures
   
   * fix: gait is a caste tag
   
   * feat: add unprocessed raw type to facilitate creature parsing
   
   * wip: creature variation parsing for unprocessed_raw
   
   * feat: handle copy_tags_from and creature_variations properly
   
   * feat: add `log_summary` option
   
   ---------

### Documentation

 - <csr-id-c530ed246593769a9c279a455f694919d94992cc/> add example usage

### Bug Fixes

 - <csr-id-cad7df65d139b392b6b7c47609240c4f7da109db/> improve error handling
 - <csr-id-c6a1b32a11814de757ff5a8c0a5739a67cf50bf8/> cli should propagate the parsing error

### Refactor

 - <csr-id-bd013b63d53420088773f96c096a0df55655bc59/> improve module ergonomics
   BREAKING CHANGE
   This changes a lot of module export locations, hopefully to avoid changing them in the future.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 9 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Improve creature parsing ([`b1fda6c`](https://github.com/nwesterhausen/dfraw_json_parser/commit/b1fda6c6f90758ffe4c19641aa49d6c9729b15e6))
</details>

## v1.0.0 (2023-11-29)

<csr-id-550c0b756c7a4c15ddc466b2ed84decc4ae8801f/>
<csr-id-65288c71f553aedef24ec9cf80cfd65601044f73/>
<csr-id-bd013b63d53420088773f96c096a0df55655bc59/>

### Changed

 - <csr-id-3afa6a764c0dcc8b4cc3678a647f6bfbe3cfcd5a/> replace log calls with tracing

### Chore

 - <csr-id-550c0b756c7a4c15ddc466b2ed84decc4ae8801f/> bump package versions
 - <csr-id-65288c71f553aedef24ec9cf80cfd65601044f73/> move to workspace

### Bug Fixes

 - <csr-id-cad7df65d139b392b6b7c47609240c4f7da109db/> improve error handling
 - <csr-id-c6a1b32a11814de757ff5a8c0a5739a67cf50bf8/> cli should propagate the parsing error

### Refactor

 - <csr-id-bd013b63d53420088773f96c096a0df55655bc59/> improve module ergonomics
   BREAKING CHANGE
   This changes a lot of module export locations, hopefully to avoid changing them in the future.

