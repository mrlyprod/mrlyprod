import type { Colorizer } from "./engine.ts";

declare module "@mrlyprod/math" {
  namespace moire {
    function render(
      field: Field,
      colorizer: Colorizer,
      levels: number,
      symmetric: boolean,
      invert: boolean,
      scale: number,
    ): Image;
  }
}
