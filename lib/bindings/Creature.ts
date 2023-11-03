// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Biome } from "./Biome";
import type { Caste } from "./Caste";
import type { CreatureTag } from "./CreatureTag";
import type { Metadata } from "./Metadata";
import type { Name } from "./Name";
import type { SelectCreature } from "./SelectCreature";
import type { SingPlurName } from "./SingPlurName";
import type { Tile } from "./Tile";

export interface Creature {
  metadata: Metadata;
  identifier: string;
  castes: Array<Caste>;
  tags: Array<CreatureTag>;
  biomes: Array<Biome>;
  prefStrings: Array<string>;
  tile: Tile;
  frequency: number;
  clusterNumber: Array<number>;
  populationNumber: Array<number>;
  undergroundDepth: Array<number>;
  generalBabyName: SingPlurName;
  generalChildName: SingPlurName;
  name: Name;
  copyTagsFrom: string;
  applyCreatureVariation: Array<string>;
  objectId: string;
  selectCreatureVariation: Array<SelectCreature>;
}
