import { expect, test } from "bun:test";
import * as life from "@mrlyprod/mrly/life";
import { renderToStaticMarkup } from "react-dom/server";
import { grammar } from "../../lib/life/engine.ts";
import type { Row } from "./shelves.tsx";
import { ALL, classes, Encyclopedia, families, shelved } from "./shelves.tsx";

const row = (
  key: string,
  fate: string,
  cls: string,
  family: string,
  gen: number,
  period: number,
  last: number,
): Row => ({
  key,
  fate,
  class: cls,
  family,
  gen,
  period,
  pop: { last, max: [last, 0] },
});

const ROWS: Row[] = [
  row(
    "code1.3.1 x1 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151",
    "dead",
    "island x1",
    "code1",
    2,
    1,
    0,
  ),
  row(
    "code1.3.1 x1 island mask:code1.3.1 birth:odds survive:odds x:256 cap:151",
    "dead",
    "island x1",
    "code1",
    1,
    1,
    0,
  ),
  row("net.3.1 x1 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151", "still", "island x1", "net", 8, 1, 4),
  row(
    "code1.3.1 x5 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151",
    "loop",
    "island x5",
    "code1",
    15,
    3,
    16,
  ),
  row(
    "code1.3.2 x1 wallpaper mask:code3.3.1 birth:primes survive:primes x:256",
    "limit",
    "wallpaper",
    "code1",
    256,
    0,
    35,
  ),
];

const keys = (shelves: [string, Row[]][]) => shelves.map(([stop, list]) => [stop, list.map((one) => one.key)]);

test("the classes keep the extract's order, the families the grammar's, and a shelf is one stop kind of the chosen class after the family, stop and search filters", () => {
  expect([classes(ROWS), families(ROWS, grammar(life).shapes)]).toEqual([
    ["island x1", "island x5", "wallpaper"],
    ["code1", "net"],
  ]);
  const open = { cls: "island x1", family: ALL, stop: ALL };
  expect(keys(shelved(ROWS, open, ""))).toEqual([
    ["dead", [ROWS[0].key, ROWS[1].key]],
    ["still", [ROWS[2].key]],
  ]);
  expect(keys(shelved(ROWS, { ...open, family: "net" }, ""))).toEqual([["still", [ROWS[2].key]]]);
  expect(keys(shelved(ROWS, { ...open, stop: "dead" }, "odds"))).toEqual([["dead", [ROWS[1].key]]]);
  expect(keys(shelved(ROWS, { ...open, cls: "" }, "ODDS"))).toEqual([["dead", [ROWS[1].key]]]);
  expect(keys(shelved(ROWS, { ...open, cls: "wallpaper" }, ""))).toEqual([["limit", [ROWS[4].key]]]);
});

test("the encyclopedia draws three chip rows and a shelf per stop with its count, the picked row current, and one bare shelf before the rows load", () => {
  const html = renderToStaticMarkup(
    <Encyclopedia
      rows={ROWS}
      life={life}
      find=""
      filter={{ cls: "", family: ALL, stop: ALL }}
      onFilter={() => {}}
      current={ROWS[1].key}
      onPick={() => {}}
    />,
  );
  expect(
    ["Class", "Family", "Stop", "Dead (2)", "Still (1)"].map((label) => html.includes(`aria-label="${label}"`)),
  ).toEqual([true, true, true, true, true]);
  expect(html.includes('aria-label="Loop (')).toBe(false);
  expect(html).toContain(
    `<button type="button" data-item="" aria-current="true" class="on" title="${ROWS[1].key}" role="listitem"><canvas width="40" height="40" aria-hidden="true"></canvas><span class="label">code1 3.1 · code1.3.1</span><span class="sub">1, p1, 0</span></button>`,
  );
  expect(
    renderToStaticMarkup(
      <Encyclopedia
        rows={null}
        life={life}
        find=""
        filter={{ cls: "", family: ALL, stop: ALL }}
        onFilter={() => {}}
        current=""
        onPick={() => {}}
      />,
    ),
  ).toBe(
    '<section class="shelf" aria-label="Encyclopedia"><h3>Encyclopedia</h3><div class="list" role="list"></div></section>',
  );
});
