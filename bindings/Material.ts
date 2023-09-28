// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DFColor } from "./DFColor";
import type { FuelType } from "./FuelType";
import type { MaterialType } from "./MaterialType";
import type { MaterialUsage } from "./MaterialUsage";
import type { StateName } from "./StateName";
import type { Temperatures } from "./Temperatures";

export interface Material { materialType: MaterialType, name: string, fuelType: FuelType, creatureIdentifier: string, plantIdentifier: string, isLocalMaterial: boolean, reagentIdentifier: string, reactionProductIdentifier: string, templateIdentifier: string, usage: Array<MaterialUsage>, value: number, color: DFColor, stateNames: StateName, stateAdjectives: StateName, stateColors: StateName, temperatures: Temperatures, properties: Array<string>, }