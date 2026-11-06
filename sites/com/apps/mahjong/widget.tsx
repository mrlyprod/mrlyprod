import { Btn } from "@mrlyprod/ui/kit/controls.tsx";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import { useMemo } from "react";
import { Arena } from "../shared/games/arena.tsx";
import type { Mahjong, Told } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Mrly, Mahjong>, "value" | "units" | "onReady" | "onStatus">;

const controls = (told: Told, scene: Handle<Mahjong> | null) => (
  <Bar side="actions">
    {told.cleared && (
      <Btn primary onClick={() => scene?.act({ verb: "next" })}>
        Next
      </Btn>
    )}
    <Btn disabled={told.hints < 1 || told.cleared || told.stuck} onClick={() => scene?.act({ verb: "hint" })}>
      Hint {told.hints}
    </Btn>
    <Btn disabled={told.undos < 1 || told.cleared} onClick={() => scene?.act({ verb: "undo" })}>
      Undo {told.undos}
    </Btn>
    <Btn
      primary={told.stuck}
      disabled={told.shuffles < 1 || told.cleared}
      onClick={() => scene?.act({ verb: "shuffle" })}
    >
      Shuffle {told.shuffles}
    </Btn>
  </Bar>
);

export function Widget({ value, units, onReady, onStatus }: Props) {
  const game = useMemo(
    () => (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, mrly: units! }),
    [units],
  );
  return <Arena<Mahjong, Told> make={game} more={controls} value={value} onReady={onReady} onStatus={onStatus} />;
}
