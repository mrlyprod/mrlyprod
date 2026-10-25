import type { WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Units } from "../../lib/app/spec.ts";
import type { Rain } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Rain>, "value" | "onReady">;

export const Widget = (props: Props) => <Scene make={make} {...props} />;
