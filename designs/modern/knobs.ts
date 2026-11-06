import type { Knobs } from "@mrlyprod/ui/shell/design.ts";

export const KNOBS: Knobs = {
  icons: true,
  own: [
    {
      id: "material",
      name: "Material",
      hint: "Glass blurs what lies under the bar, dock, widgets and panels; Frosted blurs it more, under a light tint; Solid hides it. Reduce transparency keeps it solid.",
      options: [
        ["", "Glass"],
        ["frosted", "Frosted"],
        ["solid", "Solid"],
      ],
    },
    {
      id: "font",
      name: "Font",
      options: [
        ["", "Default"],
        ["serif", "Serif"],
        ["mono", "Mono"],
      ],
    },
  ],
};
