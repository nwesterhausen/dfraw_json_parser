// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Condition } from "./Condition";
import type { Dimensions } from "./Dimensions";

export interface SpriteLayer {
  layerName: string;
  tilePageId: string;
  offset: Dimensions;
  offset2: Dimensions;
  largeImage: boolean;
  conditions: Array<[Condition, string]>;
}