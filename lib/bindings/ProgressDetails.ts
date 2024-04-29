// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { RawModuleLocation } from "./RawModuleLocation";

/**
 * Details about the current file being parsed, specifically where and what it is.
 */
export type ProgressDetails = {
  /**
   * The current module location (only for stuff parsed via a specified location, otherwise is `RawModuleLocation::Unknown`)
   */
  location: RawModuleLocation;
  /**
   * The module the current file is in (if any)
   */
  module: string | null;
  /**
   * The parsed name of the file being parsed (if it can be determined)
   */
  rawFile: string | null;
  /**
   * The location of the file being parsed (if it can be determined)
   */
  fileLocation: string | null;
};
