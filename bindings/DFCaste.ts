// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CasteTag } from "./CasteTag";
import type { DFBodySize } from "./DFBodySize";
import type { DFMilkable } from "./DFMilkable";
import type { DFTile } from "./DFTile";
import type { Name } from "./Name";
import type { SingPlurName } from "./SingPlurName";

export interface DFCaste {
  identifier: string;
  tags: Array<CasteTag>;
  description: string;
  babyName: SingPlurName;
  casteName: Name;
  childName: SingPlurName;
  clutchSize: Array<number>;
  litterSize: Array<number>;
  maxAge: Array<number>;
  baby: number;
  child: number;
  difficulty: number;
  eggSize: number;
  grassTrample: number;
  grazer: number;
  lowLightVision: number;
  petValue: number;
  popRatio: number;
  changeBodySizePercentage: number;
  creatureClass: Array<string>;
  bodySize: Array<DFBodySize>;
  milkable: DFMilkable;
  tile: DFTile;
}