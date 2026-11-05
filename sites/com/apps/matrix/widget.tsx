import type { WidgetProps } from "@mrlyprod/ui/kit/page.tsx";
import { Scene } from "@mrlyprod/ui/kit/page.tsx";
import type { Units } from "@mrlyprod/ui/kit/spec.ts";
import type { Rain } from "./scene.ts";
import { make } from "./scene.ts";

type Props = Pick<WidgetProps<Units, Rain>, "value" | "onReady">;

export const Widget = (props: Props) => <Scene make={make} {...props} />;
