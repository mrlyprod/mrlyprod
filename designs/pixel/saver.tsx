import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { useEffect, useRef } from "react";
import { play } from "./attract.ts";
import { spot, time } from "./kit.ts";

const kind = () => (typeof document === "undefined" ? "" : (document.documentElement.dataset.attract ?? ""));

const calm = () =>
  typeof matchMedia === "function" &&
  (matchMedia("(prefers-reduced-motion: reduce)").matches || document.documentElement.dataset.motion === "reduce");

export function Saver({ clock, state, stage }: Props["saver"]) {
  const at = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const canvas = at.current;
    if (!canvas || kind() === "apps") return undefined;
    return play(canvas, kind(), calm());
  }, []);
  return (
    <div className="pixel saver" data-state={state} data-attract={kind() || "maze"}>
      <div className="stage" {...stage()} />
      <canvas className="attract" ref={at} aria-hidden="true" />
      {clock && (
        <time className="big" style={spot(clock.now)}>
          {time(clock)}
        </time>
      )}
      <p className="insert" aria-hidden="true">
        INSERT COIN
      </p>
    </div>
  );
}
