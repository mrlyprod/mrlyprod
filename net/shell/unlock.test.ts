import { expect, test } from "bun:test";
import { HEADER as ASKED, DOOR as EDGE } from "../hosts/web/infra/guard.js";
import { DOOR, HEADER, knock } from "./unlock.tsx";

test("knock asks the edge's door with the password in its header, never the url, and opens only on 204", async () => {
  const seen: [string, string][] = [];
  const answer = (status: number) => async (url: string | URL | Request, init?: RequestInit) => {
    seen.push([String(url), new Headers(init?.headers).get(HEADER)!]);
    return new Response(null, { status });
  };
  const said = [
    await knock("five plain words", answer(204) as typeof fetch),
    await knock("five plain words", answer(401) as typeof fetch),
  ];
  expect([said, seen[0], [DOOR, HEADER]]).toEqual([
    [true, false],
    [EDGE, "five%20plain%20words"],
    [EDGE, ASKED],
  ]);
});
