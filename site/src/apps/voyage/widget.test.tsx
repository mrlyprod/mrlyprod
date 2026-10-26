import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { mount } from "../../../test/dom.ts";
import type { Handle } from "../../lib/app/page.tsx";
import { defaults } from "../../lib/app/spec.ts";
import type { Held, Knobs } from "./scene.ts";
import { SPEC } from "./scene.ts";
import { Widget } from "./widget.tsx";

test("without WebGL2 Again does nothing and never throws", async () => {
  let handle: Handle<Held> | null = null;
  await mount(async (draw) => {
    await draw(
      <Widget
        value={{ ...(defaults(SPEC) as Knobs), seed: 3, sound: 0, music: 0 }}
        onChange={() => {}}
        onReady={(next) => (handle = next ?? handle)}
        onStatus={() => {}}
        onExport={() => {}}
        units={{ math }}
      />,
    );
    expect(() => handle!.again()).not.toThrow();
  });
});
