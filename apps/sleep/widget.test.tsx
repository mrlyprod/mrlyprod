import { expect, test } from "bun:test";
import { Frame } from "../../net/sdk/frame.tsx";
import { defaults } from "../../net/sdk/spec.ts";
import { all, mount, text } from "../../net/test/dom.ts";
import { SPEC, TILES } from "./scene.ts";
import { Shelves } from "./widget.tsx";

test("the library lists Mixed and every tile of the five designs and marks the one on show", async () => {
  const items = await mount(async (draw, host) => {
    await draw(
      <Frame label="Sleep">
        <Shelves value={{ ...defaults(SPEC), seed: 1, design: "net", number: 7 }} onChange={() => {}} />
      </Frame>,
    );
    return all(host, (at) => "data-item" in (at.attributes ?? {}));
  });
  expect([items.length, items.filter((at) => at.attributes["aria-current"] === "true").map(text)]).toEqual([
    TILES.length,
    ["7 x 7"],
  ]);
});
