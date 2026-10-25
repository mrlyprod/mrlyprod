import { expect, test } from "bun:test";
import { appicon } from "./appicon.ts";

const icon = (body: string, defs = "") =>
  `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">${defs ? `<defs>${defs}</defs>` : ""}<rect class="back" width="1024" height="1024" fill="url(#back)"/>${body}</svg>`;
const BACK =
  '<linearGradient id="back"><stop offset="0" stop-color="#5aaeff"/><stop offset="1" stop-color="#008cff"/></linearGradient>';

test("an icon namespaces its ids and links, so two icons on one page never share a paint", () => {
  const { defs, art } = appicon(
    "snake",
    icon(
      '<circle r="9" fill="url(#sun)"/><use href="#dot"/><path id="dot" d="M0 0h9" fill="#ff8f2c"/>',
      `${BACK}<radialGradient id="sun"><stop offset="0" stop-color="#ffd100"/></radialGradient>`,
    ),
  );
  expect(defs).toContain('id="snake-back"');
  expect(defs).toContain('id="snake-sun"');
  expect(art).toContain('fill="url(#snake-back)"');
  expect(art).toContain('fill="url(#snake-sun)"');
  expect(art).toContain('href="#snake-dot"');
  expect(art).toContain('fill="#ff8f2c"');
});

test("an icon lays its shadow copy, without ids, under the faded ink, which is isolated so WebKit flattens it", () => {
  const { art } = appicon("x", icon('<path id="p" d="M0 0h9" fill="#f00"/>', BACK));
  expect(art).toBe(
    '<rect class="back" width="1024" height="1024" fill="url(#x-back)"/><g class="lift"><path d="M0 0h9" fill="#f00"/></g><g class="ink" mask="url(#appicon-fade)"><g style="isolation:isolate"><path id="x-p" d="M0 0h9" fill="#f00"/></g></g>',
  );
});

test("in Dark white ink fades from a dim key to the key, the last stop of the back, over the white shapes, and data-dark overrides one shape", () => {
  const { defs, art } = appicon(
    "mail",
    icon(
      '<path d="M276 512A236 236 0 1 0 748 512A236 236 0 1 0 276 512Z" fill="#fff"/><g fill="#fff"><rect x="100" y="800" width="10" height="20"/></g><path d="M9 0h9" fill="#fff" data-dark="#32cc58"/>',
      BACK,
    ),
  );
  expect(defs).toContain(
    '<linearGradient id="appicon-mail-w" gradientUnits="userSpaceOnUse" x1="0" y1="276" x2="0" y2="820"><stop offset="0" stop-color="color-mix(in oklab, #008cff 35%, #212121)"/><stop offset="1" stop-color="#008cff"/></linearGradient>',
  );
  expect(art).toContain('<g class="ink" mask="url(#appicon-fade)" style="--w:url(#appicon-mail-w)">');
  expect(art).toContain('<path class="w" d="M276 512A236 236 0 1 0 748 512A236 236 0 1 0 276 512Z" fill="#fff"/>');
  expect(art).toContain('<path class="d" d="M9 0h9" fill="#fff" style="--d:#32cc58"/>');
});

test("a turned white shape gets its own copy of the Dark fade, turned back so it still runs down the icon", () => {
  const { defs, art } = appicon(
    "turn",
    icon('<rect x="312" y="462" width="400" height="100" fill="#fff" transform="rotate(90 512 512)"/>', BACK),
  );
  expect(defs).toContain(
    '<linearGradient id="appicon-turn-w1" gradientUnits="userSpaceOnUse" x1="0" y1="312" x2="0" y2="712" gradientTransform="matrix(0 -1 1 0 0 1024)">',
  );
  expect(art).toContain('style="--w:url(#appicon-turn-w1)"');
});

test("a white <use> moved by y gets a Dark fade moved back, since its shapes draw in the moved space", () => {
  const { defs, art } = appicon(
    "leaf",
    icon('<use href="#leaf" y="400" fill="#fff"/>', `${BACK}<path id="leaf" d="M300 100h400v200h-400z"/>`),
  );
  expect(defs).toContain(
    '<linearGradient id="appicon-leaf-w1" gradientUnits="userSpaceOnUse" x1="0" y1="500" x2="0" y2="700" gradientTransform="matrix(1 0 0 1 0 -400)">',
  );
  expect(art).toContain('<use class="w" href="#leaf-leaf" y="400" fill="#fff" style="--w:url(#appicon-leaf-w1)"/>');
});

test("an icon refuses what the looks cannot restyle or what reaches outside it", () => {
  const bad = [
    '<path d="M0 0" style="fill:red"/>',
    "<text>A</text>",
    '<path d="M0 0" class="x"/>',
    '<path d="M0 0" fill="url(https://x.y/a)"/>',
    '<path d="M0 0" fill="url(#nope)"/>',
    '<path d="M0 0" data-mono="half"/>',
    '<image href="a.png"/>',
  ];
  for (const body of bad) expect(() => appicon("x", icon(body, BACK))).toThrow(/^appicon: x\/icon\.svg/);
  expect(() => appicon("x", icon("", BACK))).toThrow(/draws nothing/);
  expect(() => appicon("x", '<svg viewBox="0 0 100 100"><rect class="back"/><path d="M0 0"/></svg>')).toThrow(
    /viewBox/,
  );
});
