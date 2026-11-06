import { Btn } from "@mrlyprod/ui/kit/controls.tsx";
import { Bar } from "@mrlyprod/ui/kit/frame.tsx";
import type { Handle, WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { Arena } from "../shared/games/arena.tsx";
import type { Asked, Quiz } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Quiz>, "value" | "onReady" | "onStatus">;

const SLOTS = [0, 1, 2];

const choices = (told: Asked, scene: Handle<Quiz> | null) =>
  told.options && (
    <Bar side="actions">
      {SLOTS.map((at) => (
        <Btn key={at} disabled={told.over} onClick={() => scene?.answer(at)}>
          {told.options?.[at]}
        </Btn>
      ))}
    </Bar>
  );

export const Widget = (props: Props) => <Arena<Quiz, Asked> make={make} more={choices} {...props} />;
