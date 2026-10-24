import { expect, test } from "bun:test";
import { all, mount, text } from "../../../test/dom.js";
import { Frame } from "../../lib/app/frame.jsx";
import { defaults } from "../../lib/app/spec.js";
import { SETS, SPEC } from "./scene.js";
import { Shelves } from "./widget.jsx";

test("the library lists every shape and palette and marks the two on show", async () => {
  const items = await mount(async (draw, host) => {
    await draw(
      <Frame label="Polygons">
        <Shelves value={{ ...defaults(SPEC), seed: 1, shape: "hex", palette: "sea" }} onChange={() => {}} />
      </Frame>,
    );
    return all(host, (at) => "data-item" in (at.attributes ?? {}));
  });
  expect([items.length, items.filter((at) => at.attributes["aria-current"] === "true").map(text)]).toEqual([
    SETS.shape.length + SETS.palette.length,
    ["Hex6.6.6", "Sea"],
  ]);
});
