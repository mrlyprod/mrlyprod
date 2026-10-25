import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { DEEP, ledger } from "./engine.ts";
import { facts, found } from "./facts.ts";

const book = ledger(math);

test("the readouts of a row: its record, status, closed form, term count against the budget, space, axis and the records holding it", () => {
  const carpet = book.row({ dim: 2, base: 2, code: "7", measure: "fills", axis: "side" });
  book.read(carpet, 12, DEEP.cells);
  expect(facts(book, carpet, 12)).toMatchObject({
    record: { id: "A000567" },
    status: "proved",
    closed: "k(3k - 2)",
    terms: "12",
    space: "dimension 2, base 2",
    axis: "odd side 2k - 1 at level 1",
  });
  expect(facts(book, carpet, 12).found.map(({ record }) => record.id)).toEqual(["A000567"]);
  const deep = book.row({ dim: 3, base: 2, code: "23", measure: "euler", axis: "level" });
  book.read(deep, 8, DEEP.cells);
  expect(facts(book, deep, 8)).toMatchObject({
    status: "unmatched",
    closed: "none known",
    terms: "3, to the budget",
    axis: "level L at side 3",
  });
  expect(found(["4", "9", "16", "25"])).toMatchObject({ id: "A000290", at: 2, status: "Found" });
  expect(found(["4", "9"])).toBe(null);
});
