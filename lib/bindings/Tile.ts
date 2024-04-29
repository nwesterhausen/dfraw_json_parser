// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Color } from "./Color";

/**
 * Representation of a character tile (literally a single character) that is used in DF Classic
 */
export type Tile = {
  character: string;
  altCharacter: string | null;
  color: Color | null;
  glowCharacter: string | null;
  glowColor: Color | null;
};
