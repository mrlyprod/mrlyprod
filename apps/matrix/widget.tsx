import type { WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Units } from "../../net/sdk/spec.ts";
import type { Rain } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Rain>, "value" | "onReady">;

export const Widget = (props: Props) => <Scene make={make} {...props} />;
