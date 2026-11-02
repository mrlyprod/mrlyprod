import { expect, test } from "bun:test";
import type { PointerEvent, RefObject } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import type { Change, Hands, Touched } from "./touch.ts";
import { useTouch } from "./touch.ts";

type Spot = { u?: number; up?: boolean };

test("a pointer hands the scene its spot as it goes down and moves, and the lift after it, each patch to the page", () => {
  let hands: Hands | null = null;
  const Probe = ({ handle, change }: { handle: RefObject<Touched<Spot>>; change: RefObject<Change<Spot>> }) => {
    hands = useTouch(handle, change);
    return null;
  };
  const heard: unknown[][] = [];
  const sent: Spot[] = [];
  const handle: RefObject<Touched<Spot>> = {
    current: {
      touch: (u, v, start) => {
        heard.push(["touch", u, v, start]);
        return start ? null : { u };
      },
      lift: (u, v) => {
        heard.push(["lift", u, v]);
        return { up: true };
      },
    },
  };
  renderToStaticMarkup(<Probe handle={handle} change={{ current: (one) => sent.push(one) }} />);
  const box = {
    getBoundingClientRect: () => ({ left: 0, top: 0, width: 200, height: 100 }),
    setPointerCapture: () => {},
  };
  const at = (x: number, y: number) =>
    Object.assign({} as PointerEvent<Element>, { button: 0, clientX: x, clientY: y, pointerId: 1, currentTarget: box });
  hands!.onPointerMove(at(10, 10));
  hands!.onPointerDown(at(50, 50));
  hands!.onPointerMove(at(100, 50));
  hands!.onPointerUp(at(100, 50));
  hands!.onPointerUp(at(100, 50));
  expect(heard).toEqual([
    ["touch", 0.25, 0.5, true],
    ["touch", 0.5, 0.5, false],
    ["lift", 0.5, 0.5],
  ]);
  expect(sent).toEqual([{ u: 0.5 }, { up: true }]);
});
