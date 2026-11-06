import { Btn } from "@mrlyprod/ui/kit/controls.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { Arena } from "../shared/games/arena.tsx";
import type { Chess, Said } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Chess>, "value" | "onReady" | "onStatus">;

export const Widget = (props: Props) => (
  <Arena<Chess, Said>
    make={make}
    more={(told, scene) => (
      <Bar side="actions">
        <Btn disabled={told.at <= 0} onClick={() => scene?.back()}>
          Back
        </Btn>
        <Btn disabled={told.at >= told.total} onClick={() => scene?.next()}>
          Next
        </Btn>
      </Bar>
    )}
    {...props}
  />
);
