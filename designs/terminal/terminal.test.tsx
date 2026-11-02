import { expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { terminal } from "./index.ts";

test("the terminal slider draws its value as a gauge of cells", () => {
  const Control = terminal.parts.control;
  const html = renderToStaticMarkup(
    <Control kind="slider" label="Speed" value={5} min={0} max={10} onChange={() => {}} />,
  );
  expect(html).toContain(`[${"#".repeat(8)}${"-".repeat(8)}]`);
});
