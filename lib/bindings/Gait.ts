// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { GaitType } from "./GaitType";
import type { Modifier } from "./Modifier";

/**
 * Gaits are a way to describe how a creature moves. Defined in the raws with:
 *
 * "GAIT:type:name:full speed:build up time:turning max:start speed:energy use"
 *
 * * use `NO_BUILD_UP` if you jump immediately to full speed
 *
 * these optional flags go at the end:
 *
 * * `LAYERS_SLOW` - fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
 * * `STRENGTH` - strength attribute can speed/slow movement
 * * `AGILITY` - agility attribute can speed/slow movement
 * * `STEALTH_SLOWS:<n>` - n is percentage slowed
 * * it would be interesting to allow quirky attributes (like mental stats), but they aren't supported yet
 *
 * Examples:
 *
 *    `[CV_NEW_TAG:GAIT:WALK:Sprint:!ARG4:10:3:!ARG2:50:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:50]`
 *    `[CV_NEW_TAG:GAIT:WALK:Run:!ARG3:5:3:!ARG2:10:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:20]`
 *    `[CV_NEW_TAG:GAIT:WALK:Jog:!ARG2:NO_BUILD_UP:5:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:10]`
 *    `[CV_NEW_TAG:GAIT:WALK:Walk:!ARG1:NO_BUILD_UP:0]`
 *    `[CV_NEW_TAG:GAIT:WALK:Stroll:!ARG5:NO_BUILD_UP:0]`
 *    `[CV_NEW_TAG:GAIT:WALK:Creep:!ARG6:NO_BUILD_UP:0]`
 */
export type Gait = {
  /**
   * The type of gait
   */
  gaitType: GaitType;
  /**
   * The name of the gait
   */
  name: string;
  /**
   * The maximum speed achievable by a creature using this gait.
   */
  maxSpeed: number;
  /**
   * The energy use of the gait
   */
  energyUse: number;
  /**
   * The gait modifiers
   *
   * These are optional, and may be empty.
   */
  modifiers: Array<Modifier>;
};
