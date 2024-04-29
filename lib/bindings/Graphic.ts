// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CustomGraphicExtension } from "./CustomGraphicExtension";
import type { GraphicType } from "./GraphicType";
import type { RawMetadata } from "./RawMetadata";
import type { SpriteGraphic } from "./SpriteGraphic";

export type Graphic = {
  metadata: RawMetadata | null;
  identifier: string;
  objectId: string;
  casteIdentifier: string | null;
  kind: GraphicType;
  sprites: Array<SpriteGraphic> | null;
  layers: Array<[string, Array<SpriteLayer>]> | null;
  growths: Array<[string, Array<SpriteGraphic>]> | null;
  customExtensions: Array<CustomGraphicExtension> | null;
  tags: Array<string> | null;
};
