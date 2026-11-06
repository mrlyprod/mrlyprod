import { classes } from "@mrlyprod/ui/kit/room.ts";
import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { drag } from "../own/drag.ts";
import { Glyph } from "../own/glyphs.tsx";

export function Sheet({ title, tag, bind, children }: Props["sheet"]) {
  return (
    <dialog className={classes("dialog", "raised", "prop", tag)} aria-label={title} {...bind.root()}>
      <form className="title" method="dialog" onPointerDown={drag}>
        <span className="name">{title}</span>
        <span className="actions">
          <button type="submit" className="raised knob" aria-label="Close">
            <Glyph art="close" />
          </button>
        </span>
      </form>
      <div className="body">{children}</div>
    </dialog>
  );
}
