import type { WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { Arena } from "../shared/games/arena.tsx";
import type { Memory } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Memory>, "value" | "onReady" | "onStatus">;

export const Widget = (props: Props) => <Arena make={make} {...props} />;
