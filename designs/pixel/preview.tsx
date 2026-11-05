import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { Sprite } from "./sprite.tsx";

export function Preview({ title, theme }: Props["preview"]) {
  return (
    <span className="pixel preview" data-tone={theme}>
      <span className="top">
        <i />
        {title.toUpperCase()}
      </span>
      <span className="maze">
        <Sprite art="pac" size={12} />
        <span className="dots" />
        <Sprite art="ghost" size={12} />
      </span>
    </span>
  );
}
