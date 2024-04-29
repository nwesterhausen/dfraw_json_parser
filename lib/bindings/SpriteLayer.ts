// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Dimensions } from "./Dimensions";

export type SpriteLayer = {
  layerName: string;
  tilePageId: string;
  offset: Dimensions;
  offset2: Dimensions | null;
  largeImage: boolean | null;
  conditions: Array<[Condition, string]> | null;
};
