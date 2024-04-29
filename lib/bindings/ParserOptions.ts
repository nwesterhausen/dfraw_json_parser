// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ObjectType } from "./ObjectType";
import type { RawModuleLocation } from "./RawModuleLocation";

/**
 * # Parsing Options
 *
 * Specify what to parse and where to parse it from.
 *
 * ## Parsing `info.txt` vs the raw files
 *
 * There are two main parsing functions: `parse` and `parse_module_info_files`.
 *
 * Both use the same options struct, but they use it in different ways.
 *
 * When calling `parse`, the `ParserOptions` struct is used to specify what raws to parse and where to parse them from.
 * Any specified `raw_modules_to_parse` will not be parsed in the `parse` function, and the only items parsed in the
 * `parse_module_info_files` function are the `module_info_files_to_parse`.
 *
 * ## Example
 *
 * ```rust
 * use std::path::PathBuf;
 * use dfraw_json_parser::{ParserOptions, RawObject, ObjectType, RawModuleLocation};
 *
 * let mut options = ParserOptions::new("path/to/dwarf_fortress");
 * options.add_location_to_parse(RawModuleLocation::Vanilla);
 * // Clear the default object types
 * options.set_object_types_to_parse(vec![]);
 * // Add back in the ones we want
 * options.add_object_type_to_parse(ObjectType::Creature);
 * options.add_object_type_to_parse(ObjectType::CreatureVariation);
 * // Include the metadata with the parsed raws
 * options.attach_metadata_to_raws();
 *
 * // Parse the raws and info.txt files (not parsing here because the path is invalid)
 * // let parsed_raws = dfraw_json_parser::parse(&options);
 *```
 *
 */
export type ParserOptions = {
  /**
   * Whether to attach a metadata field to the raws.
   * If true, all raws will have a `metadata` field which shows information about the
   * raw file, its path, its module, and its parent directory.
   *
   * Default: false.
   */
  attachMetadataToRaws: boolean;
  /**
   * Whether to skip the "copy tags from" resolution step.
   * If true, the creature will have a populated `copy_tags_from` field instead.
   *
   * Default: false.
   */
  skipApplyCopyTagsFrom: boolean;
  /**
   * Whether to skip the apply "creature variations" resolution step.
   * When this is true, it will just leave the variations attached to the creature
   * in a `creature_variations` field.
   * If false, it will modify the creature data to include the variations.
   *
   * Note: This is currently not implemented.
   *
   * Default: false.
   */
  skipApplyCreatureVariations: boolean;
  /**
   * What types of raws to parse. If this is left empty, all parsable raws will be parsed.
   *
   * Default: [Creature, CreatureVariation, Entity, Plant, Inorganic, MaterialTemplate, Graphics, TilePage]
   */
  objectTypesToParse: Array<ObjectType>;
  /**
   * What locations to parse raws from. If this is left empty, no locations will be parsed.
   *
   * Setting locations to parse requires a valid `dwarf_fortress_directory` to be set.
   *
   * Default: None
   */
  locationsToParse: Array<RawModuleLocation>;
  /**
   * The path to the dwarf fortress directory. If no locations are specified, then this is not used.
   *
   * This is not used when parsing specific raws, modules, info files, or legends exports as specified by
   * `raw_files_to_parse`, `raw_modules_to_parse`, `module_info_files_to_parse`, or `legends_exports_to_parse`.
   *
   * Default: ""
   */
  dwarfFortressDirectory: string;
  /**
   * Optionally specify one or more legends_plus exports to parse in addition to the raws.
   * These exports include information about generated creatures which are not included in the
   * raws.
   *
   * Default: None
   */
  legendsExportsToParse: Array<string>;
  /**
   * Optionally specify one or more raw files to parse directly. These should be the raw files
   * themselves, not the containing directory.
   *
   * (e.g. `creature_standard.txt` in `data/vanilla/vanilla_creatures/objects/`)
   *
   * Note that these will be parsed in addition to the raws in the specified locations in the other
   * options. That means that if you specify a raw file that is also in the vanilla raws, it will
   * be parsed twice (if vanilla is in the locations to parse).
   *
   * Default: None
   */
  rawFilesToParse: Array<string>;
  /**
   * Optionally specify one or more raw modules to parse directly. These should be the module
   * directories, not the info.txt file.
   *
   * (e.g. `vanilla_creatures` in `data/vanilla/`)
   *
   * Note that these will be parsed in addition to the raws in the specified locations in the other
   * options. That means that if you specify a module that is also in the vanilla raws, it will
   * be parsed twice (if vanilla is in the locations to parse).
   *
   * Default: None
   */
  rawModulesToParse: Array<string>;
  /**
   * Optionally specify one or more module info files to parse directly. These should be the info.txt
   * files themselves, not the containing directory.
   *
   * (e.g. `info.txt` in `data/vanilla/vanilla_creatures/`)
   *
   * Note that if you are calling the `parse` function, this will be ignored. This is only used
   * when calling the `parse_module_info_files` function.
   */
  moduleInfoFilesToParse: Array<string>;
  /**
   * Include a summary of what was parsed in the log.
   *
   * If running with `tauri`, this will emit a `PARSE_SUMMARY` event with the summary as well.
   *
   * Default: false
   */
  logSummary: boolean;
};
