// File generated by specta. Do not edit!
//
/// lib/dfraw_json_parser::creature_caste::GaitModifier

export type Modifier =
  | "layersSlow"
  | "strength"
  | "agility"
  | { stealthSlows: { percentage: number } }
  | "noBuildUp"
  | { buildUp: { time: number; turning_max: number; start_speed: number } };
