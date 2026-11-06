import type { WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import { Arena } from "../shared/games/arena.tsx";
import type { Tennis } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Tennis>, "value" | "onReady" | "onStatus">;

export const Widget = (props: Props) => <Arena make={make} {...props} />;
