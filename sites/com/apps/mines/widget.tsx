import { Btn } from "@mrlyprod/ui/kit/controls.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { Arena } from "../shared/games/arena.tsx";
import type { Mines, Said } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Mines>, "value" | "onReady" | "onStatus">;

export const Widget = (props: Props) => (
  <Arena<Mines, Said>
    make={make}
    more={(told, scene) => (
      <Bar side="actions">
        <Btn primary={told.flagging} aria-pressed={told.flagging} onClick={() => scene?.shift()}>
          {told.flagging ? "Flag" : "Dig"}
        </Btn>
      </Bar>
    )}
    {...props}
  />
);
