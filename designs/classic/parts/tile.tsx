import type { Props } from "@mrlyprod/ui/shell/design.ts";

export function Tile({ title, href, art, current, soon, named, bind }: Props["tile"]) {
  return (
    <a
      className="tile"
      href={href}
      draggable="false"
      aria-label={named ? undefined : title}
      aria-current={current ? "page" : undefined}
      aria-disabled={soon || undefined}
      data-name={title}
      {...bind()}
    >
      <span className="art">{art}</span>
      {named && <span className="name">{title}</span>}
    </a>
  );
}
