import { classes } from "../../lib/app/room.ts";
import type { Props } from "../../shell/design.ts";
import { Glyph } from "./glyphs.tsx";

/* DIALOG */

export function Dialog({ title, light, tag, bind, children }: Props["dialog"]) {
  return (
    <dialog className={classes("classic", "dialog", light ? "light" : "raised", tag)} aria-label={title} {...bind()}>
      {!light && (
        <form className="title" method="dialog">
          <span className="name">{title}</span>
          <span className="actions">
            <button type="submit" className="raised knob" aria-label="Close">
              <Glyph art="close" />
            </button>
          </span>
        </form>
      )}
      <div className="body">{children}</div>
    </dialog>
  );
}
